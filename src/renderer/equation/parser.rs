//! 한컴 수식 스크립트 재귀 하강 파서
//!
//! 토큰 리스트를 AST(EqNode)로 변환한다.

use super::ast::*;
use super::dispatch::{classify_command, matrix_style, pile_align, EqCommandClass};
use super::symbols::{
    self, is_big_operator, is_function, is_structure_command, lookup_function, lookup_symbol,
    FontStyleKind, DECORATIONS, FONT_STYLES,
};
use super::tokenizer::{tokenize, Token, TokenType};

#[path = "parser_commands.rs"]
mod commands;

#[path = "parser_fractions.rs"]
mod fractions;

#[path = "parser_structures.rs"]
mod structures;

#[path = "parser_root.rs"]
mod root;

#[path = "parser_negation.rs"]
mod negation;

#[path = "parser_alignment.rs"]
mod alignment;

#[path = "parser_script_operand.rs"]
mod script_operand;

#[path = "parser_terms.rs"]
mod terms;

#[path = "parser_left_scripts.rs"]
mod left_scripts;

/// 수식 중첩 깊이 상한.
///
/// 적대적으로 깊게 중첩된 입력(`{{{…}}}`, `sqrt sqrt …`, `LEFT ( LEFT ( …`,
/// 중첩 `matrix`/`cases`/`pile`/`eqalign`, 긴 `OVER`/`ATOP` 연쇄)은 재귀 하강
/// 파서를 무한 재귀로 몰거나, 반복으로 쌓인 깊은 `EqNode` 트리의 재귀적
/// `Drop`/layout/svg 에서 스택을 오버플로한다. 형제 파서 가드
/// (`MAX_HWPX_SECTION_DEPTH = 64`, `MAX_HWP5_SHAPE_DEPTH`, `MAX_DRAWING_OBJECT_DEPTH`)와
/// 같은 취지로 상한을 둔다. 실측 디버그 스택 오버플로 임계(중첩 sqrt ≈ 144)보다
/// 충분히 낮고, 실제 수식의 중첩 깊이(수 단계)는 넉넉히 웃돈다. 초과분은 조용히
/// 잘라내(truncate) 유효 수식 출력은 바뀌지 않는다.
pub(super) const MAX_EQ_DEPTH: u32 = 64;

/// 수식 파서
pub struct EqParser {
    tokens: Vec<Token>,
    pos: usize,
    /// 현재 재귀 깊이. 중첩 게이트웨이(`parse_command`/`parse_group`/
    /// `parse_paren_group`)에서 증감하여 무한 재귀·깊은 트리를 막는다.
    depth: u32,
    /// LParen 인덱스 → 매칭 RParen 인덱스 사전계산 결과. `paren_then_script` 의
    /// 매 호출당 O(n) 전방 스캔을 O(1) 조회로 바꿔, 괄호가 많은/깊은 입력에서의
    /// O(n^2) 행(hang)을 없앤다. LParen 이 아니거나 짝이 없으면 `tokens.len()`.
    paren_match: Vec<usize>,
    document_terms: bool,
    term_limit: usize,
    /// 독립 Print의 명령형 첨자 오류는 중첩 그룹/행을 포함한 수식 전체에 적용된다.
    invalid_document_script: bool,
    /// 잘못된 구조 인자 뒤 입력만 중단한다. 이미 읽은 앞 항은 유지한다.
    stopped_document_input: bool,
}

impl EqParser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        super::brace_recovery::retain_owned_closing_braces(&mut tokens);
        let paren_match = Self::compute_paren_match(&tokens);
        let term_limit = tokens.len();
        Self {
            tokens,
            pos: 0,
            depth: 0,
            paren_match,
            document_terms: false,
            term_limit,
            invalid_document_script: false,
            stopped_document_input: false,
        }
    }

    pub(crate) fn for_document(tokens: Vec<Token>) -> Self {
        Self {
            document_terms: true,
            ..Self::new(tokens)
        }
    }

    /// LParen→RParen 짝을 스택으로 한 번에(O(n)) 계산한다. `paren_then_script` 의
    /// 원래 선형 깊이 스캔과 동일하게 LParen/RParen 만 세고 그 외 토큰(중괄호 등)은
    /// 무시한다. 짝 없는 LParen 은 `tokens.len()` 으로 남는다.
    fn compute_paren_match(tokens: &[Token]) -> Vec<usize> {
        let n = tokens.len();
        let mut m = vec![n; n];
        let mut stack: Vec<usize> = Vec::new();
        for (i, t) in tokens.iter().enumerate() {
            match t.ty {
                TokenType::LParen => stack.push(i),
                TokenType::RParen => {
                    if let Some(open) = stack.pop() {
                        m[open] = i;
                    }
                }
                _ => {}
            }
        }
        m
    }

    fn current(&self) -> Option<&Token> {
        if self.stopped_document_input {
            return None;
        }
        self.tokens.get(self.pos).filter(|t| t.ty != TokenType::Eof)
    }

    fn current_type(&self) -> TokenType {
        if self.stopped_document_input {
            return TokenType::Eof;
        }
        self.tokens
            .get(self.pos)
            .map(|t| t.ty)
            .unwrap_or(TokenType::Eof)
    }

    fn current_value(&self) -> &str {
        self.tokens
            .get(self.pos)
            .map(|t| t.value.as_str())
            .unwrap_or("")
    }

    fn at_end(&self) -> bool {
        self.stopped_document_input
            || self.pos >= self.tokens.len()
            || self.current_type() == TokenType::Eof
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.at_end() {
            return None;
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        Some(tok)
    }

    fn expect(&mut self, ty: TokenType) -> bool {
        if self.current_type() == ty {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// 명령어 대소문자 무시 비교
    fn cmd_eq(val: &str, target: &str) -> bool {
        val.eq_ignore_ascii_case(target)
    }

    /// 최상위 레벨에서 OVER가 있는지 확인 (괄호/LEFT-RIGHT 내부 제외)
    fn has_toplevel_over(tokens: &[Token]) -> bool {
        let mut brace_depth = 0i32;
        let mut lr_depth = 0i32;
        for t in tokens {
            match t.ty {
                TokenType::LBrace => brace_depth += 1,
                TokenType::RBrace => brace_depth -= 1,
                TokenType::Command => {
                    if Self::cmd_eq(&t.value, "LEFT") {
                        lr_depth += 1;
                    } else if Self::cmd_eq(&t.value, "RIGHT") {
                        lr_depth -= 1;
                    } else if Self::cmd_eq(&t.value, "OVER") && brace_depth == 0 && lr_depth == 0 {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// 수식 전체 파싱 (엔트리 포인트)
    pub fn parse(&mut self) -> EqNode {
        let result = self.parse_expression();
        if self.document_terms && self.invalid_document_script {
            EqNode::Empty
        } else {
            result
        }
    }

    /// OVER/ATOP/CHOOSE 중위 연산자 처리. 현재 토큰이 OVER/ATOP 이면 children 의 마지막 요소를
    /// pop 하여 분수/atop 으로 결합한다. 처리했으면 true, 아니면 false.
    /// CASES/PILE/EQALIGN 등 row-collecting 파서가 분수를 인식하지 못하는 결함(#505)을
    /// 방지하기 위해 모든 token-collecting 루프에서 호출한다.
    ///
    /// `over_run` 은 현재 루프의 "연속 OVER/ATOP 결합 횟수"다. `1 over 1 over …`
    /// 같은 긴 연쇄는 반복으로 좌편향 깊은 트리를 쌓아 [`MAX_EQ_DEPTH`] 재귀 가드를
    /// 우회하므로(그 뒤 재귀적 Drop/layout/svg 에서 오버플로), 연쇄 길이를
    /// [`MAX_EQ_DEPTH`] 로 제한한다. 상한 초과 시 결합을 멈춰 false 를 돌려주면 남은
    /// OVER 는 parse_element 가 Empty 로 소비하므로 진행이 보장된다.
    fn try_consume_infix_over_atop(
        &mut self,
        children: &mut Vec<EqNode>,
        over_run: &mut u32,
    ) -> bool {
        if self.current_type() != TokenType::Command {
            *over_run = 0; // 다음은 일반 요소 — 연쇄 종료, 카운터 리셋.
            return false;
        }
        let val = self.current_value();
        let is_over = Self::cmd_eq(val, "OVER");
        let is_atop = Self::cmd_eq(val, "ATOP");
        let is_choose = Self::cmd_eq(val, "CHOOSE");
        if !is_over && !is_atop && !is_choose {
            *over_run = 0; // OVER/ATOP 이 아닌 명령 — 연쇄 종료, 카운터 리셋.
            return false;
        }
        if *over_run >= MAX_EQ_DEPTH {
            // 상한 도달: 결합 중단(리셋하지 않음). 남은 OVER 는 parse_element 가
            // Empty 로 소비하고, 이어지는 일반 요소가 카운터를 리셋한다.
            return false;
        }
        *over_run += 1;
        self.pos += 1;
        let top = children.pop().unwrap_or(EqNode::Empty);
        let bottom = self.parse_element();
        children.push(if is_choose {
            EqNode::Binomial {
                top: Box::new(top),
                bottom: Box::new(bottom),
            }
        } else if is_atop {
            EqNode::Atop {
                top: Box::new(top),
                bottom: Box::new(bottom),
            }
        } else {
            EqNode::Fraction {
                numer: Box::new(top),
                denom: Box::new(bottom),
            }
        });
        true
    }

    /// 표현식 파싱 (중단 조건 없이 끝까지)
    /// OVER/ATOP을 중위 연산자로 처리: 바로 앞/뒤 요소를 위아래로 배치
    fn parse_expression(&mut self) -> EqNode {
        let mut children = Vec::new();
        let mut over_run = 0u32;
        while !self.at_end() {
            // 그룹 종료 또는 RIGHT 만나면 중단
            if self.current_type() == TokenType::RBrace {
                break;
            }
            if self.current_type() == TokenType::Command
                && Self::cmd_eq(self.current_value(), "RIGHT")
            {
                break;
            }
            // OVER/ATOP 중위 연산자: 직전/직후 요소를 위아래로 결합
            if self.try_consume_infix_over_atop(&mut children, &mut over_run) {
                continue;
            }
            children.push(self.parse_element());
        }
        EqNode::Row(children).simplify()
    }

    /// 단일 요소 파싱
    fn parse_element_atom(&mut self) -> EqNode {
        if self.at_end() {
            return EqNode::Empty;
        }

        let ty = self.current_type();
        let val = self.current_value().to_string();

        match ty {
            TokenType::Command => {
                self.pos += 1;
                self.parse_command(&val)
            }
            TokenType::Number => {
                self.pos += 1;
                self.try_parse_scripts(EqNode::Number(val))
            }
            TokenType::Symbol => {
                self.pos += 1;
                // 한컴의 붙은 ASCII 화살표 축약을 의미 기호로 보존한다.
                if matches!(val.as_str(), "->" | "<-") {
                    EqNode::MathSymbol(if val == "->" { "→" } else { "←" }.to_string())
                } else if !val.is_ascii() && super::symbols::is_binary_symbol(&val) {
                    // 새 기호 토큰도 기존 Text/명령 경로의 첨자 결합을 유지한다.
                    self.try_parse_scripts(EqNode::MathSymbol(val))
                } else if !val.is_ascii() && super::symbols::is_relation_symbol(&val) {
                    EqNode::MathSymbol(val)
                } else {
                    EqNode::Symbol(val)
                }
            }
            TokenType::Text => {
                self.pos += 1;
                let node = if super::symbols::is_relation_symbol(&val) {
                    EqNode::MathSymbol(val)
                } else {
                    EqNode::Text(val)
                };
                self.try_parse_scripts(node)
            }
            TokenType::Quoted => {
                self.pos += 1;
                self.try_parse_scripts(EqNode::Quoted(val))
            }
            TokenType::Whitespace => {
                self.pos += 1;
                match val.as_str() {
                    "~" => EqNode::Space(SpaceKind::Normal),
                    "`" => EqNode::Space(SpaceKind::Thin),
                    "#" => EqNode::Newline,
                    "&" => EqNode::Space(SpaceKind::Tab),
                    _ => EqNode::Empty,
                }
            }
            TokenType::LBrace => {
                let group = self.parse_group();
                self.try_parse_scripts(group)
            }
            TokenType::LParen => {
                // #1305: `(...)` 뒤에 첨자가 오면 Paren 그룹으로 묶어 첨자를 결합한다
                // (그렇지 않으면 `)` 뒤 `^2` 가 base 없는 orphan Superscript 가 됨).
                // 첨자가 없으면 기존대로 느슨한 Symbol 흐름 유지 → 일반 괄호 렌더 무변경.
                if self.paren_then_script() {
                    let group = self.parse_paren_group();
                    self.try_parse_scripts(group)
                } else {
                    self.pos += 1;
                    EqNode::Symbol(val)
                }
            }
            TokenType::RParen | TokenType::LBracket | TokenType::RBracket => {
                self.pos += 1;
                EqNode::Symbol(val)
            }
            TokenType::Subscript | TokenType::Superscript => {
                // 베이스 없는 첨자 (예: _{y} x)
                self.try_parse_scripts(EqNode::Empty)
            }
            _ => {
                self.pos += 1;
                EqNode::Empty
            }
        }
    }

    fn finish_command_atom(&mut self, node: EqNode, attach_scripts: bool) -> EqNode {
        if attach_scripts {
            self.try_parse_scripts(node)
        } else {
            node
        }
    }

    /// 중괄호 그룹 파싱: {...} (깊이 가드).
    /// 그룹 내의 OVER는 parse_expression의 중위 연산자 처리로 자동 처리된다.
    fn parse_group(&mut self) -> EqNode {
        if self.current_type() != TokenType::LBrace {
            // 여는 중괄호가 아니면 그룹이 아님 — 위임 (깊이 변화 없음).
            return self.parse_element();
        }
        self.depth += 1;
        if self.depth > MAX_EQ_DEPTH {
            // 상한 초과: 그룹 전체를 건너뛰어(truncate) 진행을 보장한다.
            self.depth -= 1;
            self.skip_braced_group();
            return EqNode::Empty;
        }
        let node = self.parse_group_inner();
        self.depth -= 1;
        node
    }

    fn parse_group_inner(&mut self) -> EqNode {
        self.expect(TokenType::LBrace); // 여는 '{' 소비 (호출부에서 존재 보장)

        let mut children = Vec::new();
        let mut over_run = 0u32;
        while !self.at_end() {
            if self.current_type() == TokenType::RBrace {
                break;
            }
            // OVER/ATOP 중위 연산자: 그룹 내에서도 동일하게 처리
            if self.try_consume_infix_over_atop(&mut children, &mut over_run) {
                continue;
            }
            children.push(self.parse_element());
        }

        // 닫는 괄호 건너뛰기
        self.expect(TokenType::RBrace);

        EqNode::Row(children).simplify()
    }

    /// 현재 `{` 부터 매칭되는 `}` 까지 통째로 소비한다(깊이 상한 초과 시 truncate).
    fn skip_braced_group(&mut self) {
        // 전제: 현재 토큰이 LBrace.
        self.pos += 1; // 여는 '{' 소비
        let close = self.find_matching_brace(self.pos);
        self.pos = close.min(self.tokens.len());
        self.expect(TokenType::RBrace); // 매칭 '}' 가 있으면 소비
    }

    /// 현재 `(` 부터 매칭되는 `)` 까지 통째로 소비한다(깊이 상한 초과 시 truncate).
    fn skip_paren_group(&mut self) {
        // 전제: 현재 토큰이 LParen.
        let mut depth = 0i32;
        while self.pos < self.tokens.len() {
            match self.tokens[self.pos].ty {
                TokenType::LParen => depth += 1,
                TokenType::RParen => {
                    depth -= 1;
                    self.pos += 1;
                    if depth <= 0 {
                        return;
                    }
                    continue;
                }
                TokenType::Eof => return,
                _ => {}
            }
            self.pos += 1;
        }
    }

    /// 매칭되는 닫는 괄호 위치 찾기
    fn find_matching_brace(&self, start: usize) -> usize {
        let mut depth = 1i32;
        let mut pos = start;
        while pos < self.tokens.len() {
            match self.tokens[pos].ty {
                TokenType::LBrace => depth += 1,
                TokenType::RBrace => {
                    depth -= 1;
                    if depth == 0 {
                        return pos;
                    }
                }
                _ => {}
            }
            pos += 1;
        }
        self.tokens.len()
    }

    /// 현재 LParen 의 매칭 RParen 다음 첨자(`^`/`_`)인지 (#1305).
    /// RParen 뒤의 Thin 공백(`) 하나는 첨자 결합 전에 보존되는 공백이므로 건너뛴다.
    /// 참이면 `(...)` 를 Paren 그룹으로 묶어 첨자를 결합해야 한다.
    /// 현재 토큰이 LParen 이라는 전제. 사전계산된 `paren_match` 로 O(1) 조회한다
    /// (원래는 매 호출 O(n) 전방 스캔 → 괄호 많은 입력에서 O(n^2) 행 유발).
    fn paren_then_script(&self) -> bool {
        let close = self
            .paren_match
            .get(self.pos)
            .copied()
            .unwrap_or(self.tokens.len());
        let next = close + 1;
        let script = if self
            .tokens
            .get(next)
            .is_some_and(|token| token.ty == TokenType::Whitespace && token.value == "`")
        {
            next + 1
        } else {
            next
        };
        matches!(
            self.tokens.get(script).map(|t| t.ty),
            Some(TokenType::Subscript) | Some(TokenType::Superscript)
        ) || (self.document_terms
            && self.tokens.get(script).is_some_and(|token| {
                token.ty == TokenType::Command
                    && ["LSUB", "LSUP", "SUB", "SUP"]
                        .iter()
                        .any(|name| Self::cmd_eq(&token.value, name))
            }))
    }

    /// `(...)` 를 자동크기 괄호 그룹으로 파싱 (#1305). 현재 LParen 전제 (깊이 가드).
    fn parse_paren_group(&mut self) -> EqNode {
        self.depth += 1;
        if self.depth > MAX_EQ_DEPTH {
            // 상한 초과: 괄호 그룹 전체를 건너뛰어(truncate) 진행을 보장한다.
            self.depth -= 1;
            self.skip_paren_group();
            return EqNode::Empty;
        }
        let node = self.parse_paren_group_inner();
        self.depth -= 1;
        node
    }

    fn parse_paren_group_inner(&mut self) -> EqNode {
        // 안쪽 소괄호의 닫힘을 바깥 묶음의 끝으로 오인하지 않는다.
        let close = self.paren_match[self.pos];
        let previous_limit = self.term_limit;
        self.term_limit = self.term_limit.min(close);
        self.pos += 1; // '(' 소비
        let mut items = Vec::new();
        let mut over_run = 0u32;
        while !self.at_end() && self.pos < close {
            if self.try_consume_infix_over_atop(&mut items, &mut over_run) {
                continue;
            }
            items.push(self.parse_element());
        }
        self.term_limit = previous_limit;
        self.expect(TokenType::RParen); // ')' 소비
        EqNode::Paren {
            left: "(".to_string(),
            right: ")".to_string(),
            body: Box::new(EqNode::Row(items).simplify()),
        }
    }

    /// 단일 토큰 또는 그룹 파싱 (첨자/인자용)
    fn parse_single_or_group(&mut self) -> EqNode {
        if self.at_end() || self.current_type() == TokenType::RBrace || self.pos >= self.term_limit
        {
            return EqNode::Empty;
        }
        if self.document_terms {
            self.parse_element()
        } else {
            self.parse_single_atom_or_group()
        }
    }

    fn parse_single_atom_or_group(&mut self) -> EqNode {
        if self.at_end() {
            return EqNode::Empty;
        }

        // RBrace는 그룹 종료 마커 — 소비하지 않고 빈 노드 반환
        if self.current_type() == TokenType::RBrace {
            return EqNode::Empty;
        }

        if self.current_type() == TokenType::LBrace {
            return self.parse_group();
        }

        // 단일 토큰
        let ty = self.current_type();
        let val = self.current_value().to_string();
        self.pos += 1;

        match ty {
            TokenType::Command => {
                if let Some(symbol) = lookup_symbol(&val) {
                    EqNode::MathSymbol(symbol.to_string())
                } else if is_function(&val) {
                    EqNode::Function(lookup_function(&val).unwrap_or(&val).to_string())
                } else {
                    // [#1204-B] decoration/구조 명령(bar, sqrt 등)도 단일 인자/body 로
                    // 올 수 있다 (`rm bar {...}`). parse_command 로 위임 — 미지 명령은
                    // parse_command 의 fall-through 가 Text 로 처리하므로 안전.
                    self.parse_command(&val)
                }
            }
            TokenType::Number => EqNode::Number(val),
            TokenType::Text => EqNode::Text(val),
            TokenType::Quoted => EqNode::Quoted(val),
            TokenType::Symbol => EqNode::Symbol(val),
            _ => EqNode::Text(val),
        }
    }

    /// 첨자(subscript/superscript) 파싱 시도
    /// 한컴 수식에서 함수/기호 뒤에 Thin 공백(`)이 오고 첨자가 따라오는 패턴이 일반적이므로,
    /// Thin 공백 뒤에 첨자가 있으면 공백을 건너뛰고 첨자를 파싱한다.
    fn try_parse_scripts(&mut self, base: EqNode) -> EqNode {
        let mut result = base;
        let mut has_sub = false;
        let mut has_sup = false;
        let mut sub = None;
        let mut sup = None;

        loop {
            if self.at_end() || (self.document_terms && self.pos >= self.term_limit) {
                break;
            }
            // Thin 공백(`) 뒤에 첨자가 오면 첨자 결합은 유지하되(과거 exam_math
            // `log`_{2}` 첨자 파싱 실패 수정), 공백의 시각 폭은 base 뒤에 보존한다.
            // 한컴은 `a_{2}` 와 `` a`_{2} `` 를 다르게 렌더한다 — 공백 토큰을
            // 삭제만 하면 첨자가 붙는 위치(가로)와 AST 정보가 함께 사라진다(#5534).
            // Subscript/Superscript 레이아웃은 base box 오른쪽 끝에 첨자를 두므로
            // base 를 Row[base, Space(Thin)] 로 감싸면 첨자가 1/4 공백만큼
            // 오른쪽에 놓인다.
            if self.current_type() == TokenType::Whitespace && self.current_value() == "`" {
                let next_pos = self.pos + 1;
                if next_pos < self.tokens.len() {
                    let next_ty = self.tokens[next_pos].ty;
                    if next_ty == TokenType::Subscript || next_ty == TokenType::Superscript {
                        self.pos += 1; // Thin 공백 소비 (폭은 아래에서 보존)
                        result = EqNode::Row(vec![result, EqNode::Space(SpaceKind::Thin)]);
                    }
                }
            }
            let ty = self.current_type();
            if ty == TokenType::Subscript && !has_sub {
                self.pos += 1;
                sub = Some(self.parse_script_operand(has_sup));
                has_sub = true;
            } else if ty == TokenType::Superscript && !has_sup {
                self.pos += 1;
                sup = Some(self.parse_superscript_operand(has_sub));
                has_sup = true;
            } else {
                break;
            }
        }

        if has_sub && has_sup {
            EqNode::SubSup {
                base: Box::new(result),
                sub: Box::new(sub.unwrap_or(EqNode::Empty)),
                sup: Box::new(sup.unwrap_or(EqNode::Empty)),
            }
        } else if has_sub {
            EqNode::Subscript {
                base: Box::new(result),
                sub: Box::new(sub.unwrap_or(EqNode::Empty)),
            }
        } else if has_sup {
            EqNode::Superscript {
                base: Box::new(result),
                sup: Box::new(sup.unwrap_or(EqNode::Empty)),
            }
        } else {
            result
        }
    }

    /// 큰 연산자 파싱 (적분, 합 등) — 첨자 포함
    fn parse_big_op(&mut self, symbol: String) -> EqNode {
        let (sub, sup) = self.parse_operator_scripts();
        EqNode::BigOp { symbol, sub, sup }
    }

    /// 극한 파싱
    fn parse_limit(&mut self, is_upper: bool) -> EqNode {
        let mut sub = None;

        if self.current_type() == TokenType::Subscript {
            self.pos += 1;
            sub = Some(Box::new(self.parse_script_operand(false)));
        }

        EqNode::Limit { is_upper, sub }
    }

    /// LEFT-RIGHT 괄호 파싱
    /// 내부의 OVER는 parse_expression의 중위 연산자 처리로 자동 처리된다.
    fn parse_left_right(&mut self) -> EqNode {
        // LEFT 다음 괄호 문자 읽기
        let left = self.read_bracket_char();

        // RIGHT까지의 내용을 parse_expression으로 파싱
        // parse_expression은 RIGHT를 만나면 자동 중단하고, OVER도 중위 연산자로 처리
        let body = self.parse_expression();

        // RIGHT 건너뛰기
        if self.current_type() == TokenType::Command && Self::cmd_eq(self.current_value(), "RIGHT")
        {
            self.pos += 1;
        }

        // RIGHT 다음 괄호 문자 읽기
        let right = self.read_bracket_char();

        EqNode::Paren {
            left,
            right,
            body: Box::new(body),
        }
    }

    /// 괄호 문자 읽기 (LEFT/RIGHT 뒤)
    fn read_bracket_char(&mut self) -> String {
        if self.at_end() {
            return String::new();
        }

        let ty = self.current_type();
        let val = self.current_value().to_string();

        match ty {
            TokenType::LParen
            | TokenType::RParen
            | TokenType::LBracket
            | TokenType::RBracket
            | TokenType::LBrace
            | TokenType::RBrace => {
                self.pos += 1;
                val
            }
            TokenType::Symbol if val == "|" || val == "." => {
                self.pos += 1;
                if val == "." {
                    String::new() // . = 괄호 생략
                } else {
                    val
                }
            }
            TokenType::Command => {
                // LBRACE, RBRACE 등 명령어
                if let Some(sym) = lookup_symbol(&val) {
                    self.pos += 1;
                    sym.to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    /// RIGHT 위치 찾기 (LEFT-RIGHT 쌍 고려)
    fn find_right_pos(&self) -> usize {
        let mut depth = 1i32;
        let mut pos = self.pos;
        while pos < self.tokens.len() {
            let t = &self.tokens[pos];
            if t.ty == TokenType::Command {
                if Self::cmd_eq(&t.value, "LEFT") {
                    depth += 1;
                } else if Self::cmd_eq(&t.value, "RIGHT") {
                    depth -= 1;
                    if depth == 0 {
                        return pos;
                    }
                }
            }
            pos += 1;
        }
        self.tokens.len()
    }

    /// 범위 내 분수 파싱
    /// OVER 앞/뒤에 중괄호 그룹이 있으면 해당 그룹만 분자/분모로 사용하고
    /// 나머지는 분수 바깥 요소로 처리한다.
    fn parse_fraction_in_range(&mut self, end: usize) -> EqNode {
        // OVER 앞의 모든 요소를 파싱
        let mut before_nodes = Vec::new();
        while self.pos < end && !self.at_end() {
            if self.current_type() == TokenType::Command
                && Self::cmd_eq(self.current_value(), "OVER")
            {
                self.pos += 1;
                break;
            }
            before_nodes.push(self.parse_element());
        }

        // 분자: OVER 바로 앞의 마지막 요소 (또는 그룹)
        // 나머지 앞 요소들은 분수 앞에 배치
        let (pre_nodes, numer) = if before_nodes.len() > 1 {
            let numer = before_nodes.pop().unwrap();
            (before_nodes, numer)
        } else {
            (Vec::new(), EqNode::Row(before_nodes).simplify())
        };

        // 분모: OVER 바로 뒤의 첫 번째 요소 (또는 그룹)
        let denom = if self.pos < end && !self.at_end() {
            self.parse_element()
        } else {
            EqNode::Empty
        };

        // 분수 뒤 나머지 요소
        let mut after_nodes = Vec::new();
        while self.pos < end && !self.at_end() {
            if self.current_type() == TokenType::Command
                && Self::cmd_eq(self.current_value(), "RIGHT")
            {
                break;
            }
            after_nodes.push(self.parse_element());
        }

        let fraction = EqNode::Fraction {
            numer: Box::new(numer),
            denom: Box::new(denom),
        };

        // 앞/뒤 요소와 분수를 Row로 조립
        if pre_nodes.is_empty() && after_nodes.is_empty() {
            fraction
        } else {
            let mut all = pre_nodes;
            all.push(fraction);
            all.extend(after_nodes);
            EqNode::Row(all).simplify()
        }
    }

    /// COLOR의 색 인자와 본체를 각각 소비한다. 비 RGB 색 인자는 한컴처럼 무시한다.
    fn parse_color(&mut self) -> EqNode {
        if !self.expect(TokenType::LBrace) {
            // 독립 Print는 이름/미지원 색 인자를 표시하지 않고 본체를 검정으로 그린다.
            let _color_argument = self.parse_element();
            return self.parse_element();
        }

        // R, G, B 값 읽기
        let mut rgb = [0u8; 3];
        for i in 0..3 {
            if self.current_type() == TokenType::Number {
                rgb[i] = self.current_value().parse().unwrap_or(0);
                self.pos += 1;
            }
            // 콤마 건너뛰기
            if self.current_type() == TokenType::Symbol && self.current_value() == "," {
                self.pos += 1;
            }
        }
        self.expect(TokenType::RBrace);

        let body = self.parse_single_or_group();

        EqNode::Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
            body: Box::new(body),
        }
    }
}

/// 수식 스크립트를 AST로 파싱
pub fn parse(script: &str) -> EqNode {
    let tokens = tokenize(script);
    let mut parser = EqParser::new(tokens);
    parser.parse()
}

#[cfg(test)]
include!("parser_syntax_tests.inc.rs");

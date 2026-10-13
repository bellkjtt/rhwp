//! 기존 입력 소유와 공통 파서 상태를 보존한다.

use super::*;

impl EqParser {
    /// 단일 요소 파싱
    pub(super) fn parse_element_atom(&mut self) -> EqNode {
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
                } else if !val.is_ascii() && super::super::symbols::is_binary_symbol(&val) {
                    // 새 기호 토큰도 기존 Text/명령 경로의 첨자 결합을 유지한다.
                    self.try_parse_scripts(EqNode::MathSymbol(val))
                } else if !val.is_ascii() && super::super::symbols::is_relation_symbol(&val) {
                    EqNode::MathSymbol(val)
                } else {
                    EqNode::Symbol(val)
                }
            }
            TokenType::Text => {
                self.pos += 1;
                let node = if super::super::symbols::is_relation_symbol(&val) {
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

    /// 단일 토큰 또는 그룹 파싱 (첨자/인자용)
    pub(super) fn parse_single_or_group(&mut self) -> EqNode {
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

    pub(super) fn parse_single_atom_or_group(&mut self) -> EqNode {
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
}

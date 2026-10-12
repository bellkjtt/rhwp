//! 한컴 항의 경계를 구조 파서의 모든 요소 수집 경로에 공급한다.

use super::{EqNode, EqParser, TokenType};

impl EqParser {
    pub(super) fn parse_element(&mut self) -> EqNode {
        if self.document_terms && self.pos >= self.term_limit {
            return EqNode::Empty;
        }
        let first_type = self.current_type();
        let first = self.parse_element_atom();
        if !self.document_terms
            || matches!(
                first_type,
                TokenType::LBrace | TokenType::Whitespace | TokenType::Eof
            )
            || is_font_switch(&first)
        {
            return first;
        }
        let mut items = vec![first];
        while self.has_term_continuation() {
            let start = self.pos;
            items.push(self.parse_element_atom());
            if self.pos == start {
                break;
            }
        }
        EqNode::Row(items).simplify()
    }

    fn has_term_continuation(&self) -> bool {
        if self.pos >= self.term_limit || self.at_end() {
            return false;
        }
        let token = &self.tokens[self.pos];
        if token.space_before
            || matches!(
                token.ty,
                TokenType::LBrace | TokenType::RBrace | TokenType::Whitespace
            )
        {
            return false;
        }
        // 구조 연산자·닫힘과 글꼴 전환은 피연산자 안으로 소비하지 않는다.
        token.ty != TokenType::Command
            || !["OVER", "ATOP", "CHOOSE", "RIGHT", "END", "OF", "RM", "IT"]
                .iter()
                .any(|name| token.value.eq_ignore_ascii_case(name))
    }
}

fn is_font_switch(node: &EqNode) -> bool {
    matches!(node, EqNode::FontStyle { body, .. } if matches!(body.as_ref(), EqNode::Empty))
}

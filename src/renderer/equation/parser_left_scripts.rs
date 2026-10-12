//! 공식 중위 LSUB/LSUP 문법을 모든 항 수집 경로에서 같은 방식으로 결합한다.

use super::{EqNode, EqParser, TokenType, MAX_EQ_DEPTH};

impl EqParser {
    pub(super) fn try_consume_left_script(
        &mut self,
        children: &mut Vec<EqNode>,
        infix_run: &mut u32,
    ) -> bool {
        if !self.document_terms || self.current_type() != TokenType::Command {
            return false;
        }
        let subscript = Self::cmd_eq(self.current_value(), "LSUB");
        if (!subscript && !Self::cmd_eq(self.current_value(), "LSUP"))
            || children.is_empty()
            || *infix_run >= MAX_EQ_DEPTH
        {
            return false;
        }
        *infix_run += 1;
        self.pos += 1;
        let script = Box::new(self.parse_element());
        let previous = children.pop().expect("앞 항이 있는 중위 첨자");
        let (base, mut sub, mut sup) = match previous {
            EqNode::LeftScripts { base, sub, sup }
                if (subscript && sub.is_none()) || (!subscript && sup.is_none()) =>
            {
                (base, sub, sup)
            }
            base => (Box::new(base), None, None),
        };
        if subscript {
            sub = Some(script);
        } else {
            sup = Some(script);
        }
        children.push(EqNode::LeftScripts { base, sub, sup });
        true
    }
}

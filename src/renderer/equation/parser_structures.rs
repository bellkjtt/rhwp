//! 기존 파서 상태를 공유하며 입력 소유 범위 안에서 처리한다.

use super::*;

impl EqParser {
    /// 독립 Print는 필수 인자 누락 뒤 입력을 중단한다. 빈 MATRIX도 중단하지만
    /// 빈 PILE/EQALIGN/CASES와 행·셀 구분자만 있는 몸체는 유효하다.
    pub(super) fn begin_structure_body(&mut self, empty_matrix_is_error: bool) -> Option<usize> {
        if !self.expect(TokenType::LBrace) {
            self.stopped_document_input |= self.document_terms;
            return None;
        }
        let end = self.find_matching_brace(self.pos);
        let empty = self.tokens[self.pos..end].iter().all(|token| {
            token.ty == TokenType::Whitespace && token.value.chars().all(char::is_whitespace)
        });
        if self.document_terms && empty_matrix_is_error && empty {
            self.stopped_document_input = true;
            return None;
        }
        Some(end)
    }

    /// 행렬 파싱: MATRIX{a & b # c & d}
    pub(super) fn parse_matrix(&mut self, style: MatrixStyle) -> EqNode {
        let Some(end) = self.begin_structure_body(true) else {
            return EqNode::Matrix {
                rows: vec![vec![]],
                style,
            };
        };
        let mut rows: Vec<Vec<EqNode>> = vec![vec![]];
        let mut current_cell = Vec::new();
        let mut over_run = 0u32;

        while self.pos < end && !self.at_end() {
            if self.current_type() == TokenType::RBrace {
                break;
            }
            if self.current_type() == TokenType::Whitespace && self.current_value() == "#" {
                // 새 행
                if let Some(last_row) = rows.last_mut() {
                    last_row.push(EqNode::Row(current_cell).simplify());
                }
                current_cell = Vec::new();
                rows.push(vec![]);
                self.pos += 1;
            } else if self.current_type() == TokenType::Whitespace && self.current_value() == "&" {
                // 새 셀
                if let Some(last_row) = rows.last_mut() {
                    last_row.push(EqNode::Row(current_cell).simplify());
                }
                current_cell = Vec::new();
                self.pos += 1;
            } else if self.try_consume_infix_over_atop(&mut current_cell, &mut over_run) {
                // OVER/ATOP 중위 처리 (#505)
                continue;
            } else {
                current_cell.push(self.parse_element());
            }
        }

        // 마지막 셀 추가
        if !current_cell.is_empty() || rows.last().map_or(false, |r| !r.is_empty()) {
            if let Some(last_row) = rows.last_mut() {
                last_row.push(EqNode::Row(current_cell).simplify());
            }
        }

        self.expect(TokenType::RBrace);

        EqNode::Matrix { rows, style }
    }

    /// 세로 쌓기 파싱
    pub(super) fn parse_pile(&mut self, align: PileAlign) -> EqNode {
        let Some(end) = self.begin_structure_body(false) else {
            return EqNode::Empty;
        };
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut over_run = 0u32;

        while self.pos < end && !self.at_end() {
            if self.current_type() == TokenType::RBrace {
                break;
            }
            if self.current_type() == TokenType::Whitespace && self.current_value() == "#" {
                rows.push(EqNode::Row(current_row).simplify());
                current_row = Vec::new();
                self.pos += 1;
            } else if self.try_consume_infix_over_atop(&mut current_row, &mut over_run) {
                // OVER/ATOP 중위 처리 (#505)
                continue;
            } else {
                current_row.push(self.parse_element());
            }
        }

        if !current_row.is_empty() || rows.is_empty() {
            rows.push(EqNode::Row(current_row).simplify());
        }

        self.expect(TokenType::RBrace);

        EqNode::Pile { rows, align }
    }
}

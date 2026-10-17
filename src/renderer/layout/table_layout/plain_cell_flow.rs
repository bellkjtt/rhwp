//! 셀의 단 설정 메타데이터와 실제 줄·개체 점유를 구분한다.

use crate::model::{control::Control, paragraph::Paragraph};

impl super::LayoutEngine {
    /// 일반 칸이 없는 행의 종료 병합 칸이 실제 수용 공간에서 모두 끝났는지 확인한다.
    /// 계속되는 다른 병합 칸의 높이와 내용 커서는 별도 원장에 남긴다.
    pub(crate) fn row_band_completes_ending_reflow_spans(
        &self,
        table: &crate::model::table::Table,
        row: usize,
        accepted: &[crate::renderer::pagination::StraddleCellCarry],
        styles: &crate::renderer::style_resolver::ResolvedStyleSet,
    ) -> bool {
        if table.page_break != crate::model::table::TablePageBreak::RowBreak
            || !Self::row_cut_cell_order(table, row).is_empty()
        {
            return false;
        }
        let reflowed = self
            .render_normalization
            .borrow()
            .table_text_reflowed(table);
        let mut has_ending_owner = false;
        for (index, cell) in table.cells.iter().enumerate().filter(|(_, cell)| {
            cell.row_span > 1 && usize::from(cell.row) + usize::from(cell.row_span) == row + 1
        }) {
            has_ending_owner = true;
            if !cell.paragraphs.iter().all(|paragraph| {
                paragraph_has_plain_flow(paragraph)
                    && (reflowed || crate::renderer::para_has_no_stored_line_segs(paragraph))
            }) {
                return false;
            }
            let Some(carry) = accepted.iter().find(|carry| carry.cell_index == index) else {
                return false;
            };
            let units = self.cell_units(cell, table, styles);
            if !carry
                .fragment_cut
                .is_some_and(|(_, end)| end == units.len())
            {
                return false;
            }
            let Some(physical) = carry.accepted_physical_height else {
                return false;
            };
            // 끝 행 소속만으로 미소비 줄이 모두 들어갔다고 추정하지 않는다.
            let content = self.cell_cut_visible_height(cell, table, styles, 0, units.len());
            let roundoff = f64::EPSILON * content.abs().max(physical.abs()).max(1.0) * 8.0;
            if content > physical + roundoff {
                return false;
            }
        }
        has_ending_owner
    }

    /// 재조판 행의 컷 뒤에도 같은 병합 칸이 이어지면 쪽 끝의 빈 띠도 그 칸이 소유한다.
    /// 내용에서 결정된 행 높이와 별개인 선언 최소 높이의 존재를 요구하지 않는다.
    pub(crate) fn row_has_continuing_reflow_span(
        &self,
        table: &crate::model::table::Table,
        row: usize,
    ) -> bool {
        self.row_uses_reflow_physical_frame(table, row)
            && table.cells.iter().any(|cell| {
                usize::from(cell.row) <= row
                    && row + 1 < usize::from(cell.row) + usize::from(cell.row_span)
            })
    }
}

impl super::CellUnit {
    /// 빈 간격 유닛을 제외하고 실제 가시 줄 범위의 존재만 공유한다.
    pub(in crate::renderer::layout) fn has_visible_line(&self) -> bool {
        !self.empty_spacer && self.vis_start < self.vis_end
    }
}

/// 한 단 설정은 이미 선택한 셀 프레임을 바꾸거나 내용 상자를 추가하지 않는다.
/// 다단 설정이나 독립 개체의 흐름 계약까지 이 판정으로 확대하지 않는다.
pub(super) fn paragraph_has_plain_flow(paragraph: &Paragraph) -> bool {
    paragraph
        .controls
        .iter()
        .all(|control| matches!(control, Control::ColumnDef(columns) if columns.column_count == 1))
}

//! 셀의 단 설정 메타데이터와 실제 줄·개체 점유를 구분한다.

use crate::model::{control::Control, paragraph::Paragraph};

impl super::LayoutEngine {
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

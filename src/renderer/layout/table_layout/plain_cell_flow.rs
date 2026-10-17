//! 셀의 단 설정 메타데이터와 실제 줄·개체 점유를 구분한다.

use crate::model::{control::Control, paragraph::Paragraph};

/// 한 단 설정은 이미 선택한 셀 프레임을 바꾸거나 내용 상자를 추가하지 않는다.
/// 다단 설정이나 독립 개체의 흐름 계약까지 이 판정으로 확대하지 않는다.
pub(super) fn paragraph_has_plain_flow(paragraph: &Paragraph) -> bool {
    paragraph
        .controls
        .iter()
        .all(|control| matches!(control, Control::ColumnDef(columns) if columns.column_count == 1))
}

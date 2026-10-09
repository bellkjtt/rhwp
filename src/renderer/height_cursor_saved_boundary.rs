//! 저장된 두 문단의 줄 경계가 연속인지 확인한다.
//!
//! 새로 만든 줄 좌표와 편집으로 무효가 된 저장 분할은 독립적인
//! 배치 근거가 아니다. 유효한 저장 경계만 실제 소비한 흐름에 연결한다.

use crate::model::paragraph::{LineSeg, Paragraph};

pub(super) fn continuous(previous: &Paragraph, current: &Paragraph) -> bool {
    if previous.stored_text_partition_is_dirty() || current.stored_text_partition_is_dirty() {
        return false;
    }
    let Some((last, first)) = previous.line_segs.last().zip(current.line_segs.first()) else {
        return false;
    };
    if last.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
        || first.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
    {
        return false;
    }
    first.vertical_pos > last.vertical_pos
        && first.vertical_pos
            == last
                .vertical_pos
                .saturating_add(last.line_height)
                .saturating_add(last.line_spacing)
}

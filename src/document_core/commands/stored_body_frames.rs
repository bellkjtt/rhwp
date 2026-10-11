//! 저장 줄에 포함된 개체 높이와 쪽 원점을 재구성 사다리에 연결한다.

use crate::model::control::Control;
use crate::model::paragraph::{LineSeg, Paragraph};

/// 한 개의 고정 크기 도형/그림이 저장 줄 전체를 점유한 경우다.
/// 표·수식·캡션은 재조판으로 높이가 달라질 수 있으므로 이 계약에 넣지 않는다.
pub(super) fn owns_saved_object_line(para: &Paragraph) -> bool {
    let [line] = para.line_segs.as_slice() else {
        return false;
    };
    let [control] = para.controls.as_slice() else {
        return false;
    };
    if para.stored_text_partition_dirty
        || line.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
        || para
            .text
            .chars()
            .any(|c| !c.is_whitespace() && c > '\u{001f}' && c != '\u{fffc}')
    {
        return false;
    }
    let common = match control {
        Control::Shape(shape)
            if shape
                .drawing()
                .is_some_and(|drawing| drawing.caption.is_none()) =>
        {
            shape.common()
        }
        Control::Picture(picture) if picture.caption.is_none() => &picture.common,
        _ => return false,
    };
    common.treat_as_char
        && common.height > 0
        && i64::from(line.line_height)
            == i64::from(common.height)
                + i64::from(common.margin.top)
                + i64::from(common.margin.bottom)
}

/// 이전 저장 줄 상자가 본문을 채우고 다음 저장 원점이 위로 돌아간 경계다.
/// 큰 개체는 마지막 줄의 시작이 낮아도 그 높이가 쪽 끝까지 점유할 수 있다.
pub(super) fn saved_positive_frame_reset(
    previous: Option<(i32, i32)>,
    first: &LineSeg,
    body_height: i32,
) -> bool {
    previous.is_some_and(|(previous_first, previous_end)| {
        first.vertical_pos > 0
            && first.line_height > 0
            && first.vertical_pos < previous_first
            && first.vertical_pos.saturating_add(first.line_height) <= body_height
            && previous_end.saturating_add(first.line_height) > body_height
    })
}

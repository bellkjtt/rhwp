//! 이미 방출한 저장 호스트 줄과 흐름 밖 그림의 좌표 기준 소유를 확인한다.

use crate::model::{control::Control, paragraph::Paragraph, shape::TextWrap};
use crate::renderer::render_tree::{RenderNode, RenderNodeType};

/// 호스트 줄의 전진이 끝난 뒤 그림만 출력한 항목은 저장 사다리의 원점을 바꾸지 않는다.
/// 가시 문자열 여부와 줄 점유를 구별하며, 실제 방출한 마지막 줄과 전진 결과를 대조한다.
pub(super) fn preserves_line_origin(
    paragraph: &Paragraph,
    para_index: usize,
    control_index: usize,
    column: &RenderNode,
    flow_before: f64,
    flow_after: f64,
) -> bool {
    let common = match paragraph.controls.get(control_index) {
        Some(Control::Picture(picture)) => &picture.common,
        Some(Control::Shape(shape)) => shape.common(),
        _ => return false,
    };
    if common.treat_as_char
        || !matches!(
            common.text_wrap,
            TextWrap::Square
                | TextWrap::Tight
                | TextWrap::Through
                | TextWrap::InFrontOfText
                | TextWrap::BehindText
        )
        || paragraph.stored_text_partition_is_dirty()
        || paragraph.cell_format_vpos_dirty
        || !flow_before.is_finite()
        || !flow_after.is_finite()
    {
        return false;
    }
    // 픽셀 관용치를 두지 않는다. 같은 흐름 원점의 부동소수점 반올림만 허용한다.
    let roundoff = (flow_before.abs() + flow_after.abs()) * f64::EPSILON;
    if (flow_after - flow_before).abs() > roundoff {
        return false;
    }
    let Some((last_index, last)) = paragraph.line_segs.iter().enumerate().next_back() else {
        return false;
    };
    if last.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0 {
        return false;
    }
    let Ok(last_index) = u32::try_from(last_index) else {
        return false;
    };
    // 셀/글상자 안의 줄이나 앞 조각만으로 호스트의 완료를 추정하지 않는다.
    column.children.iter().any(|node| {
        matches!(&node.node_type, RenderNodeType::TextLine(line)
            if line.para_index == Some(para_index) && line.line_index == Some(last_index))
    })
}

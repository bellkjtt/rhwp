//! 저장 줄이 없는 일반 본문의 측정·배치가 함께 소비하는 문단 간격.
//! 합성 줄의 vpos는 앞 간격을 버릴 근거가 아니며, 부분 문단은 첫 조각만 소비한다.

use crate::model::paragraph::Paragraph;
use crate::renderer::layout::ParagraphVerticalSpacing;
use crate::renderer::style_resolver::ResolvedParaStyle;

pub(crate) fn plain_text_spacing(
    para: &Paragraph,
    style: Option<&ResolvedParaStyle>,
    start_line: usize,
) -> Option<ParagraphVerticalSpacing> {
    if !para.controls.is_empty() || !crate::renderer::para_has_no_stored_line_segs(para) {
        return None;
    }
    Some(ParagraphVerticalSpacing {
        before: if start_line == 0 {
            style.map_or(0.0, |s| s.spacing_before)
        } else {
            0.0
        },
        after: style.map_or(0.0, |s| s.spacing_after),
    })
}

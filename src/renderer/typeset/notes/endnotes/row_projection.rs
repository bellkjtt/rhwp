//! 실제 줄 메트릭의 변경을 같은 저장 단 조각의 후속 원점에 투영한다.

use crate::model::footnote::Endnote;
use crate::renderer::style_resolver::ResolvedStyleSet;
use std::borrow::Cow;

pub(super) fn project_equation_rows<'a>(
    original: &'a Endnote,
    styles: &ResolvedStyleSet,
) -> Cow<'a, Endnote> {
    match crate::renderer::composer::stored_row_projection::project_equation_rows(
        &original.paragraphs,
        styles,
    ) {
        Some(paragraphs) => {
            let mut derived = original.clone();
            derived.paragraphs = paragraphs;
            Cow::Owned(derived)
        }
        None => Cow::Borrowed(original),
    }
}

/// 실제 emit과 fit 재생이 같은 미주 시작점·간격 입력을 사용한다.
pub(super) fn render_paragraph(
    original: &crate::model::paragraph::Paragraph,
    endnote_start: i32,
    compact_spacing: bool,
) -> crate::model::paragraph::Paragraph {
    let mut paragraph = original.clone();
    for line in &mut paragraph.line_segs {
        if compact_spacing {
            line.line_spacing = -(line.line_height / 10);
        }
        line.vertical_pos = line.vertical_pos.saturating_add(endnote_start);
    }
    paragraph
}

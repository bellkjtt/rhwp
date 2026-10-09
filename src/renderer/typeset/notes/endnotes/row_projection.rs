//! 실제 줄 메트릭의 변경을 같은 저장 단 조각의 후속 원점에 투영한다.

use crate::model::footnote::Endnote;
use crate::renderer::style_resolver::ResolvedStyleSet;
use std::borrow::Cow;

pub(super) fn project_equation_rows<'a>(
    original: &'a Endnote,
    styles: &ResolvedStyleSet,
    stored_hwpx_columns: bool,
) -> Cow<'a, Endnote> {
    let mut normalized = Cow::Borrowed(original);
    if stored_hwpx_columns {
        for (paragraph_index, paragraph) in original.paragraphs.iter().enumerate() {
            for (index, rows) in paragraph.line_segs.windows(3).enumerate() {
                let before = &rows[0];
                let previous = &rows[1];
                let current = &rows[2];
                let before_advance = before.line_height.checked_add(before.line_spacing);
                let previous_advance = previous.line_height.checked_add(previous.line_spacing);
                // 다음 줄은 새 단 원점에서 직전 줄의 전진만큼 내려간다.
                // 직전 줄의 0 원점만 누락된 저장본은 뒤 줄의 되감김에서
                // 자르면 한 줄을 앞 단에 남긴다. 원문 대신 투영 사본을 고친다.
                if before_advance.is_some_and(|advance| {
                    advance > 0
                        && before.vertical_pos.checked_add(advance) == Some(previous.vertical_pos)
                }) && previous_advance.is_some_and(|advance| {
                    advance > 0
                        && current.vertical_pos == advance
                        && current.vertical_pos < previous.vertical_pos
                }) {
                    normalized.to_mut().paragraphs[paragraph_index].line_segs[index + 1]
                        .vertical_pos = 0;
                }
            }
        }
    }
    match crate::renderer::composer::stored_row_projection::project_equation_rows(
        &normalized.paragraphs,
        styles,
    ) {
        Some(paragraphs) => {
            let mut derived = normalized.into_owned();
            derived.paragraphs = paragraphs;
            Cow::Owned(derived)
        }
        None => normalized,
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

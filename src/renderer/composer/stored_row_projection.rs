//! 재발행한 수식 줄 메트릭을 같은 저장 단 조각의 후속 원점까지 전달한다.

use crate::model::paragraph::Paragraph;
use crate::renderer::style_resolver::ResolvedStyleSet;

pub(crate) fn project_equation_rows(
    original: &[Paragraph],
    styles: &ResolvedStyleSet,
) -> Option<Vec<Paragraph>> {
    let mut derived = original.to_vec();
    let mut shift = 0_i32;
    let mut previous_source_vpos = None;
    let mut changed = false;
    for paragraph in &mut derived {
        let composed = crate::renderer::composer::compose_paragraph_in_context(paragraph, styles);
        // 줄 소유·분할이 달라진 경로는 이 좌표 투영의 계약이 아니다.
        // 같은 저장 줄에 실제 상자 메트릭을 재발행한 경우만 소비한다.
        if composed.lines.len() != paragraph.line_segs.len() {
            return None;
        }
        let source_positions: Vec<_> = paragraph
            .line_segs
            .iter()
            .map(|line| line.vertical_pos)
            .collect();
        for (index, (source, actual)) in paragraph
            .line_segs
            .iter_mut()
            .zip(&composed.lines)
            .enumerate()
        {
            let source_vpos = source.vertical_pos;
            if previous_source_vpos.is_some_and(|previous| source_vpos < previous) {
                // 저장 단/쪽 되감김은 새 좌표 원점이다. 앞 조각의 재조판량을
                // 새 단 원점까지 누적하거나 명시적 나눔을 지우지 않는다.
                shift = 0;
            }
            previous_source_vpos = Some(source_vpos);
            let projected_vpos = source_vpos.saturating_add(shift);
            changed |= projected_vpos != source_vpos;
            source.vertical_pos = projected_vpos;
            if actual.line_height != source.line_height
                || actual.baseline_distance != source.baseline_distance
                || actual.line_spacing != source.line_spacing
            {
                let height_delta = actual.line_height.saturating_sub(source.line_height);
                let spacing_delta = actual.line_spacing.saturating_sub(source.line_spacing);
                // 저장 사다리가 이미 실제 줄의 전진값이면 뒤 원점을 다시 당기지
                // 않는다. 낡은 상자의 전진을 저장한 경우만 재조판량을 투영한다.
                let already_projected = source_positions.get(index + 1).is_some_and(|next| {
                    source_vpos
                        .checked_add(actual.line_height)
                        .and_then(|end| end.checked_add(actual.line_spacing))
                        == Some(*next)
                });
                if !already_projected {
                    shift = shift
                        .saturating_add(height_delta)
                        .saturating_add(spacing_delta);
                }
                source.line_height = actual.line_height;
                source.text_height = actual.line_height;
                source.baseline_distance = actual.baseline_distance;
                source.line_spacing = actual.line_spacing;
                changed = true;
            }
        }
    }
    if changed {
        Some(derived)
    } else {
        None
    }
}

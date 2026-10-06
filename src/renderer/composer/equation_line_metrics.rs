//! 실제 수식 상자로 바뀐 줄의 점유 메트릭을 공통 구성 결과에 반영한다.

use super::{ComposedParagraph, ResolvedStyleSet};
use crate::model::{control::Control, paragraph::Paragraph};
use crate::renderer::{hwpunit_to_px, px_to_hwpunit, DEFAULT_DPI};

pub(super) fn refresh_equation_rows(
    composed: &mut ComposedParagraph,
    para: &Paragraph,
    styles: &ResolvedStyleSet,
) {
    let positions = para.control_text_positions();
    let line_count = composed.lines.len();
    let dpi = styles.resolved_dpi.unwrap_or(DEFAULT_DPI);
    for line_index in 0..composed.lines.len() {
        let stored_pair = para
            .line_segs
            .get(line_index)
            .zip(para.line_segs.get(line_index + 1));
        let end = composed
            .lines
            .get(line_index + 1)
            .map_or(para.text.chars().count(), |line| line.char_start);
        let line = &mut composed.lines[line_index];
        let mut old_ascent = 0_i32;
        let mut old_logical_height = 0_i32;
        let mut new_logical_height = 0_i32;
        let mut new_ascent = 0_i32;
        let mut new_descent = 0_i32;
        let mut changed = false;
        let mut has_equation = false;
        let mut supported = true;
        for (control, position) in para.controls.iter().zip(&positions) {
            if *position < line.char_start
                || *position > end
                || (*position == end && line_index + 1 < line_count)
            {
                continue;
            }
            match control {
                Control::Equation(equation) if equation.common.treat_as_char => {
                    has_equation = true;
                    let old_height = equation.common.height as i32;
                    let old_baseline =
                        old_height.saturating_mul(i32::from(equation.baseline)) / 100;
                    let (_, height, baseline) =
                        crate::renderer::equation::flow_metrics_hwp(equation);
                    let height = height as i32;
                    old_ascent = old_ascent.max(old_baseline);
                    old_logical_height = old_logical_height.max(old_height);
                    new_logical_height = new_logical_height.max(height);
                    new_ascent = new_ascent.max(baseline);
                    new_descent = new_descent.max(height - baseline);
                    changed |= crate::renderer::equation::normalized_simple_math_metrics(equation)
                        .is_some();
                }
                Control::Picture(_) | Control::Table(_) | Control::Shape(_) => supported = false,
                _ => {}
            }
        }
        if !supported || !has_equation {
            continue;
        }
        // 실제 텍스트가 있는 줄은 텍스트의 어센트·하강부도 같은 합집합에 포함한다.
        // 빈 run의 존재만으로 보이지 않는 텍스트 strut를 새로 추가하지 않는다.
        let text_height = line
            .runs
            .iter()
            .filter(|run| !run.text.trim().is_empty())
            .filter_map(|run| styles.char_styles.get(run.char_style_id as usize))
            .map(|style| px_to_hwpunit(style.font_size, dpi))
            .max()
            .unwrap_or(0);
        let text_ascent = (f64::from(text_height) * 0.85).round() as i32;
        let text_descent = text_height - text_ascent;
        old_ascent = old_ascent.max(text_ascent);
        old_logical_height = old_logical_height.max(text_height);
        new_logical_height = new_logical_height.max(text_height);
        new_ascent = new_ascent.max(text_ascent);
        new_descent = new_descent.max(text_descent);
        // 원본 저장 줄의 글자 높이와 다음 원점이 현재 줄 개체의 논리 높이를
        // 함께 증언하면, 다음 줄의 큰 상자를 앞줄에서도 예약하지 않는다.
        // 개체 소유가 바뀐 합성·편집 줄이나 저장 간격이 끊긴 경우는 제외한다.
        let stored_content_cadence = stored_pair.is_some_and(|(source, next)| {
            use crate::model::paragraph::LineSeg;
            !para.stored_text_partition_is_dirty()
                && line_count == para.line_segs.len()
                && source.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                && next.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                && source.text_height == old_logical_height
                && source.line_height > old_logical_height
                && (source.baseline_distance - old_ascent).abs() <= 1
                && source.vertical_pos >= 0
                && source
                    .vertical_pos
                    .checked_add(old_logical_height)
                    .and_then(|end| end.checked_add(source.line_spacing))
                    == Some(next.vertical_pos)
        });
        // 저장 줄이 이 개체 합집합에서 나온 경우만 메트릭을 다시 발행한다.
        // 별도 줄 높이·수직 정렬 계약을 실제 상자의 대용으로 덮어쓰지 않는다.
        if (changed
            && line.line_height == old_logical_height
            && (line.baseline_distance - old_ascent).abs() <= 1)
            || stored_content_cadence
        {
            let occupied_height = new_ascent + new_descent;
            // 저장 간격은 논리 상자 높이 뒤의 leading이다. 실제 기준선 합집합이
            // 더 크면 그 차이는 leading에서 소비해 상자/간격을 이중 예약하지 않는다.
            line.line_spacing = line
                .line_spacing
                .saturating_add(new_logical_height.saturating_sub(occupied_height));
            line.line_height = occupied_height;
            line.baseline_distance = new_ascent;
            debug_assert!(hwpunit_to_px(line.line_height, DEFAULT_DPI) > 0.0);
        }
    }
}

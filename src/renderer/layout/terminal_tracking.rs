//! 정렬의 가시 끝 뒤에는 다음 글자와의 자간이 없다.

use super::super::composer::ComposedLine;
use super::super::style_resolver::ResolvedStyleSet;
use super::text_measurement::estimate_text_width_unrounded;

/// 마지막 가시 글자 뒤의 자간을 반환한다. 음수 자간도 부호를 보존한다.
/// 공백과 개체의 전진량 자체는 바꾸지 않고 정렬 폭에서만 제외한다.
pub(super) fn terminal_tracking_for_alignment(
    line: &ComposedLine,
    styles: &ResolvedStyleSet,
    tac_offsets: &[(usize, f64, usize)],
    trim_trailing_spaces: bool,
) -> f64 {
    if line.runs.iter().any(|run| {
        run.char_overlap.is_some()
                || run.display_text.is_some()
                // 컨트롤 표제는 표시 글자와 원문 scalar의 좌표가 다르다.
                || run.inserted_control_text
    }) {
        return 0.0;
    }
    let mut text_end = line.char_start
        + line
            .runs
            .iter()
            .map(|run| run.source_scalar_count())
            .sum::<usize>();
    for run in line.runs.iter().rev() {
        let text = if trim_trailing_spaces {
            run.text.trim_end_matches(' ')
        } else {
            run.text.as_str()
        };
        text_end -= run.text.chars().count() - text.chars().count();
        if text.is_empty() {
            continue;
        }
        // 마지막 글자 뒤에 개체가 있으면 자간 다음의 내용이 실제로 존재한다.
        if tac_offsets.iter().any(|(pos, _, _)| *pos >= text_end) {
            return 0.0;
        }
        let Some(last) =
            unicode_segmentation::UnicodeSegmentation::graphemes(text, true).next_back()
        else {
            return 0.0;
        };
        if last.chars().any(char::is_whitespace) {
            return 0.0;
        }
        let mut style = run.text_style(styles);
        if style.letter_spacing == 0.0 || style.kerning {
            return 0.0;
        }
        let tracked = estimate_text_width_unrounded(last, &style);
        style.letter_spacing = 0.0;
        return tracked - estimate_text_width_unrounded(last, &style);
    }
    0.0
}

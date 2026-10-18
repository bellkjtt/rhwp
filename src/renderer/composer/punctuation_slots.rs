//! 해소한 구두점 언어 슬롯을 측정과 실제 표시가 함께 소비하는 run에 반영한다.

use super::{ComposedLine, ComposedTextRun, ResolvedStyleSet};
use crate::renderer::style_resolver::ResolvedCharStyle;

fn slot(ch: char, inherited: usize, regenerated: bool, style: Option<&ResolvedCharStyle>) -> usize {
    let Some(style) = style else {
        return inherited;
    };
    // 같은 face여도 원본의 영문 자간·장평·상대 크기는 한글과 다를 수 있다.
    // 중립 문자의 앞 언어를 이 세 속성의 출처로 대신하지 않는다.
    let explicit_metrics_differ = style.font_size_for_lang(inherited)
        != style.font_size_for_lang(1)
        || style.letter_spacing_for_lang(inherited) != style.letter_spacing_for_lang(1)
        || style.ratio_for_lang(inherited) != style.ratio_for_lang(1);
    if explicit_metrics_differ && (matches!(ch, ' ' | '\u{00a0}') || ch.is_ascii_punctuation()) {
        1
    } else if !style.ascii_punct_latin_slot {
        inherited
    } else if ch.is_ascii_punctuation() {
        1
    } else if regenerated {
        super::reflow_punctuation_slot(ch, inherited, true)
    } else {
        inherited
    }
}

/// 줄 채움과 실제 run 분할이 동일한 중립 문자 슬롯을 소비한다.
pub(super) fn metric_slot(
    ch: char,
    inherited: usize,
    styles: &ResolvedStyleSet,
    style_id: u32,
) -> usize {
    slot(
        ch,
        inherited,
        false,
        styles.char_styles.get(style_id as usize),
    )
}

pub(super) fn resolve_run_slots(
    lines: &mut [ComposedLine],
    styles: &ResolvedStyleSet,
    regenerated: bool,
) {
    for line in lines {
        let mut resolved = Vec::new();
        for run in std::mem::take(&mut line.runs) {
            let style = styles.char_styles.get(run.char_style_id as usize);
            if !run
                .text
                .chars()
                .any(|ch| slot(ch, run.lang_index, regenerated, style) != run.lang_index)
            {
                resolved.push(run);
                continue;
            }
            // 원문 글자를 바꾸지 않고 같은 run의 언어 경계만 나눈다. 공백도
            // 명시한 영문 속성이 다르면 그 슬롯을 쓰되 뒤 글자의 상속 언어는 유지한다.
            // 겹침 문자·PUA 표시 변환은 이 구성 결과 뒤에서 한 번 수행된다.
            let mut start = 0;
            let mut lang = run.lang_index;
            for (byte, ch) in run.text.char_indices() {
                let next = slot(ch, run.lang_index, regenerated, style);
                if next != lang {
                    if byte > start {
                        resolved.push(ComposedTextRun {
                            text: run.text[start..byte].to_string(),
                            lang_index: lang,
                            ..run.clone()
                        });
                    }
                    start = byte;
                    lang = next;
                }
            }
            if start < run.text.len() {
                resolved.push(ComposedTextRun {
                    text: run.text[start..].to_string(),
                    lang_index: lang,
                    ..run
                });
            }
        }
        line.runs = resolved;
    }
}

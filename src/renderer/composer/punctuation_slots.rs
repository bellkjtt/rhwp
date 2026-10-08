//! 해소한 구두점 언어 슬롯을 측정과 실제 표시가 함께 소비하는 run에 반영한다.

use super::{ComposedLine, ComposedTextRun, ResolvedStyleSet};

fn slot(ch: char, inherited: usize, regenerated: bool) -> usize {
    if ch.is_ascii_punctuation() {
        1
    } else if regenerated {
        super::reflow_punctuation_slot(ch, inherited, true)
    } else {
        inherited
    }
}

pub(super) fn resolve_run_slots(
    lines: &mut [ComposedLine],
    styles: &ResolvedStyleSet,
    regenerated: bool,
) {
    for line in lines {
        let mut resolved = Vec::new();
        for run in std::mem::take(&mut line.runs) {
            let uses_latin = styles
                .char_styles
                .get(run.char_style_id as usize)
                .is_some_and(|style| style.ascii_punct_latin_slot);
            if !uses_latin
                || !run
                    .text
                    .chars()
                    .any(|ch| slot(ch, run.lang_index, regenerated) != run.lang_index)
            {
                resolved.push(run);
                continue;
            }
            // 원문 글자를 바꾸지 않고 같은 run의 언어 경계만 나눈다. 공백은
            // 상속 슬롯을 유지하며 뒤의 한글·인라인 개체의 scalar 소속을 전진시키지 않는다.
            // 겹침 문자·PUA 표시 변환은 이 구성 결과 뒤에서 한 번 수행된다.
            let mut start = 0;
            let mut lang = run.lang_index;
            for (byte, ch) in run.text.char_indices() {
                let next = slot(ch, run.lang_index, regenerated);
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

//! 수학 변수의 글리프 선택. 원본 AST는 보존하고 표시 문자열만 투영한다.
//!
//! MathML의 수학 알파벳 매핑을 따른다. 수학 face의 이탤릭 글리프에
//! 일반 글꼴의 합성 기울임을 다시 적용하지 않도록 출력 경로가 같은 판정을 쓴다.

pub(super) fn display_text(text: &str, italic: bool, bold: bool) -> String {
    if !italic || text.is_empty() || !text.chars().all(|ch| ch.is_ascii_alphabetic()) {
        return text.to_owned();
    }
    text.chars()
        .map(|ch| {
            let code = match (bold, ch.is_ascii_uppercase()) {
                (false, true) => 0x1D434 + u32::from(ch) - u32::from('A'),
                (false, false) if ch == 'h' => 0x210E,
                (false, false) => 0x1D44E + u32::from(ch) - u32::from('a'),
                (true, true) => 0x1D468 + u32::from(ch) - u32::from('A'),
                (true, false) => 0x1D482 + u32::from(ch) - u32::from('a'),
            };
            char::from_u32(code).expect("수학 알파벳 매핑은 유효한 Unicode scalar다")
        })
        .collect()
}

pub(crate) fn is_styled_variable(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|ch| matches!(ch, '\u{1D400}'..='\u{1D7FF}' | '\u{210E}'))
}

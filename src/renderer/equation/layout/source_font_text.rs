//! HYhwpEQ의 기본 소문자 이탤릭 전진폭으로 글리프 원점을 구성한다.
//! 설치 글꼴 SHA-256: a3a3cd992a89ac38e7707b58143ebdb1da9511d3c462f019cda7f92c30467807.
//! 원본 MCP Print의 subset에 포함된 130개 hmtx 값과 설치 글꼴이 일치한다.
//! 사설 글꼴 바이트·윤곽은 포함하지 않는다. 공개 표시 글리프의 모양은 유지한다.

use super::{variable_style, LayoutBox, LayoutKind};

// UPM 1024, 기본 이탤릭 a..z의 cmap U+E0E5..U+E0FE.
const ADVANCES: [u16; 26] = [
    541, 439, 442, 532, 476, 553, 487, 590, 352, 421, 532, 305, 899, 614, 496, 515, 457, 462, 480,
    369, 585, 496, 732, 585, 502, 476,
];

pub(super) fn layout(text: &str, fs: f64) -> Option<LayoutBox> {
    if text.is_empty() || !text.bytes().all(|ch| ch.is_ascii_lowercase()) {
        return None;
    }
    let mut x = 0.0;
    let mut children = Vec::with_capacity(text.len());
    for ch in text.bytes() {
        let width = f64::from(ADVANCES[usize::from(ch - b'a')]) * fs / 1024.0;
        children.push(LayoutBox {
            font_size: fs,
            x,
            y: 0.0,
            width,
            height: fs,
            baseline: fs * 0.8,
            kind: LayoutKind::Text(variable_style::display_text(
                &char::from(ch).to_string(),
                true,
                false,
            )),
        });
        x += width;
    }
    Some(LayoutBox {
        font_size: fs,
        x: 0.0,
        y: 0.0,
        width: x,
        height: fs,
        baseline: fs * 0.8,
        kind: LayoutKind::Row(children),
    })
}

//! HYhwpEQ에 없는 미분 기호의 독립 Print 대체 face를 소비한다.
//! 원본 11월 Print의 Haansoft Batang subset과 설치 HBATANG.TTF는
//! U+2032의 hmtx 전진폭 276, UPM 1024가 같다. 글꼴 바이트·윤곽은 포함하지 않는다.

use super::{LayoutBox, LayoutKind};

/// 한컴 Print의 수학 전진폭과 자동 간격. 정체 ASCII 글꼴의 폭과 구분한다.
pub(super) fn operator_metrics(text: &str, fs: f64, dpi: f64) -> Option<(f64, f64)> {
    let (advance, padding) = match text {
        "+" => (795, 0.14),
        "=" => (795, 0.21),
        "×" => (831, 0.14),
        _ => return None,
    };
    Some((
        super::source_font_text::logical_advance(advance, fs, dpi),
        fs * padding,
    ))
}

pub(super) fn layout(text: &str, fs: f64) -> Option<LayoutBox> {
    if !matches!(text, "'" | "′") {
        return None;
    }
    Some(LayoutBox {
        font_size: fs,
        x: 0.0,
        y: 0.0,
        width: fs * 276.0 / 1024.0,
        height: fs,
        baseline: fs * 0.8,
        kind: LayoutKind::FontGlyph {
            text: "′".to_string(),
            font_family: "Haansoft Batang, 한컴바탕, Batang, 바탕, serif",
        },
    })
}

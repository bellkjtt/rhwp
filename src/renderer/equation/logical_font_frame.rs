//! 한컴의 단일 글자 줄·윗줄 장식에 대한 논리 프레임.
//!
//! 같은 스크립트의 독립 MCP 2020 재저장에서 일반 줄은 BaseUnit과
//! 86% 기준선, 단일 글자 줄의 bar는 1.2배 높이와 88% 기준선을,
//! 단일 글자 위첨자는 7/6배 높이와 88% 기준선을 기록한다.
//! 글자 줄만 있는 분자·분모의 분수는 2.3배 높이와 66% 기준선이다.
//! 대체 글꼴의 글리프 상자와 저장 흐름 프레임을 구분한다.

use super::layout::{LayoutBox, LayoutKind};
use super::symbols::DecoKind;

fn font_line(value: &LayoutBox, fs: f64) -> bool {
    if (value.height - fs).abs() > f64::EPSILON * fs * 4.0 {
        return false;
    }
    match &value.kind {
        LayoutKind::Text(_)
        | LayoutKind::Number(_)
        | LayoutKind::Symbol(_)
        | LayoutKind::MathSymbol(_)
        | LayoutKind::Function(_)
        | LayoutKind::Space(_) => true,
        LayoutKind::Row(items) => items.iter().all(|item| font_line(item, fs)),
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => font_line(body, fs),
        _ => false,
    }
}

fn frame(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    if font_line(value, fs) {
        return Some((fs, fs * 0.86));
    }
    match &value.kind {
        LayoutKind::Decoration {
            kind: DecoKind::Bar,
            body,
        } if font_line(body, fs) => Some((fs * 1.2, fs * 1.2 * 0.88)),
        LayoutKind::Superscript { base, sup }
            if font_line(base, fs) && font_line(sup, fs * super::layout::SCRIPT_SCALE) =>
        {
            Some((fs * 7.0 / 6.0, fs * 7.0 / 6.0 * 0.88))
        }
        LayoutKind::Fraction { numer, denom } if font_line(numer, fs) && font_line(denom, fs) => {
            Some((fs * 2.3, fs * 2.3 * 0.66))
        }
        LayoutKind::FontStyle { body, .. } => frame(body, fs),
        LayoutKind::Row(items) => {
            let mut ascent = 0.0_f64;
            let mut descent = 0.0_f64;
            for item in items {
                let (height, baseline) = frame(item, fs)?;
                ascent = ascent.max(baseline);
                descent = descent.max(height - baseline);
            }
            Some((ascent + descent, ascent))
        }
        _ => None,
    }
}

/// 중첩 분수·배열·아래첨자·복합 첨자·근호·다줄·다중 장식은 이 계약이 아니다.
pub(super) fn metrics(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    frame(value, fs)
}

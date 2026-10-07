//! 한컴의 단일 글자 줄·윗줄 장식에 대한 논리 프레임.
//!
//! 독립 한컴 재생성의 글자 크기별 메트릭을 소비한다. 일반 글자 줄,
//! 윗줄·위첨자·단순 분수의 논리 프레임을 대체 글꼴의 자연 상자와 구분한다.

use super::layout::{LayoutBox, LayoutKind};
use super::symbols::DecoKind;

// 크기·기준선·저장 줄을 비운 입력 84개를 한컴 2020에서 재생성한 논리 프레임.
// job 9e55a726-899d-4a7f-90e1-1e6a277946cb, 11.0.0.9136.
// BaseUnit, 윗줄 위첨자의 높이·기준선(%), 그 표현식 근호의 높이·기준선(%).
// 문서 좌표나 특정 수식 문자열을 기대값으로 사용하지 않는다.
const BAR_SUP_FRAMES: [[u32; 5]; 14] = [
    [600, 805, 89, 895, 90],
    [700, 928, 89, 1033, 90],
    [800, 1116, 89, 1228, 90],
    [900, 1227, 89, 1362, 91],
    [1000, 1349, 89, 1499, 90],
    [1100, 1538, 89, 1698, 90],
    [1200, 1648, 90, 1828, 91],
    [1300, 1771, 89, 1966, 91],
    [1400, 1959, 89, 2165, 90],
    [1500, 2070, 90, 2295, 91],
    [1600, 2192, 90, 2432, 91],
    [1800, 2454, 89, 2724, 91],
    [2000, 2765, 89, 3057, 91],
    [2400, 3297, 90, 3657, 91],
];

fn bar_sup_metrics(fs: f64, radical: bool) -> Option<(f64, f64)> {
    let hu = crate::renderer::px_to_hwpunit(fs, crate::renderer::DEFAULT_DPI) as u32;
    let row = BAR_SUP_FRAMES.iter().find(|row| row[0] == hu)?;
    let index = if radical { 3 } else { 1 };
    let height = crate::renderer::hwpunit_to_px(row[index] as i32, crate::renderer::DEFAULT_DPI);
    Some((height, height * f64::from(row[index + 1]) / 100.0))
}

fn bar_font_line(value: &LayoutBox, fs: f64) -> bool {
    match &value.kind {
        LayoutKind::Decoration {
            kind: DecoKind::Bar,
            body,
        } => font_line(body, fs),
        LayoutKind::FontStyle { body, .. } => bar_font_line(body, fs),
        LayoutKind::Row(items) if items.len() == 1 => bar_font_line(&items[0], fs),
        _ => false,
    }
}

/// 일반 글자와 윗줄의 단일 위첨자로 구성한 피근호식만 수용한다.
/// 중첩 근호·분수·아래첨자·근 지수·별도 크기 변경을 같은 프레임으로 접지 않는다.
fn bar_sup_radical_body(value: &LayoutBox, fs: f64) -> Option<bool> {
    if font_line(value, fs) {
        return Some(false);
    }
    match &value.kind {
        LayoutKind::Superscript { base, sup }
            if bar_font_line(base, fs) && font_line(sup, sup.font_size) =>
        {
            Some(true)
        }
        LayoutKind::FontStyle { body, .. } => bar_sup_radical_body(body, fs),
        LayoutKind::Row(items) => items.iter().try_fold(false, |has_sup, item| {
            Some(has_sup | bar_sup_radical_body(item, fs)?)
        }),
        _ => None,
    }
}

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

fn contains_prime(value: &LayoutBox) -> bool {
    match &value.kind {
        LayoutKind::Symbol(s) | LayoutKind::MathSymbol(s) => {
            s.chars().any(|ch| matches!(ch, '′' | '″' | '‴'))
        }
        LayoutKind::Row(items) => items.iter().any(contains_prime),
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => contains_prime(body),
        _ => false,
    }
}

fn frame(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    let metrics = super::font_frame_metrics::for_size(fs)?;
    if font_line(value, fs) {
        return Some(if contains_prime(value) {
            metrics.prime_line
        } else {
            metrics.line
        });
    }
    match &value.kind {
        LayoutKind::Decoration {
            kind: DecoKind::Bar,
            body,
        } if font_line(body, fs) => Some(if contains_prime(body) {
            metrics.prime_bar
        } else {
            metrics.bar
        }),
        LayoutKind::Superscript { base, sup }
            if bar_font_line(base, fs) && font_line(sup, sup.font_size) =>
        {
            bar_sup_metrics(fs, false)
        }
        LayoutKind::Sqrt { index: None, body } if bar_sup_radical_body(body, fs) == Some(true) => {
            bar_sup_metrics(fs, true)
        }
        LayoutKind::Superscript { base, sup }
            if font_line(base, fs) && font_line(sup, sup.font_size) =>
        {
            Some(metrics.sup)
        }
        LayoutKind::Fraction { numer, denom } if font_line(numer, fs) && font_line(denom, fs) => {
            let numer_height = if contains_prime(numer) {
                metrics.prime_line.0
            } else {
                metrics.line.0
            };
            let denom_height = if contains_prime(denom) {
                metrics.prime_line.0
            } else {
                metrics.line.0
            };
            let extra_numer = (numer_height - metrics.line.0).max(0.0);
            let extra_denom = (denom_height - metrics.line.0).max(0.0);
            let height = metrics.fraction.0 + extra_numer + extra_denom;
            // HWP의 기준선 저장 정밀도와 같은 %로 양자화한다.
            let percent = ((metrics.fraction.1 + extra_numer) / height * 100.0).round();
            Some((height, height * percent / 100.0))
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

/// 중첩 분수·배열·아래첨자·복합 첨자·일반 근호·다줄·다중 장식은 이 계약이 아니다.
pub(super) fn metrics(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    frame(value, fs)
}

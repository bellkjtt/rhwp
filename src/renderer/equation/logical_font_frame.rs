//! 한컴의 단일 글자 줄·윗줄 장식에 대한 논리 프레임.
//!
//! 독립 한컴 재생성의 글자 크기별 메트릭을 소비한다. 일반 글자 줄,
//! 윗줄·위첨자·단순 분수의 논리 프레임을 대체 글꼴의 자연 상자와 구분한다.

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

/// 중첩 분수·배열·아래첨자·복합 첨자·근호·다줄·다중 장식은 이 계약이 아니다.
pub(super) fn metrics(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    frame(value, fs)
}

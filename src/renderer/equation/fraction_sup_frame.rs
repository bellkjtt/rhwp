//! 한컴의 일반 글자 분수 뒤 위첨자 프레임을 배치와 논리 높이에 함께 공급한다.
//! 같은 15개 크기의 짧은 본체·괄호 분수·보통 분수를 독립 저장/Print로 대조했다.

use super::layout::{LayoutBox, LayoutKind};
use super::logical_font_frame::{contains_prime, font_line};

// 한컴 2020 11.0.0.9136, 저장 737de7ec-3d16-4f4f-ae82-68a34d7ed056,
// Print d5266fcf-23bc-42d1-b80a-964998fcacbb의 HWPUNIT 높이.
// 괄호의 유무에 관계없이 같은 분수·위첨자 프로그램의 기준선은 모두 68%다.
// 문서 ID·표현식 문자열·화면 좌표를 키로 사용하지 않는다.
const FRAMES: [[u32; 2]; 15] = [
    [600, 1465],
    [700, 1666],
    [800, 2016],
    [900, 2217],
    [1000, 2417],
    [1100, 2768],
    [1200, 2968],
    [1300, 3169],
    [1400, 3519],
    [1500, 3720],
    [1600, 3920],
    [1800, 4434],
    [2000, 4985],
    [2200, 5386],
    [2400, 5937],
];

fn contains_cjk(value: &LayoutBox) -> bool {
    match &value.kind {
        LayoutKind::Text(text)
        | LayoutKind::Function(text)
        | LayoutKind::FontGlyph { text, .. } => super::text_has_cjk(text),
        LayoutKind::Row(items) => items.iter().any(contains_cjk),
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => contains_cjk(body),
        _ => false,
    }
}

fn ordinary_line(value: &LayoutBox, fs: f64) -> bool {
    font_line(value, fs) && !contains_prime(value) && !contains_cjk(value)
}

fn ordinary_fraction(value: &LayoutBox, fs: f64) -> bool {
    match &value.kind {
        LayoutKind::Fraction { numer, denom, .. } => {
            ordinary_line(numer, fs) && ordinary_line(denom, fs)
        }
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => {
            ordinary_fraction(body, fs)
        }
        LayoutKind::Row(items) if items.len() == 1 => ordinary_fraction(&items[0], fs),
        _ => false,
    }
}

pub(super) fn frame(base: &LayoutBox, sup: &LayoutBox, fs: f64, dpi: f64) -> Option<(f64, f64)> {
    if !ordinary_fraction(base, fs)
        || !ordinary_line(sup, sup.font_size)
        || (sup.font_size - fs * 0.68).abs() > f64::EPSILON * fs * 4.0
    {
        return None;
    }
    let unit = super::font_frame_metrics::base_unit_at_dpi(fs, dpi);
    let row = FRAMES.iter().find(|row| row[0] == unit)?;
    let height = crate::renderer::hwpunit_to_px(row[1] as i32, dpi);
    Some((height, height * 0.68))
}

pub(super) fn shift(base: &LayoutBox, sup: &LayoutBox, fs: f64, dpi: f64) -> Option<f64> {
    let (_, fraction_baseline) = frame(base, sup, fs, dpi)?;
    let (sup_height, sup_baseline) = super::font_frame_metrics::superscript_at_dpi(fs, dpi)?;
    let (line_height, line_baseline) = super::font_frame_metrics::line_at_dpi(fs, dpi)?;
    // 독립 Print에서 짧은 본체와 분수의 첨자 상단은 같은 글리프 프로그램이다.
    // 기존 단일 글자 첨자의 기준선 위치를 소비해 큰 본체의 shift를 해소한다.
    let plain_shift = sup_height - line_height + line_baseline * (1.0 - sup.font_size / fs);
    Some(fraction_baseline - sup_baseline + plain_shift)
}

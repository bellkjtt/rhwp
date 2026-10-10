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

fn bar_sup_metrics(fs: f64, radical: bool, dpi: f64) -> Option<(f64, f64)> {
    let hu = super::font_frame_metrics::base_unit_at_dpi(fs, dpi);
    let row = BAR_SUP_FRAMES.iter().find(|row| row[0] == hu)?;
    let index = if radical { 3 } else { 1 };
    let height = crate::renderer::hwpunit_to_px(row[index] as i32, dpi);
    Some((height, height * f64::from(row[index + 1]) / 100.0))
}

fn bar_font_line(value: &LayoutBox, fs: f64) -> bool {
    accent_font_line(value, fs, DecoKind::Bar)
}

fn accent_font_line(value: &LayoutBox, fs: f64, accent: DecoKind) -> bool {
    match &value.kind {
        LayoutKind::Decoration { kind, body, .. } if *kind == accent => font_line(body, fs),
        LayoutKind::FontStyle { body, .. } => accent_font_line(body, fs, accent),
        LayoutKind::Row(items) if items.len() == 1 => accent_font_line(&items[0], fs, accent),
        _ => false,
    }
}

/// 독립 한컴 정상 저장의 벡터 바깥 위첨자 프레임.
/// 32개 본체 대조군의 9/11/16/22pt에서 확인했다. 다른 장식·중첩 본체는 수용하지 않는다.
fn vector_sup_metrics(fs: f64, dpi: f64) -> Option<(f64, f64)> {
    // job b599ad0e-cb8d-449d-92d5-bf289839d52d, Print d69615b4-ca03-449f-8288-82d7196787e2.
    const FRAMES: [[u32; 3]; 4] = [
        [900, 1230, 90],
        [1100, 1538, 89],
        [1600, 2228, 90],
        [2200, 3023, 90],
    ];
    let unit = super::font_frame_metrics::base_unit_at_dpi(fs, dpi);
    let row = FRAMES.iter().find(|row| row[0] == unit)?;
    let height = crate::renderer::hwpunit_to_px(row[1] as i32, dpi);
    Some((height, height * f64::from(row[2]) / 100.0))
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

/// Row는 CASES·PILE의 세로 묶음에도 쓰인다. 실제 자식 기준선이 같은
/// 가로 줄만 글꼴 프레임으로 합친다. 세로 행의 y를 버리면 저장 높이가 줄어든다.
pub(super) fn horizontal_row(value: &LayoutBox, items: &[LayoutBox]) -> bool {
    let roundoff = f64::EPSILON * value.height.abs().max(1.0) * 8.0;
    items
        .iter()
        .all(|item| (item.y + item.baseline - value.baseline).abs() <= roundoff)
}

pub(super) fn font_line(value: &LayoutBox, fs: f64) -> bool {
    if (value.height - fs).abs() > f64::EPSILON * fs * 4.0 {
        return false;
    }
    match &value.kind {
        LayoutKind::Text(_)
        | LayoutKind::Number(_)
        | LayoutKind::Symbol(_)
        | LayoutKind::MathSymbol(_)
        | LayoutKind::FontGlyph { .. }
        | LayoutKind::Function(_)
        | LayoutKind::Space(_) => true,
        LayoutKind::Row(items) => {
            horizontal_row(value, items) && items.iter().all(|item| font_line(item, fs))
        }
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => font_line(body, fs),
        _ => false,
    }
}

/// 같은 글자 크기의 DYAD 행은 예약 프레임 안의 자연 표시 원점을 유지한다.
/// 분수·첨자·크기 변경·다줄의 장식 본체는 이 글자 줄 계약으로 접지 않는다.
pub(super) fn dyad_font_line(value: &LayoutBox, fs: f64) -> bool {
    fn contains_cjk(value: &LayoutBox) -> bool {
        match &value.kind {
            LayoutKind::Text(text)
            | LayoutKind::Number(text)
            | LayoutKind::Symbol(text)
            | LayoutKind::MathSymbol(text)
            | LayoutKind::Function(text)
            | LayoutKind::FontGlyph { text, .. } => super::text_has_cjk(text),
            LayoutKind::FontStyle { body, .. }
            | LayoutKind::Decoration { body, .. }
            | LayoutKind::Paren { body, .. } => contains_cjk(body),
            LayoutKind::Row(items) => items.iter().any(contains_cjk),
            _ => false,
        }
    }
    fn contains_dyad(value: &LayoutBox, fs: f64) -> Option<bool> {
        match &value.kind {
            LayoutKind::Decoration {
                kind: DecoKind::Dyad,
                body,
                ..
            } if font_line(body, fs) => Some(true),
            LayoutKind::FontStyle { body, .. } => contains_dyad(body, fs),
            LayoutKind::Row(items) => items
                .iter()
                .try_fold(false, |found, item| Some(found | contains_dyad(item, fs)?)),
            _ if font_line(value, fs) => Some(false),
            _ => None,
        }
    }
    // HYhwpEQ의 cmap에 없는 한글은 다른 본문 프로그램으로 표시된다.
    // 그 경로를 수학 글꼴의 글자 줄과 같은 원점 계약으로 판단하지 않는다.
    !contains_cjk(value) && contains_dyad(value, fs) == Some(true)
}

/// 단일 글자 줄 위첨자의 기준선 거리를 독립 한컴 논리 프레임에서 해소한다.
/// 공개 표시 글꼴의 MATH shift를 원본 face의 프레임과 혼용하지 않는다.
pub(super) fn superscript_shift(
    base: &LayoutBox,
    sup: &LayoutBox,
    fs: f64,
    dpi: f64,
) -> Option<f64> {
    // 벡터는 본체 글자의 기준선을 바꾸지 않는다. 장식 높이를 일반 MATH
    // shift로 대신하지 않고 같은 크기의 글줄·첨자 기준선 거리를 소비한다.
    let vector_base = accent_font_line(base, fs, DecoKind::Vec)
        && !contains_prime(base)
        && vector_sup_metrics(fs, dpi).is_some();
    if !(font_line(base, fs) || vector_base)
        || !font_line(sup, sup.font_size)
        || (sup.font_size - fs * 0.68).abs() > f64::EPSILON * fs * 4.0
        || (super::font_frame_metrics::base_unit_at_dpi(fs, dpi) == 2200
            && (contains_prime(base) || contains_prime(sup)))
    {
        return None;
    }
    let (sup_height, _) = super::font_frame_metrics::superscript_at_dpi(fs, dpi)?;
    let (line_height, line_baseline) = super::font_frame_metrics::line_at_dpi(fs, dpi)?;
    let shift = sup_height - line_height + line_baseline * (1.0 - sup.font_size / fs);
    Some(shift)
}

/// 같은 일반 글줄의 첨자 프레임에서 하단 점유를 공유한다.
/// 자식의 상단 위치 대신 본체·첨자 기준선과 실제 글줄 깊이를 연결한다.
pub(super) fn subscript_geometry(
    base: &LayoutBox,
    sub: &LayoutBox,
    fs: f64,
    dpi: f64,
) -> Option<(f64, f64)> {
    if !font_line(base, fs)
        || !font_line(sub, sub.font_size)
        || (sub.font_size - fs * 0.68).abs() > f64::EPSILON * fs * 4.0
    {
        return None;
    }
    let (height, baseline) = super::font_frame_metrics::subscript_at_dpi(fs, dpi)?;
    let (line_height, line_baseline) = super::font_frame_metrics::line_at_dpi(fs, dpi)?;
    let sub_depth = (line_height - line_baseline) * sub.font_size / fs;
    let baseline_drop = height - baseline - sub_depth;
    Some((base.baseline + baseline_drop - sub.baseline, height))
}

pub(super) fn contains_prime(value: &LayoutBox) -> bool {
    match &value.kind {
        LayoutKind::Symbol(s)
        | LayoutKind::MathSymbol(s)
        | LayoutKind::FontGlyph { text: s, .. } => {
            s.chars().any(|ch| matches!(ch, '′' | '″' | '‴'))
        }
        LayoutKind::Row(items) => items.iter().any(contains_prime),
        LayoutKind::FontStyle { body, .. }
        | LayoutKind::Paren { body, .. }
        | LayoutKind::Decoration { body, .. } => contains_prime(body),
        _ => false,
    }
}

fn frame(value: &LayoutBox, fs: f64, dpi: f64) -> Option<(f64, f64)> {
    frame_with_style(value, fs, dpi, false)
}

fn frame_with_style(value: &LayoutBox, fs: f64, dpi: f64, roman: bool) -> Option<(f64, f64)> {
    use super::symbols::FontStyleKind;
    match &value.kind {
        LayoutKind::FontStyle {
            style: FontStyleKind::Roman,
            body,
        } => {
            return frame_with_style(body, fs, dpi, true);
        }
        LayoutKind::FontStyle {
            style: FontStyleKind::Italic,
            body,
        } => {
            return frame_with_style(body, fs, dpi, false);
        }
        LayoutKind::Paren { body, .. } => return frame_with_style(body, fs, dpi, roman),
        LayoutKind::Row(items) if !items.is_empty() && horizontal_row(value, items) => {
            let mut ascent = 0.0_f64;
            let mut descent = 0.0_f64;
            for item in items {
                let (height, baseline) = frame_with_style(item, fs, dpi, roman)?;
                ascent = ascent.max(baseline);
                descent = descent.max(height - baseline);
            }
            return Some((ascent + descent, ascent));
        }
        LayoutKind::Decoration {
            kind: DecoKind::Bar | DecoKind::Vec | DecoKind::Dyad,
            body,
            ..
        } if font_line(body, fs) => {
            let prime = contains_prime(body);
            if roman && !prime {
                if let Some(frame) = super::font_frame_metrics::roman_at_dpi(fs, dpi, true) {
                    return Some(frame);
                }
            }
            let (accent_height, accent_baseline) =
                super::font_frame_metrics::accent_at_dpi(fs, dpi, prime)?;
            // 장식 안의 정체 명령은 본체 줄 높이를 먼저 바꾼다. 외부 정체 명령의
            // 장식 프레임을 그 자리에 대신 쓰지 않는다. 독립 16/22pt의 내부/외부
            // 정체 입력은 각각 1915/1902HU, 2635/2622HU로 서로 다르다.
            let base_line_height = super::font_frame_metrics::line_at_dpi(fs, dpi)?.0;
            let body_line_height = frame_with_style(body, fs, dpi, roman)
                .map_or(base_line_height, |(height, _)| height);
            let height = accent_height + body_line_height - base_line_height;
            return Some((height, height * accent_baseline / accent_height));
        }
        _ => {}
    }
    if roman && font_line(value, fs) && !contains_prime(value) {
        if let Some(frame) = super::font_frame_metrics::roman_at_dpi(fs, dpi, false) {
            return Some(frame);
        }
    }
    if let LayoutKind::Superscript { base, sup } = &value.kind {
        if accent_font_line(base, fs, DecoKind::Vec)
            && !contains_prime(base)
            && font_line(sup, sup.font_size)
            && (sup.font_size - fs * 0.68).abs() <= f64::EPSILON * fs * 4.0
        {
            return vector_sup_metrics(fs, dpi);
        }
    }
    if let LayoutKind::Subscript { base, sub } = &value.kind {
        if subscript_geometry(base, sub, fs, dpi).is_some() {
            return super::font_frame_metrics::subscript_at_dpi(fs, dpi);
        }
    }
    if super::font_frame_metrics::base_unit_at_dpi(fs, dpi) == 2200 {
        return super::font_frame_22::frame(value, fs, dpi);
    }
    let metrics = super::font_frame_metrics::for_size_at_dpi(fs, dpi)?;
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
            ..
        } if font_line(body, fs) => Some(if contains_prime(body) {
            metrics.prime_bar
        } else {
            metrics.bar
        }),
        LayoutKind::Superscript { base, sup }
            if bar_font_line(base, fs) && font_line(sup, sup.font_size) =>
        {
            bar_sup_metrics(fs, false, dpi)
        }
        LayoutKind::Sqrt { index: None, body } if bar_sup_radical_body(body, fs) == Some(true) => {
            bar_sup_metrics(fs, true, dpi)
        }
        LayoutKind::Superscript { base, sup }
            if font_line(base, fs) && font_line(sup, sup.font_size) =>
        {
            Some(metrics.sup)
        }
        LayoutKind::Fraction { numer, denom, .. }
            if font_line(numer, fs) && font_line(denom, fs) =>
        {
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
        LayoutKind::FontStyle { body, .. } | LayoutKind::Paren { body, .. } => frame(body, fs, dpi),
        LayoutKind::Row(items) if horizontal_row(value, items) => {
            let mut ascent = 0.0_f64;
            let mut descent = 0.0_f64;
            for item in items {
                let (height, baseline) = frame(item, fs, dpi)?;
                ascent = ascent.max(baseline);
                descent = descent.max(height - baseline);
            }
            Some((ascent + descent, ascent))
        }
        _ => None,
    }
}

/// 중첩 분수·배열·중첩 첨자·복합 첨자·일반 근호·다줄·다중 장식은 이 계약이 아니다.
pub(super) fn metrics(value: &LayoutBox, fs: f64) -> Option<(f64, f64)> {
    frame(value, fs, crate::renderer::DEFAULT_DPI)
}

pub(super) fn metrics_at_dpi(value: &LayoutBox, fs: f64, dpi: f64) -> Option<(f64, f64)> {
    frame(value, fs, dpi)
}

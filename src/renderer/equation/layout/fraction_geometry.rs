//! 표시 글꼴의 MATH 메트릭으로 분자·분모의 기준선과 점유를 함께 계산한다.
//!
//! LatinModernMath-Regular.woff2, UPM 1000, SHA-256:
//! 1177d9025d1e797c54b3c7676976365abd90f903423091de7f0f4e7a4b372162.
//! 배치 계약: https://www.w3.org/TR/mathml-core/#fraction-with-nonzero-line-thickness

use super::LayoutBox;

pub(super) fn position(numer: &mut LayoutBox, denom: &mut LayoutBox, fs: f64) -> (f64, f64) {
    let axis = fs * super::AXIS_HEIGHT;
    let half_rule = fs * super::FRAC_LINE_THICK * 0.5;
    let gap = fs * 120.0 / 1000.0;
    // FractionNumeratorDisplayStyleShiftUp와 FractionDenominatorDisplayStyleShiftDown.
    // 복합 분수에서도 자식의 기준선·하강부를 소비해 괘선과 겹치지 않게 한다.
    let numer_shift =
        (fs * 677.0 / 1000.0).max(axis + half_rule + gap + numer.height - numer.baseline);
    let denom_shift = (fs * 686.0 / 1000.0).max(half_rule + gap + denom.baseline - axis);
    let ascent = (numer_shift + numer.baseline)
        .max(denom.baseline - denom_shift)
        .max(axis + half_rule);
    let descent = (numer.height - numer.baseline - numer_shift)
        .max(denom_shift + denom.height - denom.baseline)
        .max(half_rule - axis)
        .max(0.0);
    numer.y = ascent - numer_shift - numer.baseline;
    denom.y = ascent + denom_shift - denom.baseline;
    (ascent + descent, ascent)
}

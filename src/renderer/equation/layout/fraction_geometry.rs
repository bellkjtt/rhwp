//! 표시 글꼴의 MATH 메트릭으로 분자·분모의 기준선과 점유를 함께 계산한다.
//!
//! LatinModernMath-Regular.woff2, UPM 1000, SHA-256:
//! 1177d9025d1e797c54b3c7676976365abd90f903423091de7f0f4e7a4b372162.
//! 배치 계약: https://www.w3.org/TR/mathml-core/#fraction-with-nonzero-line-thickness

use super::LayoutBox;

pub(super) fn position(
    numer: &mut LayoutBox,
    denom: &mut LayoutBox,
    fs: f64,
    axis: f64,
) -> (f64, f64) {
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

use super::{
    EqLayout, EqNode, LayoutKind, AXIS_HEIGHT, FRAC_LINE_PAD, RADICAL_DEGREE_SCALE, SQRT_PAD,
};

impl EqLayout {
    pub(super) fn layout_fraction(&self, numer: &EqNode, denom: &EqNode, fs: f64) -> LayoutBox {
        let n = self.layout_node(numer, fs);
        let d = self.layout_node(denom, fs);

        // 한컴이 새로 생성한 단순 분수는 반각 숫자와 양쪽 1/4 em을 예약한다.
        // 세로 기준선 간격과 수평 괘선 여백을 같은 임의 비율로 묶지 않는다.
        let pad = fs
            * if self.hancom_math_face {
                0.25
            } else {
                FRAC_LINE_PAD
            };
        let w = n.width.max(d.width) + pad * 2.0;

        let mut n_box = n;
        n_box.x = (w - n_box.width) / 2.0;
        let mut d_box = d;
        d_box.x = (w - d_box.width) / 2.0;
        let axis_height = self.math_axis_height(fs);
        let (total_h, baseline) = position(&mut n_box, &mut d_box, fs, axis_height);

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: w,
            height: total_h,
            baseline,
            kind: LayoutKind::Fraction {
                numer: Box::new(n_box),
                denom: Box::new(d_box),
                axis_height,
            },
        }
    }

    pub(super) fn layout_atop(&self, top: &EqNode, bottom: &EqNode, fs: f64) -> LayoutBox {
        let t = self.layout_node(top, fs);
        let b = self.layout_node(bottom, fs);

        let pad = fs * FRAC_LINE_PAD;
        let axis = fs * AXIS_HEIGHT;
        let w = t.width.max(b.width) + pad * 2.0;

        let top_h = t.height + pad;
        let bottom_h = b.height + pad;
        let baseline = top_h + axis;
        let total_h = top_h + bottom_h;

        let mut top_box = t;
        top_box.x = (w - top_box.width) / 2.0;
        top_box.y = pad;

        let mut bottom_box = b;
        bottom_box.x = (w - bottom_box.width) / 2.0;
        bottom_box.y = top_h;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: w,
            height: total_h,
            baseline,
            kind: LayoutKind::Atop {
                top: Box::new(top_box),
                bottom: Box::new(bottom_box),
            },
        }
    }

    pub(super) fn layout_sqrt(
        &self,
        index: &Option<Box<EqNode>>,
        body: &EqNode,
        fs: f64,
    ) -> LayoutBox {
        let b = self.layout_node(body, fs);
        let pad = fs * SQRT_PAD;
        // 수식 face의 논리 전진폭. HYhwpEQ의 수학 radical은 1em,
        // 배포 Latin Modern Math의 U+221A는 833/1000em이다.
        let sign_w = if self.hancom_math_face {
            super::source_roman_advance::em_advance(fs, self.dpi).unwrap_or(fs)
        } else {
            fs * 0.833
        };
        // 정상 한컴 Save/Print의 숫자·변수·분수·중첩 근호 70건에서
        // 기호의 논리 em과 별도로 0.17em의 끝 여백을 예약한다.
        // 세로 여백은 같은 가로 비율로 덮어쓰지 않는다.
        let right_pad = if self.hancom_math_face {
            fs * 0.17
        } else {
            pad
        };
        let body_w = b.width + right_pad;
        let body_h = b.height + pad * 2.0;

        let idx = index.as_ref().map(|i| {
            let mut ib = self.layout_node(i, fs * RADICAL_DEGREE_SCALE);
            // MATH RadicalKernBeforeDegree=278/1000em.
            ib.x = fs * 0.278;
            // MATH RadicalDegreeBottomRaisePercent=60: 근 지수 하단을
            // radical 높이의 60%만큼 올린다.
            ib.y = body_h * 0.4 - ib.height;
            ib
        });
        // MATH RadicalKernAfterDegree=-556/1000em은 근 지수와 기호의
        // 겹쳐 쓸 수 있는 폭이다. 그 폭을 넘는 지수만 추가 공간을 점유한다.
        let degree_extension = idx
            .as_ref()
            .map(|i| (i.x + i.width - fs * 0.556).max(0.0))
            .unwrap_or(0.0);
        let total_w = degree_extension + sign_w + body_w;

        let mut body_box = b;
        body_box.x = total_w - body_w;
        body_box.y = pad;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_w,
            height: body_h,
            baseline: body_box.y + body_box.baseline,
            kind: LayoutKind::Sqrt {
                index: idx.map(Box::new),
                body: Box::new(body_box),
            },
        }
    }
}

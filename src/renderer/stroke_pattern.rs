//! 도형의 한컴 Print 반복 비율을 선언 선 굵기로 변환한다.

use super::StrokeDash;

impl StrokeDash {
    pub(crate) fn is_hwp_pattern(self) -> bool {
        matches!(
            self,
            Self::HwpDash
                | Self::HwpDot
                | Self::HwpDashDot
                | Self::HwpDashDotDot
                | Self::HwpLongDash
                | Self::HwpCircle
        )
    }

    pub(crate) fn uses_round_cap(self) -> bool {
        self == Self::HwpCircle
    }

    /// 기존 범용 선은 종전 구간을 유지한다. 한컴 도형은 선 굵기의 배수다.
    pub(crate) fn intervals(self, width: f64) -> Vec<f64> {
        let values: &[f64] = match self {
            Self::Solid => &[],
            Self::Dash => &[6.0, 3.0],
            Self::Dot => &[2.0, 2.0],
            Self::DashDot => &[6.0, 3.0, 2.0, 3.0],
            Self::DashDotDot => &[6.0, 3.0, 2.0, 3.0, 2.0, 3.0],
            Self::HwpDash => &[7.5, 4.5],
            Self::HwpDot => &[1.5, 2.25],
            Self::HwpDashDot => &[15.0, 4.5, 1.5, 4.5],
            Self::HwpDashDotDot => &[15.0, 4.5, 1.5, 4.5, 1.5, 4.5],
            Self::HwpLongDash => &[15.0, 4.5],
            // 길이 0인 round cap의 지름은 선 굵기다.
            Self::HwpCircle => &[0.0, 2.25],
        };
        let scale = if self.is_hwp_pattern() { width } else { 1.0 };
        values.iter().map(|value| value * scale).collect()
    }

    pub(crate) fn canvas_width(self, width: f64) -> f64 {
        if self.is_hwp_pattern() && width > 0.0 {
            width
        } else {
            width.max(0.5)
        }
    }
}

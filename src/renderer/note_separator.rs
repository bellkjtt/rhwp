//! 미주 구분선의 중심 경로와 위·아래 여백에서 정해지는 공통 흐름 높이.

use super::{hwpunit_to_px, layout::border_width_to_px};

pub(crate) struct NoteSeparatorGeometry {
    pub(crate) center_offset: f64,
    pub(crate) width: f64,
    pub(crate) advance: f64,
}

impl NoteSeparatorGeometry {
    pub(crate) fn new(above_hu: i32, below_hu: i32, visible: bool, width: u8, dpi: f64) -> Self {
        let width = if visible {
            border_width_to_px(width).max(0.5)
        } else {
            0.0
        };
        let center_offset = hwpunit_to_px(above_hu, dpi);
        // upstream의 독립 한컴 출력 계약: 선 굵기는 흐름 높이에 더하지 않는다.
        // 측정과 실제 배치는 같은 위·아래 여백의 합을 소비한다.
        let advance = center_offset + hwpunit_to_px(below_hu, dpi);
        Self {
            center_offset,
            width,
            advance,
        }
    }
}

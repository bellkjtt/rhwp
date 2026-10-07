//! 위첨자의 기준선과 하단 여유를 같은 배치 결과로 계산한다.

use super::{EqLayout, EqNode, LayoutBox, LayoutKind};

// 배포 Latin Modern Math의 MATH 상수(UPM 1,000). 문서 좌표가 아닌 em 비율이다.
const SUPERSCRIPT_SHIFT_UP: f64 = 0.363;
const SUPERSCRIPT_BOTTOM_MIN: f64 = 0.108;

impl EqLayout {
    pub(super) fn layout_superscript(&self, base: &EqNode, sup: &EqNode, fs: f64) -> LayoutBox {
        let mut base_box = self.layout_node(base, fs);
        let mut sup_box = self.layout_node(sup, self.script_font_size(fs));

        // 위첨자의 깊이를 포함해 본체 기준선 위에 하단 여유를 확보한다.
        // 키 큰 본체는 원래 기준선을 유지하므로 위첨자를 맞추려고 본체를 올리지 않는다.
        let sup_depth = sup_box.height - sup_box.baseline;
        let shift = (fs * SUPERSCRIPT_SHIFT_UP).max(sup_depth + fs * SUPERSCRIPT_BOTTOM_MIN);
        let baseline = base_box.baseline.max(sup_box.baseline + shift);
        base_box.x = 0.0;
        base_box.y = baseline - base_box.baseline;
        sup_box.x = base_box.width;
        sup_box.y = baseline - shift - sup_box.baseline;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: base_box.width + sup_box.width,
            height: (base_box.y + base_box.height).max(sup_box.y + sup_box.height),
            baseline,
            kind: LayoutKind::Superscript {
                base: Box::new(base_box),
                sup: Box::new(sup_box),
            },
        }
    }
}

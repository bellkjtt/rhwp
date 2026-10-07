//! 위첨자의 기준선과 하단 여유를 같은 배치 결과로 계산한다.

use super::{is_integral_symbol, EqLayout, EqNode, LayoutBox, LayoutKind};

// 배포 Latin Modern Math의 MATH 상수(UPM 1,000). 문서 좌표가 아닌 em 비율이다.
const SUPERSCRIPT_SHIFT_UP: f64 = 0.363;
const SUPERSCRIPT_BOTTOM_MIN: f64 = 0.108;

impl EqLayout {
    pub(super) fn layout_superscript(&self, base: &EqNode, sup: &EqNode, fs: f64) -> LayoutBox {
        if matches!(base, EqNode::MathSymbol(symbol) if is_integral_symbol(symbol)) {
            return self
                .integral_placement(base, None, Some(sup), fs)
                .into_script(fs);
        }
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
    pub(super) fn layout_subscript(&self, base: &EqNode, sub: &EqNode, fs: f64) -> LayoutBox {
        if matches!(base, EqNode::MathSymbol(symbol) if is_integral_symbol(symbol)) {
            return self
                .integral_placement(base, Some(sub), None, fs)
                .into_script(fs);
        }
        let b = self.layout_node(base, fs);
        let s = self.layout_node(sub, self.script_font_size(fs));

        let sub_shift = b.baseline * 0.4;
        let total_h = (b.height).max(sub_shift + s.height);

        let mut base_box = b;
        base_box.x = 0.0;
        base_box.y = 0.0;

        let mut sub_box = s;
        sub_box.x = base_box.width;
        sub_box.y = sub_shift;

        let total_w = base_box.width + sub_box.width;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_w,
            height: total_h,
            baseline: base_box.baseline,
            kind: LayoutKind::Subscript {
                base: Box::new(base_box),
                sub: Box::new(sub_box),
            },
        }
    }

    pub(super) fn layout_subsup(
        &self,
        base: &EqNode,
        sub: &EqNode,
        sup: &EqNode,
        fs: f64,
    ) -> LayoutBox {
        if matches!(base, EqNode::MathSymbol(symbol) if is_integral_symbol(symbol)) {
            return self
                .integral_placement(base, Some(sub), Some(sup), fs)
                .into_script(fs);
        }
        let b = self.layout_node(base, fs);
        let sb = self.layout_node(sub, self.script_font_size(fs));
        let sp = self.layout_node(sup, self.script_font_size(fs));

        let sup_shift = b.baseline - sp.height * 0.7;
        let sub_shift = b.baseline * 0.4;

        let ascent = if sup_shift < 0.0 {
            sp.height - sup_shift.abs()
        } else {
            sp.height.max(0.0)
        };
        let top = sup_shift.min(0.0).abs();
        let total_h = (top + b.height)
            .max(top + sub_shift + sb.height)
            .max(ascent + b.height);

        let base_y = top.max(
            if sup_shift > 0.0 {
                0.0
            } else {
                sp.height - sup_shift.abs() - b.baseline
            }
            .max(0.0),
        );

        let mut base_box = b;
        base_box.x = 0.0;
        base_box.y = base_y;

        let mut sup_box = sp;
        sup_box.x = base_box.width;
        sup_box.y = 0.0;

        let mut sub_box = sb;
        sub_box.x = base_box.width;
        sub_box.y = base_y + sub_shift;

        let script_w = sup_box.width.max(sub_box.width);
        let total_w = base_box.width + script_w;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_w,
            height: total_h
                .max(base_box.y + base_box.height)
                .max(sub_box.y + sub_box.height),
            baseline: base_box.y + base_box.baseline,
            kind: LayoutKind::SubSup {
                base: Box::new(base_box),
                sub: Box::new(sub_box),
                sup: Box::new(sup_box),
            },
        }
    }
}

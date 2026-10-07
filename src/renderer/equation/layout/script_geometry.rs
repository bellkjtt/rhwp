//! 위첨자의 기준선과 하단 여유를 같은 배치 결과로 계산한다.

use super::{
    integral_geom, is_integral_symbol, EqLayout, EqNode, LayoutBox, LayoutKind, BIG_OP_TRAIL_PAD,
};

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
    pub(super) fn layout_subscript(&self, base: &EqNode, sub: &EqNode, fs: f64) -> LayoutBox {
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
        // 적분 기호: 상한은 기호 상단, 하한은 기호 하단에 맞춤
        let is_integral = matches!(base, EqNode::MathSymbol(s) if is_integral_symbol(s));

        let b = self.layout_node(base, fs);
        let sb = self.layout_node(sub, self.script_font_size(fs));
        let sp = self.layout_node(sup, self.script_font_size(fs));

        if is_integral {
            // 적분 전용 배치 (Task #1317): 글리프를 stroke path 로 그리므로 상·하한 attach
            // point 를 path 기하(`integral_geom`)에 맞춰 산출한다. SVG/Canvas/Skia 가 동일
            // geom 을 공유하여 폰트 대체에 무관하게 정합한다(정답 한글 2022 비례).
            //   - 상한(sup): 상단 갈고리 우측, 글리프 최상단 근처
            //   - 하한(sub): 하단 갈고리 우측, 글리프 최하단(=baseline 근처)
            let geom = integral_geom(fs);
            // 상·하한이 적분 줄기에 붙지 않도록 **가로 간격(gap_x)만** 키워 띄운다.
            // 세로로 벌리면 적분 박스 높이가 커져 줄 간격이 위/아래로 넓어지므로
            // 세로 위치는 컴팩트하게 유지한다 (작업지시자 피드백, Task #1317 v4).
            let gap_x = fs * 0.22;

            let mut base_box = b;
            base_box.x = 0.0;
            base_box.width = geom.width; // 글리프 advance = path 폭

            // 글리프 기준 첨자 세로 위치(박스 상단 원점) — 컴팩트(글리프 상·하단 근처).
            let sup_dy = geom.top_y - sp.height * 0.30; // 상한: 최상단 근처
            let sub_dy = geom.bottom_y - sb.height * 0.72; // 하한: 최하단 근처

            // 상한이 박스 위로 넘치지 않도록 글리프를 그만큼 아래로 내린다.
            let head = (-sup_dy).max(0.0);
            base_box.y = head;

            let mut sup_box = sp;
            sup_box.x = base_box.x + geom.top_hook_x + gap_x;
            sup_box.y = base_box.y + sup_dy;

            let mut sub_box = sb;
            sub_box.x = base_box.x + geom.bottom_hook_x + gap_x;
            sub_box.y = base_box.y + sub_dy;

            let right = (sup_box.x + sup_box.width)
                .max(sub_box.x + sub_box.width)
                .max(base_box.x + geom.width);
            let total_w = right + fs * BIG_OP_TRAIL_PAD;
            let total_h = (sub_box.y + sub_box.height)
                .max(base_box.y + base_box.height)
                .max(sup_box.y + sup_box.height);

            return LayoutBox {
                font_size: fs,
                x: 0.0,
                y: 0.0,
                width: total_w,
                height: total_h,
                baseline: base_box.y + base_box.baseline,
                kind: LayoutKind::SubSup {
                    base: Box::new(base_box),
                    sub: Box::new(sub_box),
                    sup: Box::new(sup_box),
                },
            };
        }

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

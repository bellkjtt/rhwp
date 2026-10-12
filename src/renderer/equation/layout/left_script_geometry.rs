//! 왼쪽 첨자는 본체·첨자의 공통 세로 메트릭을 소비하고 가로 소속만 바꾼다.

use super::{EqLayout, EqNode, LayoutBox, LayoutKind};

impl EqLayout {
    pub(super) fn layout_left_scripts(
        &self,
        base: &EqNode,
        sub: Option<&EqNode>,
        sup: Option<&EqNode>,
        fs: f64,
    ) -> LayoutBox {
        let mut body = self.layout_node(base, fs);
        let mut lower = sub.map(|node| self.layout_node(node, self.script_font_size(fs)));
        let mut upper = sup.map(|node| self.layout_node(node, self.script_font_size(fs)));
        let upper_y = upper.as_ref().map_or(0.0, |script| {
            body.baseline - self.script_superscript_shift(&body, script, fs) - script.baseline
        });
        let (lower_y, lower_height) = lower.as_ref().map_or((0.0, body.height), |script| {
            self.script_subscript_geometry(&body, script, fs)
        });
        let body_y = (-upper_y).max(-lower_y).max(0.0);
        let width = lower
            .as_ref()
            .into_iter()
            .chain(upper.as_ref())
            .map(|script| script.width)
            .fold(0.0, f64::max);
        body.x = width;
        body.y = body_y;
        let total_width = width + body.width;
        let baseline = body_y + body.baseline;
        let mut height = (body_y + lower_height).max(body_y + body.height);
        let mut children = Vec::with_capacity(3);
        if let Some(script) = &mut lower {
            script.x = width - script.width;
            script.y = body_y + lower_y;
            height = height.max(script.y + script.height);
        }
        if let Some(script) = &mut upper {
            script.x = width - script.width;
            script.y = body_y + upper_y;
            height = height.max(script.y + script.height);
        }
        children.extend(lower);
        children.extend(upper);
        children.push(body);
        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_width,
            height,
            baseline,
            kind: LayoutKind::Row(children),
        }
    }
}

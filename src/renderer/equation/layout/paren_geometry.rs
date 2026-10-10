//! 괄호 슬롯·본체 원점·논리 프레임을 한 배치에서 결정한다.
use super::{
    delimiter_advance, source_font_advance, source_font_text, EqLayout, EqNode, LayoutBox,
    LayoutKind, PAREN_PAD,
};
impl EqLayout {
    pub(super) fn layout_paren(
        &self,
        left: &str,
        right: &str,
        body: &EqNode,
        fs: f64,
    ) -> LayoutBox {
        let b = self.layout_node(body, fs);
        let logical_frame = self
            .hancom_math_face
            .then(|| super::super::logical_font_frame::metrics_at_dpi(&b, fs, self.dpi))
            .flatten();
        let delimiter_height = logical_frame.map_or(b.height, |(height, _)| height);
        let glyph_delimiters = delimiter_height <= fs * 1.2;
        let source_stretch = (!glyph_delimiters && logical_frame.is_some())
            .then(|| delimiter_advance::source_stretch(left, right, fs, self.dpi))
            .flatten();
        let use_stretch_round = !glyph_delimiters && matches!((left, right), ("(", ")"));
        let source_curly = self.hancom_math_face
            && matches!(left, "" | "{" | "}")
            && matches!(right, "" | "{" | "}");
        let source_round = self.hancom_math_face
            && !use_stretch_round
            && matches!(left, "" | "(" | ")")
            && matches!(right, "" | "(" | ")");
        let pad = if source_round || source_curly || source_stretch.is_some() {
            0.0
        } else if use_stretch_round {
            fs * 0.03
        } else {
            fs * PAREN_PAD
        };
        // Times New Roman '(' advance (em 기준) = 0.333. 텍스트 높이 glyph는 이 폭을 유지하고,
        // 큰 둥근 괄호 path는 한컴 HyhwpEQ 출력에 가깝게 더 좁게 잡는다. (Task #283, #1139)
        let paren_w = if source_curly {
            source_font_text::logical_advance(512, fs, self.dpi)
        } else if source_round {
            source_font_advance::paren(fs, self.dpi)
        } else if use_stretch_round {
            fs * 0.27
        } else {
            fs * 0.333
        };

        let (left_w, right_w) = source_stretch.unwrap_or((
            if left.is_empty() { 0.0 } else { paren_w },
            if right.is_empty() { 0.0 } else { paren_w },
        ));

        let mut body_box = b;
        body_box.x = left_w + pad;
        body_box.y = 0.0;

        let total_w = left_w + pad + body_box.width + pad + right_w;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_w,
            height: body_box.height,
            baseline: body_box.baseline,
            kind: LayoutKind::Paren {
                left: left.to_string(),
                right: right.to_string(),
                left_width: left_w,
                right_width: right_w,
                delimiter_height,
                glyph_delimiters,
                body: Box::new(body_box),
            },
        }
    }
}

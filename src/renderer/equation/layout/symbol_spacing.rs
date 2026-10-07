//! 수식 글줄의 끝과 인접 연산자 경계에서 자동 여백을 중복 예약하지 않는다.
//! 명시적인 Space는 별도 요소로 보존한다. 실제 glyph와 양쪽 여백을 같은 Row로 전달한다.

use super::{estimate_text_width, EqLayout, EqNode, LayoutBox, LayoutKind};

fn operator(node: &EqNode) -> bool {
    matches!(node, EqNode::Symbol(s) | EqNode::MathSymbol(s) if super::super::hancom_glyphs::is_operator(s) || matches!(s.as_str(), "+" | "-" | "=" | "<" | ">" | "×" | "÷"))
}

impl EqLayout {
    pub(super) fn row_children(&self, nodes: &[EqNode], fs: f64) -> Vec<LayoutBox> {
        let significant: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| !matches!(node, EqNode::Space(_) | EqNode::Empty))
            .map(|(index, _)| index)
            .collect();
        nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let mut result = self.layout_node(node, fs);
                if !self.hancom_math_face || !operator(node) {
                    return result;
                }
                let position = significant
                    .iter()
                    .position(|value| *value == index)
                    .expect("연산자는 유효한 글줄 요소다");
                let left =
                    if position == 0 || operator(nodes.get(significant[position - 1]).unwrap()) {
                        0.0
                    } else {
                        fs * 0.15
                    };
                let right = if position + 1 == significant.len()
                    || operator(&nodes[significant[position + 1]])
                {
                    0.0
                } else {
                    fs * 0.15
                };
                if let LayoutKind::HancomGlyphs(run) = &mut result.kind {
                    for position in &mut run.positions {
                        *position += left;
                    }
                    run.width += left + right;
                    result.width = run.width;
                    return result;
                }
                let LayoutKind::Symbol(text) = &result.kind else {
                    return result;
                };
                let glyph_width = estimate_text_width(text, fs, false);
                result.x = left;
                result.width = glyph_width;
                LayoutBox {
                    font_size: fs,
                    x: 0.0,
                    y: 0.0,
                    width: left + glyph_width + right,
                    height: result.height,
                    baseline: result.baseline,
                    kind: LayoutKind::Row(vec![result]),
                }
            })
            .collect()
    }
}

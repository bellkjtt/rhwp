//! 수식 글줄의 끝과 인접 연산자 경계에서 자동 여백을 중복 예약하지 않는다.
//! 명시적인 Space는 별도 요소로 보존한다. 실제 glyph와 양쪽 여백을 같은 Row로 전달한다.

use super::{estimate_text_width, source_font_symbol, EqLayout, EqNode, LayoutBox, LayoutKind};

fn operator(node: &EqNode) -> bool {
    matches!(node, EqNode::Symbol(s) | EqNode::MathSymbol(s) if super::super::hancom_glyphs::is_operator(s) || matches!(s.as_str(), "+" | "-" | "=" | "<" | ">" | "×" | "÷"))
}

/// 분수의 내부 여백·닫는 괄호의 점유 뒤에 연산자 앞 여백을 중복 예약하지 않는다.
fn reserves_operator_gap(node: &EqNode) -> bool {
    match node {
        EqNode::Fraction { .. } => true,
        EqNode::Paren { right, .. } => !right.is_empty(),
        EqNode::FontStyle { body, .. } => reserves_operator_gap(body),
        EqNode::Row(items) => items.last().is_some_and(reserves_operator_gap),
        _ => false,
    }
}

impl EqLayout {
    pub(super) fn row_children(&self, nodes: &[EqNode], fs: f64) -> Vec<LayoutBox> {
        let significant: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| !matches!(node, EqNode::Space(_) | EqNode::Empty))
            .map(|(index, _)| index)
            .collect();
        let mut next_position = 0;
        nodes
            .iter()
            .map(|node| {
                let mut result = self.layout_node(node, fs);
                if matches!(node, EqNode::Space(_) | EqNode::Empty) {
                    return result;
                }
                let position = next_position;
                next_position += 1;
                if !self.hancom_math_face || !operator(node) {
                    return result;
                }
                let text = match &result.kind {
                    LayoutKind::Symbol(text) => text.as_str(),
                    LayoutKind::HancomGlyphs(_) => match node {
                        EqNode::Symbol(text) | EqNode::MathSymbol(text) => text.as_str(),
                        _ => return result,
                    },
                    _ => return result,
                };
                let source_metrics = if matches!(result.kind, LayoutKind::HancomGlyphs(_)) {
                    None
                } else {
                    source_font_symbol::operator_metrics(text, fs, self.dpi)
                };
                let padding = source_metrics.map_or(fs * 0.15, |(_, padding)| padding);
                let left = if position == 0
                    || operator(&nodes[significant[position - 1]])
                    || (source_metrics.is_some()
                        && reserves_operator_gap(&nodes[significant[position - 1]]))
                {
                    0.0
                } else {
                    padding
                };
                let right = if position + 1 == significant.len()
                    || operator(&nodes[significant[position + 1]])
                {
                    0.0
                } else {
                    padding
                };
                let glyph_width = source_metrics
                    .map_or_else(|| estimate_text_width(text, fs, false), |(width, _)| width);
                if let LayoutKind::HancomGlyphs(run) = &mut result.kind {
                    for position in &mut run.positions {
                        *position += left;
                    }
                    run.width += left + right;
                    result.width = run.width;
                    return result;
                }
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

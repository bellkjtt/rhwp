//! 수식 글줄의 끝과 인접 연산자 경계에서 자동 여백을 중복 예약하지 않는다.
//! 명시적인 Space는 별도 요소로 보존한다. 실제 glyph와 양쪽 여백을 같은 Row로 전달한다.

use super::{estimate_text_width, source_font_symbol, EqLayout, EqNode, LayoutBox, LayoutKind};

fn operator(node: &EqNode) -> bool {
    matches!(node, EqNode::Symbol(s) | EqNode::MathSymbol(s)
        if super::super::hancom_glyphs::is_operator(s) || super::super::symbols::is_binary_symbol(s)
            || matches!(s.as_str(), "=" | "≠" | "<" | ">"))
}

/// 분수·근호의 끝 여백·닫는 괄호의 점유 뒤에 연산자 앞 여백을 중복 예약하지 않는다.
fn reserves_operator_gap(node: &EqNode) -> bool {
    match node {
        EqNode::Fraction { .. } | EqNode::Sqrt { .. } => true,
        EqNode::Paren { right, .. } => !right.is_empty(),
        EqNode::FontStyle { body, .. } => reserves_operator_gap(body),
        EqNode::Row(items) => items.last().is_some_and(reserves_operator_gap),
        _ => false,
    }
}

fn ends_with_script(node: &EqNode) -> bool {
    match node {
        EqNode::Superscript { .. } | EqNode::Subscript { .. } => true,
        EqNode::FontStyle { body, .. } => ends_with_script(body),
        EqNode::Row(items) => items.last().is_some_and(ends_with_script),
        _ => false,
    }
}

/// 인접한 닫는 괄호는 하나의 구분자 묶음이다. operand 끝만 자동 간격을 만든다.
fn operand_end(node: &EqNode) -> bool {
    match node {
        EqNode::Text(_) | EqNode::Number(_) | EqNode::Function(_) => true,
        EqNode::Symbol(text) | EqNode::MathSymbol(text) => {
            matches!(text.as_str(), "'" | "′")
                || (text.chars().count() == 1
                    && text
                        .chars()
                        .all(|ch| matches!(ch, '\u{0391}'..='\u{03A9}' | '\u{03B1}'..='\u{03C9}')))
        }
        EqNode::Row(items) => items
            .iter()
            .rfind(|item| !matches!(item, EqNode::Space(_) | EqNode::Empty))
            .is_some_and(operand_end),
        EqNode::FontStyle { body, .. } | EqNode::Color { body, .. } => operand_end(body),
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
                if self.hancom_math_face
                    && position > 0
                    && matches!(node, EqNode::Symbol(text) if matches!(text.as_str(), "(" | ")"))
                    && operand_end(&nodes[significant[position - 1]])
                {
                    let left = fs * 0.07;
                    let width = result.width + left;
                    result.x = left;
                    return LayoutBox {
                        font_size: fs,
                        x: 0.0,
                        y: 0.0,
                        width,
                        height: result.height,
                        baseline: result.baseline,
                        kind: LayoutKind::Row(vec![result]),
                    };
                }
                if !self.hancom_math_face || !operator(node) {
                    return result;
                }
                let text = match &result.kind {
                    LayoutKind::Symbol(text) => text.as_str(),
                    LayoutKind::FontGlyph { text, outline: Some(_), .. } => text.as_str(),
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
                    if source_metrics.is_some()
                        && ends_with_script(&nodes[significant[position - 1]])
                    {
                        padding.max(fs * 0.20)
                    } else {
                        padding
                    }
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

//! 수식 글줄의 끝과 인접 연산자 경계에서 자동 여백을 중복 예약하지 않는다.
//! 명시적인 Space는 별도 요소로 보존한다. 실제 glyph와 양쪽 여백을 같은 Row로 전달한다.

use super::{estimate_text_width, source_font_symbol, EqLayout, EqNode, LayoutBox, LayoutKind};

fn operator(node: &EqNode) -> bool {
    matches!(node, EqNode::Symbol(s) | EqNode::MathSymbol(s)
        if super::super::hancom_glyphs::is_operator(s) || super::super::symbols::is_binary_symbol(s)
            || matches!(s.as_str(), "=" | "≠" | "<" | ">" | "≤" | "≥"))
}

/// 명시적인 공백은 자동 연산자 간격을 지우는 인접 경계가 아니다.
/// 한컴 Print의 `x<`-2`와 `x=-2`는 이 경계에서 서로 다른 전진을 갖는다.
fn explicit_gap(nodes: &[EqNode], left: usize, right: usize) -> bool {
    nodes[left + 1..right]
        .iter()
        .any(|node| matches!(node, EqNode::Space(_)))
}

fn opening_delimiter(node: &EqNode) -> bool {
    match node {
        EqNode::Symbol(text) => matches!(text.as_str(), "(" | "[" | "{"),
        EqNode::Decoration { body, .. } | EqNode::FontStyle { body, .. } => opening_delimiter(body),
        _ => false,
    }
}

/// 분수·근호의 끝 여백·닫는 괄호의 점유 뒤에 연산자 앞 여백을 중복 예약하지 않는다.
fn reserves_operator_gap(node: &EqNode) -> bool {
    match node {
        EqNode::Fraction { .. } | EqNode::Sqrt { .. } => true,
        // 독립 Print의 벡터 합·괄호식은 장식 점유 뒤 앞 연산자 여백을 다시 더하지 않는다.
        // 장식 본체의 좌우 여유와 연산자 뒤 여백은 각각 별도로 보존한다.
        EqNode::Decoration {
            kind:
                super::super::symbols::DecoKind::Bar
                | super::super::symbols::DecoKind::Vec
                | super::super::symbols::DecoKind::Dyad,
            ..
        } => true,
        EqNode::Paren { right, .. } => !right.is_empty(),
        EqNode::FontStyle { body, .. } | EqNode::Color { body, .. } => reserves_operator_gap(body),
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

fn boundary_element(node: &EqNode, first: bool) -> &EqNode {
    match node {
        EqNode::Row(items) => {
            let mut significant = items
                .iter()
                .filter(|item| !matches!(item, EqNode::Space(_) | EqNode::Empty));
            let edge = if first {
                significant.next()
            } else {
                significant.next_back()
            };
            edge.map_or(node, |item| boundary_element(item, first))
        }
        EqNode::FontStyle { body, .. } | EqNode::Color { body, .. } => {
            boundary_element(body, first)
        }
        _ => node,
    }
}

fn before_operator_gap(
    previous: &EqNode,
    adjacent: bool,
    source: bool,
    padding: f64,
    fs: f64,
) -> f64 {
    if (operator(previous) && adjacent)
        || opening_delimiter(previous)
        || (source && reserves_operator_gap(previous))
    {
        0.0
    } else if source && ends_with_script(previous) {
        padding.max(fs * 0.2)
    } else {
        padding
    }
}

impl EqLayout {
    /// & 칸 경계의 여유는 같은 행 내부의 연산자 간격과 구분한다.
    /// 분수·근호 뒤의 일반 행 여유를 억제하는 조건으로 열 사이 간격을 지우지 않는다.
    pub(super) fn alignment_boundary_gap(&self, right: &EqNode, fs: f64) -> f64 {
        let next = boundary_element(right, true);
        if !operator(next) {
            return 0.0;
        }
        let text = match next {
            EqNode::Symbol(text) | EqNode::MathSymbol(text) => text,
            _ => return 0.0,
        };
        let source = source_font_symbol::operator_metrics(text, fs, self.dpi);
        source.map_or(fs * 0.15, |(_, pad)| pad)
    }

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
                    LayoutKind::FontGlyph { text, .. } => text.as_str(),
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
                let left = if position == 0 {
                    0.0
                } else {
                    before_operator_gap(
                        &nodes[significant[position - 1]],
                        !explicit_gap(nodes, significant[position - 1], significant[position]),
                        source_metrics.is_some(),
                        padding,
                        fs,
                    )
                };
                let right = if position + 1 == significant.len()
                    || (operator(&nodes[significant[position + 1]])
                        && !explicit_gap(nodes, significant[position], significant[position + 1]))
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

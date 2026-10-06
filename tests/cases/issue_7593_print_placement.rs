//! #7593 공개 원본과 독립 MCP2020 Print의 배치 관계를 보존한다.
//! Native/fresh WASM 전쪽 최저92.28714% 확인 후 추가했다.
//! 실물 문서의 절대 픽셀이나 SVG 해시 대신 소유·기준선·탭 격자를 검사한다.
#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::model::control::Control;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};
use rhwp::renderer::{hwpunit_to_px, DEFAULT_DPI};

const ORIGINAL: &[u8] = include_bytes!("../../samples/issue7593/repro-equation-issue7593.hwpx");
const MINIMUM: &[u8] = include_bytes!("../../samples/issue7593/repro-dyad-min-no-script.hwpx");

fn descendants<'a>(node: &'a RenderNode, out: &mut Vec<&'a RenderNode>) {
    out.push(node);
    for child in &node.children {
        descendants(child, out);
    }
}

#[test]
fn master_separator_owns_the_saved_frame_without_advancing_body() {
    let core = DocumentCore::from_bytes(ORIGINAL).unwrap();
    assert_eq!(core.page_count(), 1, "Print 전체 쪽수와 배경의 흐름 중립성");
    let tree = core.build_page_render_tree(0).unwrap();
    let mut nodes = Vec::new();
    descendants(&tree.root, &mut nodes);
    let master = nodes
        .iter()
        .find(|n| matches!(n.node_type, RenderNodeType::MasterPage))
        .unwrap();
    let body = nodes
        .iter()
        .find(|n| matches!(n.node_type, RenderNodeType::Body { .. }))
        .unwrap();
    let mut background = Vec::new();
    descendants(master, &mut background);
    let lines: Vec<_> = background
        .iter()
        .filter_map(|n| match &n.node_type {
            RenderNodeType::Line(line) => Some(line),
            _ => None,
        })
        .collect();
    assert_eq!(lines.len(), 1, "Print의 바탕쪽 단 구분선 한 개");
    let line = lines[0];
    let saved = &core.document().sections[0].section_def.master_pages[0];
    let pi = line.para_index.expect("바탕쪽 문단 소유");
    let ci = line.control_index.expect("바탕쪽 colPr 소유");
    let Control::ColumnDef(definition) = &saved.paragraphs[pi].controls[ci] else {
        panic!("바탕쪽 단 정의의 선이어야 한다");
    };
    assert_eq!(definition.column_count, 2);
    assert_eq!(line.section_index, Some(0));
    let width = hwpunit_to_px(saved.text_width as i32, DEFAULT_DPI);
    let height = hwpunit_to_px(saved.text_height as i32, DEFAULT_DPI);
    // Print에서 확인한 대칭 두 단의 가운데와 저장 프레임 전체 높이.
    let center_ratio = (line.x1 - body.bbox.x) / width;
    assert!((center_ratio - 0.5).abs() <= f64::EPSILON * 128.0);
    assert_eq!(line.x1, line.x2);
    assert!(((line.y2 - line.y1) / height - 1.0).abs() <= f64::EPSILON * 128.0);
    let table = nodes
        .iter()
        .find(|n| matches!(n.node_type, RenderNodeType::Table(_)))
        .expect("보기 표 보존");
    assert!(line.y1 <= table.bbox.y);
    assert!(line.y2 > table.bbox.y + table.bbox.height);
    assert_eq!(
        nodes
            .iter()
            .filter(|n| matches!(n.node_type, RenderNodeType::Equation(_)))
            .count(),
        7
    );

    let minimum = DocumentCore::from_bytes(MINIMUM).unwrap();
    let minimum_tree = minimum.build_page_render_tree(0).unwrap();
    let mut minimum_nodes = Vec::new();
    descendants(&minimum_tree.root, &mut minimum_nodes);
    for master in minimum_nodes
        .iter()
        .filter(|n| matches!(n.node_type, RenderNodeType::MasterPage))
    {
        let mut background = Vec::new();
        descendants(master, &mut background);
        assert!(
            !background
                .iter()
                .any(|n| matches!(n.node_type, RenderNodeType::Line(_))),
            "구분선 없는 최소 대조군에 선을 만들지 않는다"
        );
    }
    assert_eq!(minimum.page_count(), 1);
}

#[test]
fn tall_cell_equation_keeps_print_baseline_order_and_inline_ownership() {
    let core = DocumentCore::from_bytes(ORIGINAL).unwrap();
    let tree = core.build_page_render_tree(0).unwrap();
    let mut nodes = Vec::new();
    descendants(&tree.root, &mut nodes);
    let line = nodes.iter().find(|n| n.children.iter().any(|child| {
        matches!(&child.node_type, RenderNodeType::Equation(eq) if eq.script == "dyad{rm AB}" && eq.cell_index.is_some())
    })).expect("보기 ㄴ의 실제 인라인 줄");
    let (equation_node, equation) = line
        .children
        .iter()
        .find_map(|n| match &n.node_type {
            RenderNodeType::Equation(eq) if eq.script == "dyad{rm AB}" => Some((n, eq)),
            _ => None,
        })
        .unwrap();
    let (text_node, text) = line
        .children
        .iter()
        .find_map(|n| match &n.node_type {
            RenderNodeType::TextRun(run) if run.text.contains("이다") => Some((n, run)),
            _ => None,
        })
        .expect("수식 뒤 글자 보존");
    let context = text.cell_context.as_ref().expect("뒤 글자의 셀 소유");
    let cell = context.path.first().unwrap();
    assert_eq!(equation.para_index, Some(context.parent_para_index));
    assert_eq!(equation.control_index, Some(cell.control_index));
    assert_eq!(equation.cell_index, Some(cell.cell_index));
    assert_eq!(equation.cell_para_index, Some(cell.cell_para_index));
    assert_eq!(text.para_index, Some(cell.cell_para_index));
    assert!(equation_node.bbox.x + equation_node.bbox.width <= text_node.bbox.x);
    let text_baseline = text_node.bbox.y + text.baseline;
    let visible_baseline = equation_node.bbox.y + equation.layout_box.baseline;
    assert!(
        visible_baseline < text_baseline,
        "Print의 ㄴ 수식은 앞뒤 본문보다 위에 표시된다"
    );
    let Control::Table(table) = &core.document().sections[0].paragraphs
        [equation.para_index.unwrap()]
    .controls[equation.control_index.unwrap()] else {
        panic!("셀의 표 소유");
    };
    let paragraph =
        &table.cells[equation.cell_index.unwrap()].paragraphs[equation.cell_para_index.unwrap()];
    let saved = paragraph
        .controls
        .iter()
        .find_map(|c| match c {
            Control::Equation(eq) if eq.script == equation.script => Some(eq),
            _ => None,
        })
        .unwrap();
    let anchor_ratio = (text_baseline - equation_node.bbox.y) / equation_node.bbox.height;
    assert!(
        (anchor_ratio - f64::from(saved.baseline) / 100.0).abs() <= f64::EPSILON * 128.0,
        "저장 기준선 거리와 실제 줄 원점이 일치한다"
    );
}

#[test]
fn no_stored_rows_reflow_choice_equations_on_the_paragraph_tab_grid() {
    let core = DocumentCore::from_bytes(ORIGINAL).unwrap();
    let tree = core.build_page_render_tree(0).unwrap();
    let mut nodes = Vec::new();
    descendants(&tree.root, &mut nodes);
    let choice_line = nodes.iter().find(|n| n.children.iter().any(|child| {
        matches!(&child.node_type, RenderNodeType::Equation(eq) if eq.script == "3" && eq.cell_index.is_none())
    })).unwrap();
    let equations: Vec<_> = choice_line
        .children
        .iter()
        .filter_map(|n| match &n.node_type {
            RenderNodeType::Equation(eq) => Some((n, eq)),
            _ => None,
        })
        .collect();
    assert_eq!(equations.len(), 3, "①/②/③ 수식의 순서와 완전성");
    let pi = equations[0].1.para_index.unwrap();
    let paragraph = &core.document().sections[0].paragraphs[pi];
    assert!(
        paragraph.line_segs.is_empty()
            || paragraph
                .line_segs
                .iter()
                .all(|seg| seg.tag & 0x8000_0000 != 0)
    );
    let tab_step = hwpunit_to_px(i32::from(paragraph.tab_extended[0][0]), DEFAULT_DPI);
    // 독립 Print의 세 선택지는 기본 격자의 세 칸 간격이다. 앞 수식의 폭이
    // 다음 탭 위치에 다시 더해지지 않으며 최종 수식도 같은 단 안에 남는다.
    for pair in equations.windows(2) {
        let ratio = (pair[1].0.bbox.x - pair[0].0.bbox.x) / tab_step;
        assert!(
            (ratio - 3.0).abs() <= f64::EPSILON * 128.0,
            "선택지 간격의 탭 격자 비율={ratio}"
        );
        assert_eq!(pair[0].1.para_index, pair[1].1.para_index);
        assert!(pair[0].0.bbox.x + pair[0].0.bbox.width <= pair[1].0.bbox.x);
    }
    let column = nodes
        .iter()
        .find(|n| matches!(n.node_type, RenderNodeType::Column(_)))
        .unwrap();
    let last = equations.last().unwrap().0;
    assert!(last.bbox.x + last.bbox.width <= column.bbox.x + column.bbox.width);
}

#[test]
fn same_base_unit_keeps_arrow_paint_width_when_reserved_frames_differ() {
    let core = DocumentCore::from_bytes(ORIGINAL).unwrap();
    let tree = core.build_page_render_tree(0).unwrap();
    let mut nodes = Vec::new();
    descendants(&tree.root, &mut nodes);
    let equation = |script: &str| {
        nodes
            .iter()
            .find_map(|node| match &node.node_type {
                RenderNodeType::Equation(eq) if eq.script == script => Some((*node, eq)),
                _ => None,
            })
            .unwrap()
    };
    let (first_node, first) = equation("rm dyadAB");
    let (cell_node, cell) = equation("rm dyad AB");
    assert_eq!(first.font_size, cell.font_size);
    assert!(first_node.bbox.width > cell_node.bbox.width);
    assert_eq!(first.layout_box.width, cell.layout_box.width);

    // 독립 Print의 발문/보기 AB는 같은 BaseUnit과 같은 장식 폭이다.
    // 생산 SVG의 모든 조상 transform을 반영해 실제 paint 폭을 비교한다.
    let svg = core
        .render_page_svg_layer_with_profile_native(0, rhwp::paint::RenderProfile::Print)
        .unwrap();
    let xml = roxmltree::Document::parse(&svg).unwrap();
    let mut widths = Vec::new();
    for group in xml.descendants().filter(|n| n.has_tag_name("g")) {
        let texts: Vec<_> = group
            .children()
            .filter(|n| n.has_tag_name("text"))
            .collect();
        if texts.len() != 1 || texts[0].text() != Some("AB") {
            continue;
        }
        let Some(line) = group.children().find(|n| n.has_tag_name("line")) else {
            continue;
        };
        let x1: f64 = line.attribute("x1").unwrap().parse().unwrap();
        let x2: f64 = line.attribute("x2").unwrap().parse().unwrap();
        let mut scale = 1.0;
        for parent in group.ancestors() {
            for transform in
                svgtypes::TransformListParser::from(parent.attribute("transform").unwrap_or(""))
            {
                match transform.unwrap() {
                    svgtypes::TransformListToken::Scale { sx, .. } => scale *= sx,
                    svgtypes::TransformListToken::Translate { .. } => {}
                    other => panic!("원본 AB의 축 정렬 대조에 예상하지 않은 transform: {other:?}"),
                }
            }
        }
        widths.push((x2 - x1).abs() * scale);
        assert_ne!(
            texts[0].attribute("font-style"),
            Some("italic"),
            "rm은 정체다"
        );
    }
    assert_eq!(widths.len(), 3, "발문·보기 ㄱ/ㄴ AB 장식 모두 보존");
    for width in &widths[1..] {
        assert!(
            (widths[0] / width - 1.0).abs() <= f64::EPSILON * 128.0,
            "같은 BaseUnit의 장식 폭 비율이 프레임 폭에 따라 달라지면 안 된다: {widths:?}"
        );
    }
}

fn assert_relation_space(marker: &str, expected_symbol: &str) {
    use rhwp::renderer::equation::layout::{LayoutBox, LayoutKind};
    use rhwp::renderer::equation::symbols::DecoKind;

    fn collect(
        layout: &LayoutBox,
        parent_x: f64,
        arrows: &mut Vec<(f64, f64)>,
        relations: &mut Vec<(String, f64, f64)>,
    ) {
        let x = parent_x + layout.x;
        match &layout.kind {
            LayoutKind::Row(children) => {
                for child in children {
                    collect(child, x, arrows, relations);
                }
            }
            LayoutKind::FontStyle { body, .. } => collect(body, x, arrows, relations),
            LayoutKind::Decoration {
                kind: DecoKind::Dyad,
                body,
            } => {
                arrows.push((x + body.x, body.width));
                collect(body, x, arrows, relations);
            }
            LayoutKind::MathSymbol(symbol) if symbol == "⊥" || symbol == "⫽" => {
                relations.push((symbol.clone(), x, layout.width));
            }
            _ => {}
        }
    }

    let core = DocumentCore::from_bytes(ORIGINAL).unwrap();
    let tree = core.build_page_render_tree(0).unwrap();
    let mut nodes = Vec::new();
    descendants(&tree.root, &mut nodes);
    {
        let equation = nodes
            .iter()
            .find_map(|node| match &node.node_type {
                RenderNodeType::Equation(eq) if eq.script.contains(marker) => Some(eq),
                _ => None,
            })
            .unwrap();
        let mut arrows = Vec::new();
        let mut relations = Vec::new();
        collect(&equation.layout_box, 0.0, &mut arrows, &mut relations);
        assert_eq!(arrows.len(), 2, "양쪽 dyad의 내용 보존");
        assert_eq!(relations.len(), 1, "직접 Unicode도 관계 기호로 보존");
        let (symbol, x, width) = &relations[0];
        assert_eq!(symbol, expected_symbol);
        assert!(
            *x > arrows[0].0 + arrows[0].1,
            "Print처럼 왼쪽 수식과 기호 사이를 띄운다"
        );
        assert!(
            *x + width < arrows[1].0,
            "기호의 실제 점유 폭 뒤에 오른쪽 수식을 놓는다"
        );
    }
}

#[test]
fn choice_perpendicular_keeps_operand_order_and_clear_space() {
    assert_relation_space(" BOT ", "⊥");
}

#[test]
fn choice_direct_unicode_relation_uses_the_same_math_spacing() {
    assert_relation_space("⫽", "⫽");
}

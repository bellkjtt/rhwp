//! 본문 선두의 미주 참조 번호는 본문 문자열과 별개의 표시 런이다.

use super::*;

pub(crate) struct LeadingEndnotePrefixRun {
    pub text: String,
    pub style: TextStyle,
    pub style_id: u32,
    pub position: usize,
    pub width: f64,
}

/// 첫 본문 줄 앞의 번호 표시와 그 줄 채움이 소비하는 공통 점유.
pub(crate) struct LeadingEndnotePrefixGeometry {
    pub runs: Vec<LeadingEndnotePrefixRun>,
    pub advance: f64,
    pub height: f64,
    pub baseline: f64,
    pub baseline_rise: f64,
}

pub(crate) fn leading_endnote_prefix_geometry(
    paragraph: &Paragraph,
    styles: &ResolvedStyleSet,
    first_char: usize,
) -> LeadingEndnotePrefixGeometry {
    let positions = paragraph.control_text_positions();
    let style_id = paragraph.char_shapes.first().map_or(0, |s| s.char_shape_id);
    let base_style = resolved_to_text_style(styles, style_id, 0);
    // 정상 한컴 Print의 본문 참조 번호는 본문 글자의 3/4 크기다.
    let scale = 0.75;
    let rise = base_style.font_size * (1.0 - scale) * TAC_OBJECT_ASCENT_RATIO;
    let mut geometry = LeadingEndnotePrefixGeometry {
        runs: Vec::new(),
        advance: 0.0,
        height: 0.0,
        baseline: base_style.font_size * TAC_OBJECT_ASCENT_RATIO - rise,
        baseline_rise: rise,
    };
    for (control_index, control) in paragraph.controls.iter().enumerate() {
        let Control::Endnote(note) = control else {
            continue;
        };
        let Some(&position) = positions.get(control_index) else {
            continue;
        };
        if !is_leading_endnote_marker_rendered_as_prefix(
            Some(paragraph),
            control_index,
            0,
            0,
            position,
            first_char,
        ) {
            continue;
        }
        // 원문에 공백을 추가하지 않는다. 언어별 표시 face와 실제 폭도 공유한다.
        let label = note_marker_text_from_control(Some(control), note.number);
        let runs = crate::renderer::composer::split_runs_by_lang(vec![ComposedTextRun {
            text: label,
            char_style_id: style_id,
            ..Default::default()
        }]);
        for run in runs {
            let mut style = resolved_to_text_style(styles, style_id, run.lang_index);
            style.font_size *= scale;
            let width = estimate_text_width_exact(&run.text, &style);
            geometry.advance += width;
            geometry.height = geometry.height.max(style.font_size);
            geometry.runs.push(LeadingEndnotePrefixRun {
                text: run.text,
                style,
                style_id,
                position,
                width,
            });
        }
    }
    geometry
}

impl LayoutEngine {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn layout_leading_endnote_prefix(
        &self,
        tree: &mut PageLayoutContext,
        parent: &mut RenderNode,
        paragraph: Option<&Paragraph>,
        composed: &ComposedParagraph,
        styles: &ResolvedStyleSet,
        start_line: usize,
        area: &LayoutRect,
        margin_left: f64,
        indent: f64,
        y: f64,
        spacing_before: f64,
        section_index: usize,
        para_index: usize,
    ) -> f64 {
        let Some(paragraph) = paragraph.filter(|_| start_line == 0) else {
            return 0.0;
        };
        let first = composed.lines.first();
        let first_char = first.map_or(0, |line| line.char_start);
        let geometry = leading_endnote_prefix_geometry(paragraph, styles, first_char);
        let baseline = hwpunit_to_px(first.map_or(0, |line| line.baseline_distance), self.dpi)
            - geometry.baseline_rise;
        let height = hwpunit_to_px(first.map_or(0, |line| line.line_height), self.dpi);
        let mut advance = 0.0;
        for run in geometry.runs {
            let node = RenderNode::new(
                tree.next_id(),
                RenderNodeType::TextRun(TextRunNode {
                    text: run.text,
                    style: run.style,
                    char_shape_id: Some(run.style_id),
                    para_shape_id: Some(composed.para_style_id),
                    section_index: Some(section_index),
                    para_index: Some(para_index),
                    char_start: Some(run.position),
                    baseline,
                    cell_context: None,
                    is_para_end: false,
                    is_line_break_end: false,
                    rotation: 0.0,
                    is_vertical: false,
                    char_overlap: None,
                    border_fill_id: 0,
                    field_marker: FieldMarkerType::None,
                    layout_positions: None,
                    display_text: None,
                }),
                BoundingBox::new(
                    area.x + margin_left + indent + advance,
                    y + spacing_before,
                    run.width,
                    height,
                ),
            );
            parent.children.push(node);
            advance += run.width;
        }
        advance
    }
}

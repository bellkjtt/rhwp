//! 본문 선두의 미주 참조 번호는 본문 문자열과 별개의 표시 런이다.

use super::*;

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
        let positions = paragraph.control_text_positions();
        let first = composed.lines.first();
        let first_char = first.map_or(0, |line| line.char_start);
        let style_id = paragraph.char_shapes.first().map_or(0, |s| s.char_shape_id);
        let base_style = resolved_to_text_style(styles, style_id, 0);
        // 독립 한컴 Print의 참조 번호는 본문 글자의 3/4 크기이며,
        // 줄 기준선에서 줄어든 글자 어센트만큼 올라간다. 미주 영역의 번호
        // 모양 옵션과 본문 참조 번호의 이 표시 계약을 구분한다.
        let scale = 0.75;
        let rise = base_style.font_size * (1.0 - scale) * TAC_OBJECT_ASCENT_RATIO;
        let baseline =
            hwpunit_to_px(first.map_or(0, |line| line.baseline_distance), self.dpi) - rise;
        let height = hwpunit_to_px(first.map_or(0, |line| line.line_height), self.dpi);
        let mut advance = 0.0;
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
                start_line,
                position,
                first_char,
            ) {
                continue;
            }
            // 원문에 없는 공백을 덧붙이지 않는다. 장식·숫자의 언어별 face도
            // 뒤 본문과 같은 resolver로 정해 측정과 실제 표시가 함께 소비한다.
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
                let node = RenderNode::new(
                    tree.next_id(),
                    RenderNodeType::TextRun(TextRunNode {
                        text: run.text,
                        style,
                        char_shape_id: Some(style_id),
                        para_shape_id: Some(composed.para_style_id),
                        section_index: Some(section_index),
                        para_index: Some(para_index),
                        char_start: Some(position),
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
                        width,
                        height,
                    ),
                );
                parent.children.push(node);
                advance += width;
            }
        }
        advance
    }
}

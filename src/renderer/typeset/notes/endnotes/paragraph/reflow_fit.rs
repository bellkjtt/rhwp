//! 수용한 저장 경계 뒤의 전체 문단과 줄 컷을 실제 점유 결과로 fit한다.

use crate::renderer::typeset::{
    ColumnBreakType, FormattedParagraph, Paragraph, ResolvedStyleSet, TypesetEngine, TypesetState,
};

pub(super) enum EndnoteFragmentFit {
    Whole,
    Prefix(usize),
    Move,
}

impl TypesetEngine {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_whole_paragraph_fits(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        para_index: usize,
        available: f64,
        column_width: f64,
    ) -> bool {
        paragraph.column_type == ColumnBreakType::None
            && self
                .simulate_endnote_column_with_tail(
                    st,
                    paragraphs,
                    styles,
                    available,
                    column_width,
                    Some((para_index, None)),
                    true,
                    Some((&[], 0, false)),
                )
                .is_some_and(|occupied_end| occupied_end <= available)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_fragment_fit(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        para_index: usize,
        available: f64,
        column_width: f64,
    ) -> Option<EndnoteFragmentFit> {
        if !st.column_had_compact_endnote_rewind
            || st.current_items.is_empty()
            || formatted.line_heights.is_empty()
            || formatted.line_heights.len() != paragraph.line_segs.len()
            || paragraph.column_type != ColumnBreakType::None
        {
            return None;
        }
        let style = styles.para_styles.get(paragraph.para_shape_id as usize);
        if style.is_some_and(|style| style.keep_with_next) {
            return None;
        }
        let occupied_end = |end_line| {
            self.simulate_endnote_column_with_tail(
                st,
                paragraphs,
                styles,
                available,
                column_width,
                Some((para_index, end_line)),
                true,
                Some((&[], 0, false)),
            )
        };
        if occupied_end(None)? <= available {
            return Some(EndnoteFragmentFit::Whole);
        }
        // 별도 Shape 항목이 필요한 개체의 새 prefix 컷은 소유 항목의 분할
        // 계약까지 갖춰야 한다. 현재 측정만으로 그 계약을 대신하지 않는다.
        let has_separate_object = paragraph.controls.iter().any(|control| {
            matches!(
                control,
                crate::model::control::Control::Picture(_)
                    | crate::model::control::Control::Table(_)
                    | crate::model::control::Control::Shape(_)
            )
        });
        if !style.is_some_and(|style| style.keep_lines) && !has_separate_object {
            let count = formatted.line_heights.len();
            for end in (1..count).rev() {
                if count >= 4
                    && style.is_some_and(|style| style.widow_orphan)
                    && (end == 1 || count - end == 1)
                {
                    continue;
                }
                if occupied_end(Some(end))? <= available {
                    return Some(EndnoteFragmentFit::Prefix(end));
                }
            }
        }
        Some(EndnoteFragmentFit::Move)
    }
}

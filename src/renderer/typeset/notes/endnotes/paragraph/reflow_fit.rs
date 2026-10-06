//! 수용한 저장 되감김 뒤의 단일 줄은 같은 실제 점유 결과로 fit한다.

use crate::renderer::typeset::{
    ColumnBreakType, FormattedParagraph, Paragraph, ResolvedStyleSet, TypesetEngine, TypesetState,
};

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
                    Some(para_index),
                    true,
                    Some((&[], 0, false)),
                )
                .is_some_and(|occupied_end| occupied_end <= available)
    }
    /// 하단 제목 보정이 앞 내용의 점유 영역과 미주 간격을 지울 수는 없다.
    /// 기존 묶음 판정이 제목을 남긴 경우에도 같은 자연 점유 하한을 적용한다.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_new_head_overflows(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        para_index: usize,
        available: f64,
        column_width: f64,
    ) -> bool {
        if st.current_items.is_empty()
            || st.endnote_between_notes_hu <= 0
            || formatted.line_count() != 1
            || paragraph.column_type != ColumnBreakType::None
        {
            return false;
        }
        let previous = self.simulate_endnote_column_with_tail(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            None,
            true,
            Some((&[], 0, false)),
        );
        previous.is_some_and(|bottom| {
            if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
                eprintln!(
                    "EN_HEAD_OCCUPIED pi={para_index} previous={bottom:.3} head_height={:.3} gap={:.3} available={available:.3}",
                    formatted.height_for_fit,
                    crate::renderer::hwpunit_to_px(st.endnote_between_notes_hu, self.dpi),
                );
            }
            bottom
                + crate::renderer::hwpunit_to_px(st.endnote_between_notes_hu, self.dpi)
                + formatted.height_for_fit
                > available
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_single_line_overflows(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        para_index: usize,
        available: f64,
        column_width: f64,
    ) -> bool {
        // 다줄의 분할 컷은 이 단일 줄 계약으로 바꾸지 않는다.
        if st.current_items.is_empty()
            || formatted.line_heights.len() != 1
            || paragraph.line_segs.len() != 1
            || paragraph.column_type != ColumnBreakType::None
        {
            return false;
        }
        self.simulate_endnote_column_with_tail(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            Some(para_index),
            true,
            Some((&[], 0, false)),
        )
        .is_some_and(|occupied_end| occupied_end > available)
    }
    /// 제목의 실제 paint가 하단 보정을 쓰더라도 자연 점유 하한을 먼저 확인한다.
    /// keep 그룹이 아닌 한 줄 제목만 기존 묶음 판정에서 분리할 수 있다.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_new_head_fits(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        para_index: usize,
        available: f64,
        column_width: f64,
    ) -> bool {
        let Some(style) = styles.para_styles.get(paragraph.para_shape_id as usize) else {
            return false;
        };
        if style.keep_with_next || style.page_break_before || formatted.line_count() != 1 {
            return false;
        }
        let previous_occupied = self.simulate_endnote_column_with_tail(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            None,
            true,
            Some((&[], 0, false)),
        );
        previous_occupied.is_some_and(|bottom| {
            bottom
                + crate::renderer::hwpunit_to_px(st.endnote_between_notes_hu.max(0), self.dpi)
                + formatted.height_for_fit
                <= available
        }) && self.reflowed_endnote_whole_paragraph_fits(
            st,
            paragraphs,
            styles,
            paragraph,
            para_index,
            available,
            column_width,
        )
    }

    /// 앞 단에 남긴 제목의 뒤 첫 줄도 같은 점유 끝에서 시작해야 한다.
    /// 실제 첫 줄조차 안 들어가면 다줄 전체를 새 단에서 시작한다.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_head_successor_first_line_overflows(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        available: f64,
        column_width: f64,
    ) -> bool {
        if paragraph.column_type != ColumnBreakType::None {
            return false;
        }
        let Some(&first_height) = formatted.line_heights.first() else {
            return false;
        };
        let previous = st
            .current_items
            .iter()
            .rev()
            .filter_map(crate::renderer::typeset::page_item_para_index)
            .next()
            .and_then(|index| {
                crate::renderer::typeset::paragraph_by_global_index(
                    paragraphs,
                    &st.endnote_paragraphs,
                    index,
                )
            });
        let Some(last) = previous.and_then(|p| p.line_segs.last()) else {
            return false;
        };
        self.simulate_endnote_column_with_tail(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            None,
            true,
            Some((&[], 0, false)),
        )
        .is_some_and(|bottom| {
            bottom
                + crate::renderer::hwpunit_to_px(last.line_spacing.max(0), self.dpi)
                + formatted.spacing_before
                + first_height
                > available
        })
    }
}

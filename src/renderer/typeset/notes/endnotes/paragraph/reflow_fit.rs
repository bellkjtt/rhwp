//! 수용한 저장 되감김 뒤의 단일 줄은 같은 실제 점유 결과로 fit한다.

use crate::renderer::typeset::{
    ColumnBreakType, FormattedParagraph, Paragraph, ResolvedStyleSet, TypesetEngine, TypesetState,
};

impl TypesetEngine {
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
        if !st.column_had_compact_endnote_rewind
            || st.current_items.is_empty()
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
}

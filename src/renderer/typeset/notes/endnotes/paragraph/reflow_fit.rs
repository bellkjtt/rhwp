//! 저장 줄의 실제 흐름 예약과 점유 결과로 미주의 fit을 확인한다.

use crate::renderer::typeset::{
    ColumnBreakType, FormattedParagraph, MeasuredTable, PageItem, Paragraph, ResolvedStyleSet,
    TypesetEngine, TypesetState,
};

/// 실제 점유 하한과 명시적 묶음 속성을 구분한 새 미주 제목의 fit 결과.
#[derive(Default)]
pub(super) struct NewHeadFit {
    pub(super) natural_overflows: bool,
    pub(super) can_stay: bool,
}

impl TypesetEngine {
    /// 저장 단의 잔여 높이는 현재 문단뿐 아니라 앞서 방출한 수식 줄에도 의존한다.
    pub(super) fn endnote_column_has_reissued_rows(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
    ) -> bool {
        st.current_items.iter().any(|item| {
            let Some(para_index) = crate::renderer::typeset::page_item_para_index(item) else {
                return false;
            };
            let Some(source) = para_index
                .checked_sub(paragraphs.len())
                .and_then(|local| st.endnote_para_sources.get(local))
            else {
                return false;
            };
            let Some(crate::model::control::Control::Endnote(note)) = paragraphs
                .get(source.para_index)
                .and_then(|parent| parent.controls.get(source.control_index))
            else {
                return false;
            };
            let Some(original) = note.paragraphs.get(source.note_para_index) else {
                return false;
            };
            let composed =
                crate::renderer::composer::compose_paragraph_in_context(original, styles);
            crate::renderer::composer::stored_row_projection::reissued_equation_rows(
                original, &composed,
            )
        })
    }

    /// 메트릭이 바뀐 저장 줄의 분할은 실제 앞부분의 점유로 확인한다.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reflowed_endnote_prefix_split(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        para_index: usize,
        saved_split: usize,
        available: f64,
        column_width: f64,
        measured_tables: &[MeasuredTable],
    ) -> Option<usize> {
        let style = styles.para_styles.get(paragraph.para_shape_id as usize)?;
        if paragraph.column_type != ColumnBreakType::None
            || style.keep_lines
            || style.keep_with_next
            || paragraph.controls.iter().any(|control| {
                matches!(
                    control,
                    crate::model::control::Control::Table(_)
                        | crate::model::control::Control::Picture(_)
                        | crate::model::control::Control::Shape(_)
                )
            })
        {
            return None;
        }
        (saved_split + 1..paragraph.line_segs.len())
            .rev()
            .find(|&end_line| {
                if style.widow_orphan
                    && (end_line < 2 || paragraph.line_segs.len() - end_line < 2)
                {
                    return false;
                }
                let occupied = self.simulate_endnote_column_with_item(
                    st, paragraphs, styles, available, column_width,
                    Some(PageItem::PartialParagraph { para_index, start_line: 0, end_line }),
                    true, None, measured_tables, true,
                );
                if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
                    eprintln!("EN_PREFIX_OCCUPIED pi={para_index} end={end_line} bottom={occupied:?} available={available:.3}");
                }
                occupied.is_some_and(|bottom| bottom <= available)
            })
    }

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
        measured_tables: &[MeasuredTable],
    ) -> bool {
        paragraph.column_type == ColumnBreakType::None
            && self
                .simulate_endnote_column_with_item(
                    st,
                    paragraphs,
                    styles,
                    available,
                    column_width,
                    Some(PageItem::FullParagraph { para_index }),
                    true,
                    None,
                    measured_tables,
                    true,
                )
                .is_some_and(|occupied_end| occupied_end <= available)
    }
    /// 이월 전에 실제 배치의 흐름 끝·점유 끝·간격을 함께 확인한다.
    /// 명시적 묶음은 유지하며, 저장 높이 추정만으로 제목을 넘기지 않는다.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn query_reflowed_endnote_new_head_fit(
        &self,
        st: &TypesetState,
        paragraphs: &[Paragraph],
        styles: &ResolvedStyleSet,
        paragraph: &Paragraph,
        formatted: &FormattedParagraph,
        para_index: usize,
        available: f64,
        column_width: f64,
        measured_tables: &[MeasuredTable],
    ) -> NewHeadFit {
        if st.current_items.is_empty()
            || formatted.line_count() != 1
            || paragraph.column_type != ColumnBreakType::None
        {
            return NewHeadFit::default();
        }
        let Some(previous) = self.measure_endnote_column_with_item(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            None,
            true,
            None,
            measured_tables,
        ) else {
            return NewHeadFit::default();
        };
        let Some(occupied) = previous.occupied_bottom else {
            return NewHeadFit::default();
        };
        let gap = crate::renderer::hwpunit_to_px(st.endnote_between_notes_hu.max(0), self.dpi);
        let origin = crate::renderer::endnote_spacing::origin_after_gap(
            previous.flow_bottom,
            Some(occupied),
            gap,
        );
        let natural_overflows = origin + formatted.height_for_fit > available;
        let can_stay = !natural_overflows
            && styles
                .para_styles
                .get(paragraph.para_shape_id as usize)
                .is_some_and(|style| !style.keep_with_next && !style.page_break_before)
            && self.reflowed_endnote_whole_paragraph_fits(
                st,
                paragraphs,
                styles,
                paragraph,
                para_index,
                available,
                column_width,
                measured_tables,
            );
        if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
            eprintln!(
                "EN_HEAD_FIT pi={para_index} flow={:.3} occupied={occupied:.3} origin={origin:.3} head_height={:.3} gap={gap:.3} available={available:.3} overflows={natural_overflows} can_stay={can_stay}",
                previous.flow_bottom,
                formatted.height_for_fit,
            );
        }
        NewHeadFit {
            natural_overflows,
            can_stay,
        }
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
        measured_tables: &[MeasuredTable],
    ) -> bool {
        if formatted.line_heights.len() > 1
            && paragraph.line_segs.len() == formatted.line_heights.len()
            && paragraph.column_type == ColumnBreakType::None
            && !paragraph
                .line_segs
                .windows(2)
                .any(|pair| pair[1].vertical_pos < pair[0].vertical_pos)
        {
            // 유효한 저장 내부 컷은 별도 경로에서 소비한다. 연속 다줄 문단의
            // 첫 물리 줄조차 들어가지 않으면 뒤 줄이나 bleed 예외로 fit하지 않는다.
            let geometry = self.measure_endnote_column_with_item(
                st,
                paragraphs,
                styles,
                available,
                column_width,
                Some(PageItem::PartialParagraph {
                    para_index,
                    start_line: 0,
                    end_line: 1,
                }),
                true,
                None,
                measured_tables,
            );
            if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
                eprintln!(
                    "EN_FIRST_LINE_FIT pi={para_index} flow={:?} occupied={:?} available={available:.3}",
                    geometry.as_ref().map(|result| result.flow_bottom),
                    geometry.as_ref().and_then(|result| result.occupied_bottom),
                );
            }
            return geometry.is_some_and(|result| {
                result.flow_bottom > available
                    || result
                        .occupied_bottom
                        .is_some_and(|bottom| bottom > available)
            });
        }
        // 다줄의 분할 컷은 이 단일 줄 계약으로 바꾸지 않는다.
        if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
            eprintln!(
                "EN_SINGLE_FIT pi={para_index} items={} formatted_lines={} stored_lines={} break={:?} available={available:.3}",
                st.current_items.len(), formatted.line_heights.len(), paragraph.line_segs.len(), paragraph.column_type,
            );
        }
        if st.current_items.is_empty()
            || formatted.line_heights.len() != 1
            || paragraph.line_segs.len() != 1
            || paragraph.column_type != ColumnBreakType::None
        {
            return false;
        }
        // 현재 단과 후보만 측정한다. 빈 미래 문단으로 표·그림의 실제 문맥을
        // 거절하거나 측정값을 빈 배열로 바꾸지 않는다.
        let occupied = self.simulate_endnote_column_with_item(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            Some(PageItem::FullParagraph { para_index }),
            true,
            None,
            measured_tables,
            true,
        );
        if std::env::var_os("RHWP_FLOW_BREAK_DEBUG").is_some() {
            eprintln!(
                "EN_SINGLE_OCCUPIED pi={para_index} bottom={occupied:?} available={available:.3}"
            );
        }
        occupied.is_some_and(|occupied_end| occupied_end > available)
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
        measured_tables: &[MeasuredTable],
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
        self.measure_endnote_column_with_item(
            st,
            paragraphs,
            styles,
            available,
            column_width,
            None,
            true,
            None,
            measured_tables,
        )
        .is_some_and(|geometry| {
            let origin = crate::renderer::endnote_spacing::origin_after_gap(
                geometry.flow_bottom,
                geometry.occupied_bottom,
                crate::renderer::hwpunit_to_px(last.line_spacing.max(0), self.dpi),
            );
            origin + formatted.spacing_before + first_height > available
        })
    }
}

//! 셀의 단 설정 메타데이터와 실제 줄·개체 점유를 구분한다.

use crate::model::{control::Control, paragraph::Paragraph};

impl super::LayoutEngine {
    /// 실제 재조판 요구가 선언을 넘으면 미지 행의 옛 배분을 버리고 같은 제약을 다시 푼다.
    /// 저장 프레임의 요구는 기존 순서대로 마지막 걸침 행에서 계속 보존한다.
    pub(crate) fn resolve_reflow_span_requirements(
        table: &crate::model::table::Table,
        requirements: &[(usize, f64)],
        initial_rows: &[f64],
        content_floors: &[f64],
        declared_shrink_hu: &[u32],
        dpi: f64,
        table_reflowed: bool,
    ) -> Option<Vec<f64>> {
        let is_plain_reflow = |cell: &crate::model::table::Cell| {
            cell.text_direction == 0 && Self::cell_has_plain_reflow_paragraphs(cell, table_reflowed)
        };
        if !requirements.iter().any(|&(index, height)| {
            let cell = &table.cells[index];
            cell.height < 0x8000_0000
                && is_plain_reflow(cell)
                && height > super::hwpunit_to_px(cell.height as i32, dpi)
        }) {
            return None;
        }
        let mut required = vec![0.0f64; table.cells.len()];
        for &(index, height) in requirements {
            required[index] = height;
        }
        let mut constraints: Vec<(usize, usize, f64)> = Vec::new();
        for (index, cell) in table.cells.iter().enumerate() {
            let row = usize::from(cell.row);
            let span = usize::from(cell.row_span);
            if span <= 1 || row + span > initial_rows.len() || cell.height >= 0x8000_0000 {
                continue;
            }
            let declared = super::hwpunit_to_px(cell.height as i32, dpi);
            let total = if is_plain_reflow(cell) {
                declared.max(required[index])
            } else {
                declared
            } - super::hwpunit_to_px(1, dpi) * table.inner_cell_spacing_hu(span) as f64;
            if let Some(prior) = constraints
                .iter_mut()
                .find(|value| value.0 == row && value.1 == span)
            {
                prior.2 = prior.2.max(total);
            } else {
                constraints.push((row, span, total));
            }
        }
        constraints.sort_by_key(|&(_, span, _)| span);
        let mut heights = initial_rows.to_vec();
        crate::renderer::height_measurer::solve_rowspan_row_heights(&constraints, &mut heights);
        for (row, &shrink) in declared_shrink_hu.iter().enumerate().take(heights.len()) {
            if shrink != 0 {
                heights[row] = (heights[row] - super::hwpunit_to_px(shrink as i32, dpi))
                    .max(content_floors[row]);
            }
        }
        for &(index, required) in requirements {
            let cell = &table.cells[index];
            let row = usize::from(cell.row);
            let span = usize::from(cell.row_span);
            let combined = heights[row..row + span].iter().sum::<f64>()
                + super::hwpunit_to_px(1, dpi) * table.inner_cell_spacing_hu(span) as f64;
            if required > combined {
                heights[row + span - 1] += required - combined;
            }
        }
        Some(heights)
    }

    /// 저장 프레임을 높이 컷으로 다시 해석하지 않는 일반 재조판 출처.
    pub(crate) fn cell_has_plain_reflow_paragraphs(
        cell: &crate::model::table::Cell,
        table_reflowed: bool,
    ) -> bool {
        cell.paragraphs.iter().all(|paragraph| {
            paragraph_has_plain_flow(paragraph)
                && (table_reflowed || crate::renderer::para_has_no_stored_line_segs(paragraph))
        })
    }

    pub(crate) fn cell_uses_plain_reflow_units(
        &self,
        cell: &crate::model::table::Cell,
        table: &crate::model::table::Table,
    ) -> bool {
        Self::cell_has_plain_reflow_paragraphs(
            cell,
            self.render_normalization
                .borrow()
                .table_text_reflowed(table),
        )
    }

    /// 일반 칸이 없는 행의 종료 병합 칸이 실제 수용 공간에서 모두 끝났는지 확인한다.
    /// 계속되는 다른 병합 칸의 높이와 내용 커서는 별도 원장에 남긴다.
    pub(crate) fn row_band_completes_ending_reflow_spans(
        &self,
        table: &crate::model::table::Table,
        row: usize,
        accepted: &[crate::renderer::pagination::StraddleCellCarry],
        styles: &crate::renderer::style_resolver::ResolvedStyleSet,
    ) -> bool {
        if table.page_break != crate::model::table::TablePageBreak::RowBreak
            || !Self::row_cut_cell_order(table, row).is_empty()
        {
            return false;
        }
        let mut has_ending_owner = false;
        for (index, cell) in table.cells.iter().enumerate().filter(|(_, cell)| {
            cell.row_span > 1 && usize::from(cell.row) + usize::from(cell.row_span) == row + 1
        }) {
            has_ending_owner = true;
            if !self.cell_uses_plain_reflow_units(cell, table) {
                return false;
            }
            let Some(carry) = accepted.iter().find(|carry| carry.cell_index == index) else {
                return false;
            };
            let units = self.cell_units(cell, table, styles);
            if !carry
                .fragment_cut
                .is_some_and(|(_, end)| end == units.len())
            {
                return false;
            }
            let Some(physical) = carry.accepted_physical_height else {
                return false;
            };
            // 끝 행 소속만으로 미소비 줄이 모두 들어갔다고 추정하지 않는다.
            let content = self.cell_cut_visible_height(cell, table, styles, 0, units.len());
            let roundoff = f64::EPSILON * content.abs().max(physical.abs()).max(1.0) * 8.0;
            if content > physical + roundoff {
                return false;
            }
        }
        has_ending_owner
    }
}

impl super::CellUnit {
    /// 빈 간격 유닛을 제외하고 실제 가시 줄 범위의 존재만 공유한다.
    pub(in crate::renderer::layout) fn has_visible_line(&self) -> bool {
        !self.empty_spacer && self.vis_start < self.vis_end
    }
}

/// 한 단 설정은 이미 선택한 셀 프레임을 바꾸거나 내용 상자를 추가하지 않는다.
/// 다단 설정이나 독립 개체의 흐름 계약까지 이 판정으로 확대하지 않는다.
pub(super) fn paragraph_has_plain_flow(paragraph: &Paragraph) -> bool {
    paragraph
        .controls
        .iter()
        .all(|control| matches!(control, Control::ColumnDef(columns) if columns.column_count == 1))
}

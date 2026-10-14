//! 저장 선행 간격의 공통 원점: 측정과 실제 배치가 같은 경계를 소비한다.
use super::{hwpunit_to_px, Control, Paragraph};

/// A saved first line fixes both the column origin and retained before-space.
/// Pagination and paint consume this same frame instead of rebasing separately.
#[derive(Clone, Copy, Debug)]
pub struct StoredColumnTopSpacing {
    pub before: f64,
    pub source_origin_hu: i32,
}

/// Resolve a saved first-line before-space without confusing a continuous
/// section coordinate with a page coordinate. The caller already reserved
/// `spacing_before`; a closed source boundary proves that paint must keep it.
pub(crate) fn stored_column_top_spacing_before(
    paragraphs: &[Paragraph],
    index: usize,
    spacing_before: f64,
    styles: &crate::renderer::style_resolver::ResolvedStyleSet,
    dpi: f64,
) -> Option<StoredColumnTopSpacing> {
    let current = paragraphs.get(index)?;
    let valid = |para: &Paragraph| {
        (para.controls.is_empty()
            || matches!((para.controls.as_slice(), para.line_segs.as_slice()),
                ([Control::Table(table)], [line])
                if table.common.treat_as_char
                    && table.common.height > 0
                    && table.common.height < 0x8000_0000
                    && para.text.chars().all(|ch| ch.is_whitespace() || ch < '\u{0020}' || ch == '\u{FFFC}')
                    && i64::from(line.line_height) == i64::from(table.common.height)
                        + i64::from(table.outer_margin_top) + i64::from(table.outer_margin_bottom)))
            && !para.stored_text_partition_is_dirty()
            && !para.cell_format_vpos_dirty
            && !para.line_segs.is_empty()
            && para.line_segs.iter().all(|line| {
                line.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                    && line.vertical_pos >= 0
                    && line.line_height > 0
                    && line.line_spacing >= 0
            })
    };
    // 현재 문단의 첫 저장 경계만 사용한다. 뒤 줄의 쪽 되감김은 그 경계를
    // 무효화하지 않는다. 새 쪽 전체 이월 여부는 호출자의 소유·흐름 검사로 확정된다.
    if !valid(current) || !spacing_before.is_finite() || spacing_before <= 0.0 {
        return None;
    }
    let source_position = |para: &Paragraph, line: usize| {
        para.source_line_seg_vertical_pos
            .as_ref()
            .and_then(|positions| positions.get(line).copied())
            .unwrap_or(para.line_segs[line].vertical_pos)
    };
    let first = source_position(current, 0);
    let one_source_unit = dpi / 7200.0;
    let frame = StoredColumnTopSpacing {
        before: spacing_before,
        source_origin_hu: first.checked_sub(crate::renderer::px_to_hwpunit(spacing_before, dpi))?,
    };
    if (hwpunit_to_px(first, dpi) - spacing_before).abs() <= one_source_unit {
        return Some(frame);
    }
    let previous = paragraphs.get(index.checked_sub(1)?)?;
    if !valid(previous)
        || !previous.line_segs.windows(2).all(|lines| {
            lines[1]
                .vertical_pos
                .checked_sub(lines[0].vertical_pos)
                .is_some_and(|delta| delta >= lines[0].line_height)
        })
    {
        return None;
    }
    let last_index = previous.line_segs.len() - 1;
    let last = &previous.line_segs[last_index];
    let gap = first
        .checked_sub(source_position(previous, last_index))?
        .checked_sub(last.line_height)?
        .checked_sub(last.line_spacing)?;
    let previous_after = styles
        .para_styles
        .get(previous.para_shape_id as usize)?
        .spacing_after;
    // Close both paragraphs' declared spaces against their saved boundary;
    // unexplained blank space is left to its own flow contract.
    ((hwpunit_to_px(gap, dpi) - previous_after - spacing_before).abs() <= one_source_unit)
        .then_some(frame)
}

//! 빈 문자열과 실제 저장 줄의 점유를 구분한다. 입력 캐시는 수정하지 않는다.

use super::InlineFlowPlan;
use crate::model::{
    control::Control,
    paragraph::{ColumnBreakType, LineSeg, Paragraph},
    shape::TextWrap,
};
use crate::renderer::{hwpunit_to_px, px_to_hwpunit, style_resolver::ResolvedStyleSet};

/// 다음 실제 줄의 원점이 증언하는 정상 저장 Square 호스트 예약.
pub(crate) fn plan_stored_empty_square_host(
    host: &Paragraph,
    next: &Paragraph,
    styles: &ResolvedStyleSet,
    dpi: f64,
) -> Option<InlineFlowPlan> {
    if host.text.chars().any(|c| c > '\u{001f}' && c != '\u{fffc}')
        || host.stored_text_partition_is_dirty()
        || host.cell_format_vpos_dirty
        || next.stored_text_partition_is_dirty()
        || next.cell_format_vpos_dirty
        || !matches!(next.column_type, ColumnBreakType::None)
    {
        return None;
    }
    let mut saw_float = false;
    for control in &host.controls {
        let common = match control {
            Control::Picture(p) if p.caption.is_none() => &p.common,
            Control::Shape(s) => s.common(),
            Control::SectionDef(_)
            | Control::ColumnDef(_)
            | Control::PageNumberPos(_)
            | Control::PageNumCtrl(_) => continue,
            _ => return None,
        };
        if common.treat_as_char || common.text_wrap != TextWrap::Square {
            return None;
        }
        saw_float = true;
    }
    if !saw_float {
        return None;
    }
    let first = host.line_segs.first()?;
    let following = next.line_segs.first()?;
    let synthetic = LineSeg::TAG_IMPLEMENTATION_PROPERTY;
    if following.tag & synthetic != 0
        || following.line_height <= 0
        || first.line_height <= 0
        || host.line_segs.iter().any(|s| {
            s.tag & synthetic != 0
                || s.vertical_pos != first.vertical_pos
                || s.line_height <= 0
                || s.line_spacing < 0
        })
        || host
            .line_segs
            .iter()
            .skip(1)
            .any(|s| s.column_start == first.column_start)
    {
        return None;
    }
    let style = styles.para_styles.get(host.para_shape_id as usize)?;
    let following_style = styles.para_styles.get(next.para_shape_id as usize)?;
    if following_style.page_break_before {
        return None;
    }
    let mut row = first.clone();
    row.line_height = host.line_segs.iter().map(|s| s.line_height).max()?;
    row.line_spacing = host.line_segs.iter().map(|s| s.line_spacing).max()?;
    let delta = i64::from(following.vertical_pos) - i64::from(first.vertical_pos);
    let expected = i64::from(row.line_height)
        + i64::from(row.line_spacing)
        + i64::from(px_to_hwpunit(style.spacing_after, dpi))
        + i64::from(px_to_hwpunit(following_style.spacing_before, dpi));
    // HWPUNIT 변환의 저장 반올림만 허용한다. 위치를 px 범위로 골라내지 않는다.
    if delta <= 0 || (delta - expected).abs() > 1 {
        return None;
    }
    row.vertical_pos = 0;
    let end = style.spacing_before
        + hwpunit_to_px(row.line_height + row.line_spacing, dpi)
        + style.spacing_after;
    Some(InlineFlowPlan {
        stored_empty_host: true,
        text_rows: Some(vec![row]),
        square_host_control: None,
        text_spacing_before: Some(style.spacing_before),
        square_host_placement: None,
        square_host_exclusion: None,
        start: 0.0,
        end,
        boxes: Vec::new(),
        rows: Vec::new(),
        carved: false,
        next_row_top: end,
        fallback_font_size: 0.0,
    })
}

//! 수용한 원본 글줄의 내용 끝을 배치와 같은 단 좌표에서 조회한다.

use super::metrics::FormattedParagraph;
use super::{Paragraph, ResolvedStyleSet, TypesetState};
use crate::model::paragraph::LineSeg;
use crate::renderer::{hwpunit_to_px, pagination::PageItem};

/// 단의 원본 글줄 축과 구성 메트릭이 그대로일 때의 실제 하단.
/// 순차 fit 커서에서 생략한 간격을 글줄의 물리 원점으로 대신하지 않는다.
/// 다른 단 원점·미주·재조판은 이 저장 좌표 계약으로 인증하지 않는다.
pub(super) fn stored_content_bottom(
    st: &TypesetState,
    para: &Paragraph,
    fmt: &FormattedParagraph,
    paragraphs: &[Paragraph],
    styles: &ResolvedStyleSet,
    dpi: f64,
) -> Option<f64> {
    if !(st.profile.hwp5_stored_pagination_layout() || st.profile.hwpx_stored_layout())
        || st.profile.hwp3_layout()
        || st.profile.session_edited()
        || st.current_endnote_flow
        || st.vpos_ladder_dirty
        || para.stored_text_partition_is_dirty()
        || para.cell_format_vpos_dirty
        || fmt.exact_reflow_text_box
        || fmt.line_count() != para.line_segs.len()
    {
        return None;
    }
    let (first_owner, first_line) = match st.current_items.first()? {
        PageItem::FullParagraph { para_index } => (*para_index, 0),
        PageItem::PartialParagraph {
            para_index,
            start_line,
            ..
        } => (*para_index, *start_line),
        _ => return None,
    };
    let first = paragraphs.get(first_owner)?;
    let first_seg = first.line_segs.get(first_line)?;
    // 표의 확정 원점이 인증한 축도 실제 단 시작 높이를 소유한다.
    // 저장 좌표의 기준과 물리 앵커를 분리하여 시작 밴드를 두 번 더하지 않는다.
    let (source_base, anchor) = if st.vpos_page_base_stored {
        let base = st.vpos_page_base?;
        if first_seg.vertical_pos != base {
            return None;
        }
        (base, st.vpos_col_anchor)
    } else if first_seg.vertical_pos == 0 && st.current_start_height == 0.0 {
        (0, 0.0)
    } else if first_line == 0
        && st.vpos_col_anchor == st.current_start_height
        && first_seg.vertical_pos
            == (st.current_start_height * crate::renderer::HWPUNIT_PER_INCH / dpi).round() as i32
    {
        // 자동·명시적 단 시작 모두 원본 줄과 실제 시작 밴드가 같은 축을 인증한다.
        // 첫 표가 일반 float여도 뒤 본문을 순차 누적 높이로 되돌리지 않는다.
        (first_seg.vertical_pos, st.vpos_col_anchor)
    } else {
        return None;
    };
    if first.stored_text_partition_is_dirty()
        || first.cell_format_vpos_dirty
        || first_seg.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
        || styles
            .para_styles
            .get(first.para_shape_id as usize)?
            .spacing_before
            != 0.0
        || para
            .line_segs
            .windows(2)
            .any(|pair| pair[1].vertical_pos < pair[0].vertical_pos)
    {
        return None;
    }
    let mut bottom = None;
    for (index, seg) in para.line_segs.iter().enumerate() {
        // 새로 구성한 줄의 높이·간격은 원본 사다리에 억지로 맞추지 않는다.
        if seg.tag & LineSeg::TAG_IMPLEMENTATION_PROPERTY != 0
            || seg.vertical_pos < 0
            || seg.line_height < 0
            || fmt.line_heights[index] != hwpunit_to_px(seg.line_height, dpi)
            || fmt.line_spacings[index] != hwpunit_to_px(seg.line_spacing, dpi)
        {
            return None;
        }
        let end = seg.vertical_pos.checked_add(seg.line_height)?;
        bottom = Some(bottom.map_or(end, |previous: i32| previous.max(end)));
    }
    bottom
        .and_then(|end| end.checked_sub(source_base))
        .map(|end| anchor + hwpunit_to_px(end, dpi))
}

//! 저장 줄 복원 관용과 새로 조판한 텍스트 줄의 합산 오차를 구분한다.

use super::{CellUnit, ROW_CUT_CAPACITY_FP_EPSILON_PX};
use crate::model::table::Cell;

/// 재조판 줄은 실제 내용 예산을 넘겨 수용하지 않는다. 부동소수 합산의
/// 연산 횟수와 값의 크기로 설명되는 오차만 남긴다. 기존 저장 프레임과
/// 중첩 개체의 복원 관용은 해당 경로의 계약을 유지한다.
pub(super) fn for_next_unit(
    cell: &Cell,
    unit: &CellUnit,
    unit_count: usize,
    next_height: f64,
    available: f64,
) -> f64 {
    let composed_text = unit.nested_row.is_none()
        && !unit.mixed_nested_fragment
        && !unit.top_and_bottom_flow
        && cell.paragraphs.get(unit.para_idx).is_some_and(|para| {
            para.controls.is_empty() && crate::renderer::para_has_no_stored_line_segs(para)
        });
    if !composed_text {
        return ROW_CUT_CAPACITY_FP_EPSILON_PX;
    }
    let operations = unit_count.saturating_add(1) as f64;
    (next_height.abs() + available.abs()) * f64::EPSILON * operations
}

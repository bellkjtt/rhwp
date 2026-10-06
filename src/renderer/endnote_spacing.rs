//! 실제 미주 점유 끝에서 간격을 한 번만 예약하는 공통 원점 계약.

pub(crate) fn origin_after_gap(flow_origin: f64, occupied_end: Option<f64>, gap: f64) -> f64 {
    // 실제 점유 끝이 없으면 기존 흐름 기반 추정을 유지한다. 점유 결과가
    // 있으면 이미 소비한 trailing에 같은 미주 간격을 다시 더하지 않는다.
    occupied_end.map_or(flow_origin + gap, |end| (end + gap).max(flow_origin))
}

/// 원점 이동에 따른 부동소수점 반올림만 허용해 간격의 예약 여부를 판정한다.
pub(crate) fn gap_is_reserved(flow_origin: f64, occupied_end: f64, gap: f64) -> bool {
    let end_with_gap = occupied_end + gap;
    let roundoff = (flow_origin.abs() + occupied_end.abs() + gap.abs()) * f64::EPSILON;
    end_with_gap <= flow_origin + roundoff
}

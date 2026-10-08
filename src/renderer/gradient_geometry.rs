//! 도형과 같은 물리 좌표계에서 채우기의 원형 축을 계산한다.

/// 원형 채우기는 긴 축의 절반을 반지름으로 사용한다. 상자의 가로·세로를
/// 각각 정규화하면 직사각형 안에서 원이 타원으로 바뀌므로 사용하지 않는다.
pub(crate) fn radial_circle(
    bbox: (f64, f64, f64, f64),
    center_x: i16,
    center_y: i16,
) -> Option<(f64, f64, f64)> {
    let (x, y, width, height) = bbox;
    if ![x, y, width, height].into_iter().all(f64::is_finite) || width <= 0.0 || height <= 0.0 {
        return None;
    }
    Some((
        x + width * f64::from(center_x) / 100.0,
        y + height * f64::from(center_y) / 100.0,
        width.max(height) / 2.0,
    ))
}

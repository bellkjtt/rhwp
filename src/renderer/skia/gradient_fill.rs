//! 원본 그라데이션의 공통 Native shader를 소비한다.

use crate::renderer::{render_tree::BoundingBox, GradientFillInfo};

pub(super) fn gradient_paint(
    gradient: &GradientFillInfo,
    bbox: &BoundingBox,
    opacity: f32,
) -> Option<skia_safe::Paint> {
    super::fill::gradient_paint(gradient, bbox, f64::from(opacity))
}

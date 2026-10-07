//! 글자처럼 그림이 줄 구성에 제공하는 HWPUNIT 상자 크기.
//!
//! 용지·쪽 상대 비율로 확대된 인쇄 잉크와 줄 상자의 기준선 정렬 크기를 구분한다.
//! 저장 줄과 재조판이 소비하는 동일한 크기를 실제 그림 배치에도 전달한다.

use crate::model::image::Picture;
use crate::model::shape::SizeCriterion;

pub(crate) fn inline_picture_size_hu(picture: &Picture) -> (i32, i32) {
    // 상대 크기의 common 값은 비율이다. 한컴 정상 저장본의 현재 크기와
    // LineSeg는 HWPUNIT이므로 서로 다른 단위인 비율을 줄 높이로 사용하지 않는다.
    let dimension = |declared: u32, current: u32, criterion: SizeCriterion| match criterion {
        SizeCriterion::Absolute => declared as i32,
        _ => current as i32,
    };
    (
        dimension(
            picture.common.width,
            picture.shape_attr.current_width,
            picture.common.width_criterion,
        ),
        dimension(
            picture.common.height,
            picture.shape_attr.current_height,
            picture.common.height_criterion,
        ),
    )
}

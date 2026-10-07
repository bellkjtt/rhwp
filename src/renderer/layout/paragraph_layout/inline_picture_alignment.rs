//! 한 줄 소유 그림의 각 상자를 공통 글줄 기준선에 맞춘다.

use super::{tac_object_box_height_px, tac_picture_outer_margins_px, TAC_OBJECT_ASCENT_RATIO};
use crate::model::{control::Control, image::Picture, paragraph::Paragraph};
use crate::renderer::style_resolver::ResolvedStyleSet;
use crate::renderer::{hwpunit_to_px, layout::LayoutEngine};

impl LayoutEngine {
    pub(super) fn inline_picture_top(
        &self,
        para: Option<&Paragraph>,
        styles: &ResolvedStyleSet,
        line_offsets: &[(usize, f64, usize)],
        line_y: f64,
        baseline: f64,
        picture: &Picture,
    ) -> f64 {
        // 줄 구성에 제공한 상자로 정렬한다. Paper/Page 비율의 인쇄 크기로
        // 다시 계산하면 저장 줄의 기준선과 서로 다른 크기를 빼게 된다.
        let box_height = self.inline_picture_box_height(picture);
        // HWP 5.0 표 44의 명시적 세로 정렬을 그림 상자의 정렬점에도 적용한다.
        // CENTER 줄의 저장 기준선은 줄 중앙인데 상자는 글꼴 어센트(0.85)로
        // 정렬하면 높이의 0.35만큼 위로 올라간다. 원본 정렬점을 보존한다.
        let alignment = para
            .and_then(|p| styles.para_styles.get(p.para_shape_id as usize))
            .map_or(0, |style| style.vertical_alignment);
        let ascent_ratio = match alignment {
            1 => 0.0,
            2 => 0.5,
            3 => 1.0,
            _ => TAC_OBJECT_ASCENT_RATIO,
        };
        let baseline = if baseline > 0.0 {
            baseline
        } else {
            // 기준선 없는 개체 줄은 그 줄에 속한 상자의 어센트로 구성한다.
            // 문단 전체의 다른 줄 그림을 같은 줄의 형제로 사용하지 않는다.
            let tallest = line_offsets
                .iter()
                .filter_map(|(_, _, control)| match para?.controls.get(*control)? {
                    Control::Picture(picture) => Some(self.inline_picture_box_height(picture)),
                    _ => None,
                })
                .fold(box_height, f64::max);
            tallest * ascent_ratio
        };
        line_y + baseline - box_height * ascent_ratio
    }

    fn inline_picture_box_height(&self, picture: &Picture) -> f64 {
        let (_, height_hu) = crate::renderer::composer::inline_picture_size_hu(picture);
        let (_, _, top, bottom) = tac_picture_outer_margins_px(picture, self.dpi);
        tac_object_box_height_px(
            hwpunit_to_px(height_hu, self.dpi),
            &picture.caption,
            self.dpi,
        ) + top
            + bottom
    }
}

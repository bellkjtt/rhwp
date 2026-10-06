//! 한 줄 소유 그림의 각 상자를 공통 글줄 기준선에 맞춘다.

use super::{tac_object_box_height_px, tac_picture_outer_margins_px, TAC_OBJECT_ASCENT_RATIO};
use crate::model::{control::Control, paragraph::Paragraph};
use crate::renderer::{layout::LayoutEngine, page_layout::LayoutRect};

impl LayoutEngine {
    pub(super) fn inline_picture_top(
        &self,
        para: Option<&Paragraph>,
        line_offsets: &[(usize, f64, usize)],
        area: &LayoutRect,
        line_y: f64,
        baseline: f64,
        box_height: f64,
    ) -> f64 {
        let baseline = if baseline > 0.0 {
            baseline
        } else {
            // 기준선 없는 개체 줄은 그 줄에 속한 상자의 어센트로 구성한다.
            // 문단 전체의 다른 줄 그림을 같은 줄의 형제로 사용하지 않는다.
            let tallest = line_offsets
                .iter()
                .filter_map(|(_, _, control)| match para?.controls.get(*control)? {
                    Control::Picture(picture) => {
                        let height = self.resolve_inline_picture_size(picture, area).1;
                        let (_, _, top, bottom) = tac_picture_outer_margins_px(picture, self.dpi);
                        Some(
                            tac_object_box_height_px(height, &picture.caption, self.dpi)
                                + top
                                + bottom,
                        )
                    }
                    _ => None,
                })
                .fold(box_height, f64::max);
            tallest * TAC_OBJECT_ASCENT_RATIO
        };
        line_y + baseline - box_height * TAC_OBJECT_ASCENT_RATIO
    }
}

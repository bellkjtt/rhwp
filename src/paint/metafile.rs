//! 레이어 출력의 장치와 그림 배치를 메타파일 재생에 전달한다.

use crate::model::style::ImageFillMode;
use crate::paint::{ResolvedImageKind, ResolvedImagePayload};
use crate::renderer::render_tree::{BoundingBox, ImageNode, RawSvgNode};

/// 한컴 2024 Print의 EMF 래스터 장치. 독립 Print의 1515×1500 자원에서
/// 1픽셀 cosmetic 선을 확인했다. 한컴 2020의 벡터 경로에는 적용하지 않는다.
pub(crate) const HANCOM_2024_PRINT_DEVICE_DPI: f64 = 600.0;

pub(super) fn image_payload(
    image: &ImageNode,
    bbox: BoundingBox,
    device_pixel_width: Option<f64>,
) -> Option<ResolvedImagePayload> {
    let Some(pixel) = device_pixel_width else {
        return crate::renderer::image_resolver::resolve_image_payload(image);
    };
    let data = image.data.as_deref()?;
    if crate::renderer::image_resolver::detect_image_mime_type(data) != "image/x-emf" {
        return crate::renderer::image_resolver::resolve_image_payload(image);
    }
    let (natural_w, natural_h) = crate::emf::standalone_size(data)?;
    let crop = image.crop.map(|crop| {
        crate::renderer::svg::compute_image_crop_src(
            crop,
            image.crop_reference_size,
            natural_w,
            natural_h,
        )
    });
    let (_, _, crop_w, crop_h) = crop.unwrap_or((0.0, 0.0, natural_w, natural_h));
    if crop_w <= 0.0 || crop_h <= 0.0 {
        return crate::renderer::image_resolver::resolve_image_payload(image);
    }
    let target = image
        .transform
        .effective_image_bbox(&image.paint_bbox(&bbox));
    let mode = image.fill_mode.unwrap_or(ImageFillMode::FitToSize);
    let (width, height) = match mode {
        ImageFillMode::FitToSize | ImageFillMode::Total => (
            target.width * natural_w / crop_w,
            target.height * natural_h / crop_h,
        ),
        ImageFillMode::None | ImageFillMode::Zoom => {
            let scale = (target.width / crop_w).min(target.height / crop_h);
            (natural_w * scale, natural_h * scale)
        }
        _ => {
            let (w, h) = image.original_size.unwrap_or((natural_w, natural_h));
            (w * natural_w / crop_w, h * natural_h / crop_h)
        }
    };
    let data = crate::emf::convert_to_sized_svg(data, width, height, pixel)?;
    Some(ResolvedImagePayload {
        data,
        mime: "image/svg+xml",
        kind: ResolvedImageKind::OutputDevice,
        suppress_effects: false,
    })
}

pub(super) fn raw_svg(raw: &RawSvgNode, device_pixel_width: Option<f64>) -> RawSvgNode {
    let mut output = raw.clone();
    if let (Some(pixel), Some(source)) = (device_pixel_width, raw.emf_source.as_ref()) {
        if let Ok(svg) =
            crate::emf::convert_to_svg_with_device(&source.data, source.render_rect, Some(pixel))
        {
            output.svg = svg;
        }
    }
    output
}

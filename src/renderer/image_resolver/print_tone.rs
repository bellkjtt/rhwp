//! 한컴 2024 Print의 보정 없는 원본색 bitmap 전달.
//!
//! 256/257색 경계와 BMP·PNG·JPEG·2020/2024 Print의 독립 대조는
//! mydocs/working/task_m100_7642_stage2025.md에 있다. 그림 효과 합성은 별도 계약이다.

use std::collections::HashSet;
use std::io::Cursor;

use super::{decode_image_with_format_limited, detect_image_mime_type, memoized, Conversion};
use crate::model::image::ImageEffect;
use crate::paint::{ResolvedImageKind, ResolvedImagePayload};
use crate::renderer::render_tree::ImageNode;

pub(crate) fn is_print_bitmap_candidate(image: &ImageNode) -> bool {
    image.hancom2024_print_bitmap
        && image.effect == ImageEffect::RealPic
        && image.brightness == 0
        && image.contrast == 0
        && image
            .data
            .as_deref()
            .is_some_and(|data| matches!(detect_image_mime_type(data), "image/bmp" | "image/png"))
}

pub(crate) fn print_bitmap_payload(image: &ImageNode) -> Option<ResolvedImagePayload> {
    if !is_print_bitmap_candidate(image) {
        return None;
    }
    Some(ResolvedImagePayload {
        data: bitmap_bytes_to_png(image.data.as_deref()?)?,
        mime: "image/png",
        kind: ResolvedImageKind::FormatConverted,
        suppress_effects: false,
    })
}

pub(super) fn bitmap_bytes_to_png(data: &[u8]) -> Option<Vec<u8>> {
    memoized(Conversion::Hancom2024Bitmap, data, || {
        let format = match detect_image_mime_type(data) {
            "image/bmp" => image::ImageFormat::Bmp,
            "image/png" => image::ImageFormat::Png,
            _ => return None,
        };
        let mut decoded = decode_image_with_format_limited(data, format)?.to_rgba8();
        // 한컴의 팔레트 전달과 full-color 전달을 원본 RGB 색 수로 구분한다.
        // 알파는 별도의 투명도이며 RGB 팔레트의 색 수에 합산하지 않는다.
        const PALETTE_CAPACITY: usize = 1 << 8;
        let mut colors = HashSet::with_capacity(PALETTE_CAPACITY + 1);
        for pixel in decoded.pixels() {
            colors.insert([pixel[0], pixel[1], pixel[2]]);
            if colors.len() > PALETTE_CAPACITY {
                break;
            }
        }
        if colors.len() <= PALETTE_CAPACITY {
            return None;
        }
        for pixel in decoded.pixels_mut() {
            for channel in &mut pixel.0[..3] {
                *channel = channel.saturating_add(*channel >> 5);
            }
        }
        let mut output = Vec::new();
        image::DynamicImage::ImageRgba8(decoded)
            .write_to(&mut Cursor::new(&mut output), image::ImageFormat::Png)
            .ok()?;
        Some(output)
    })
}

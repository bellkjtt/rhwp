//! 선택한 실제 face와 본문 스타일로 동일한 글자 폰트를 만든다.
use super::super::font_lookup::{select_typeface_for_character, TypefaceCandidate};
use crate::renderer::TextStyle;
use skia_safe::{font, Font, Typeface};

pub(super) fn for_text(
    chain: &[TypefaceCandidate],
    sample: &str,
    size: f32,
    style: &TextStyle,
) -> Option<Font> {
    if let Some(ch) = sample.chars().find(|ch| !ch.is_whitespace()) {
        return select_typeface_for_character(chain, ch)
            .map(|candidate| finish(candidate.typeface.clone(), size, style));
    }
    if let Some(candidate) = chain.first() {
        Some(finish(candidate.typeface.clone(), size, style))
    } else {
        let mut font = Font::default();
        font.set_size(size);
        font.set_edging(font::Edging::AntiAlias);
        font.set_subpixel(true);
        Some(font)
    }
}

fn finish(tf: Typeface, size: f32, style: &TextStyle) -> Font {
    let is_bold_face = *tf.font_style().weight() >= 600;
    let is_slanted_face = tf.font_style().slant() != skia_safe::font_style::Slant::Upright;
    let mut font = Font::new(tf, size);
    font.set_edging(font::Edging::AntiAlias);
    // 글줄이 정한 소수 원점을 glyph raster에서도 보존한다.
    font.set_subpixel(true);
    // 원본 폭과 기준선으로 배치한 생성 번호의 소수 원점을
    // 정수로 반올림하지 않는다. Canvas도 이 원점에서 그린다.
    if style.fixed_glyph_advances {
        font.set_baseline_snap(false);
    }
    if style.is_visually_bold() && !is_bold_face {
        font.set_embolden(true);
    }
    // strict glyph replay와 PDF가 쓰는 같은 합성 기울임이다.
    // 실제 기울임 face에는 중복 shear를 적용하지 않는다.
    if style.italic && !is_slanted_face {
        font.set_skew_x(-0.25);
    }
    font
}

//! Sampling declared by a bounded PDF importer in a PNG resource.
//! `rhWP` is a private ancillary, unsafe-to-copy chunk immediately after IHDR.
//! Raw BinData preservation keeps this contract through HWP/HWPX save/reopen;
//! ordinary images without the declaration retain the renderer's profile policy.
fn declaration(data: &[u8]) -> Option<(u8, u8)> {
    if data.len() < 47
        || &data[..8] != b"\x89PNG\r\n\x1a\n"
        || data[8..16] != [0, 0, 0, 13, b'I', b'H', b'D', b'R']
        || data[33..41] != [0, 0, 0, 2, b'r', b'h', b'W', b'P']
        || !matches!(data[41], 1 | 2)
        || data[42] > 1
    {
        return None;
    }
    let expected = u32::from_be_bytes(data[43..47].try_into().ok()?);
    (crc32fast::hash(&data[37..43]) == expected).then_some((data[41], data[42]))
}

pub(crate) fn interpolation(data: &[u8]) -> Option<bool> {
    declaration(data).and_then(|(version, value)| (version == 1).then_some(value != 0))
}

/// 두 번째 버전은 한컴 2024의 실제 색 확장 payload만 표시한다.
/// 일반 그림·팔레트 전달·다른 Print 버전의 표본 정책을 바꾸지 않는다.
pub(crate) fn print_bitmap_reconstruction(data: &[u8]) -> bool {
    declaration(data) == Some((2, 1))
}

pub(super) fn mark_print_bitmap(data: &mut Vec<u8>) {
    if data.len() < 33 || &data[..8] != b"\x89PNG\r\n\x1a\n" {
        return;
    }
    let mut chunk = [0, 0, 0, 2, b'r', b'h', b'W', b'P', 2, 1, 0, 0, 0, 0];
    let crc = crc32fast::hash(&chunk[4..10]);
    chunk[10..14].copy_from_slice(&crc.to_be_bytes());
    data.splice(33..33, chunk);
}

pub(crate) fn preserve(source: &[u8], output: &mut Vec<u8>) {
    if interpolation(source).is_some()
        && interpolation(output).is_none()
        && output.len() >= 33
        && output[..16] == source[..16]
    {
        output.splice(33..33, source[33..47].iter().copied());
    }
}

/// PDF 32000-1 8.9.5.3 defines interpolation for source samples enlarged onto
/// device pixels. Minification still needs the backend's coverage/prefilter;
/// forcing point sampling there discards source pixels (and thin image lines).
pub(crate) fn magnification(
    declared: Option<bool>,
    source: (f64, f64),
    destination: (f64, f64),
) -> Option<bool> {
    if [source.0, source.1, destination.0, destination.1]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.)
        || destination.0 < source.0
        || destination.1 < source.1
    {
        return None;
    }
    declared
}

pub(crate) fn dimensions(data: &[u8]) -> Option<(f64, f64)> {
    interpolation(data)?;
    Some((
        u32::from_be_bytes(data[16..20].try_into().ok()?) as f64,
        u32::from_be_bytes(data[20..24].try_into().ok()?) as f64,
    ))
}

#[cfg(test)]
mod print_transfer_contracts {
    use super::*;

    fn encoded_color() -> Vec<u8> {
        let image = image::RgbaImage::from_pixel(1, 1, image::Rgba([228, 217, 128, 173]));
        let mut output = std::io::Cursor::new(Vec::new());
        image
            .write_to(&mut output, image::ImageFormat::Png)
            .unwrap();
        output.into_inner()
    }

    #[test]
    fn print_transfer_marker_keeps_source_color_alpha_and_dimensions() {
        let source = encoded_color();
        let mut printed = source.clone();
        mark_print_bitmap(&mut printed);
        assert!(print_bitmap_reconstruction(&printed));
        assert_eq!(interpolation(&printed), None);
        assert_eq!(
            image::load_from_memory(&printed).unwrap().to_rgba8(),
            image::load_from_memory(&source).unwrap().to_rgba8(),
            "표식은 실제 색·알파·해상도를 바꾸면 안 된다",
        );
    }

    #[test]
    fn invalid_crc_truncation_and_unknown_marker_version_never_select_reconstruction() {
        let mut printed = encoded_color();
        mark_print_bitmap(&mut printed);
        let mut corrupt = printed.clone();
        corrupt[46] ^= 1;
        assert!(!print_bitmap_reconstruction(&corrupt));
        assert!(!print_bitmap_reconstruction(&printed[..46]));
        printed[41] = 3;
        let crc = crc32fast::hash(&printed[37..43]);
        printed[43..47].copy_from_slice(&crc.to_be_bytes());
        assert!(!print_bitmap_reconstruction(&printed));
        assert_eq!(interpolation(&printed), None);
    }

    #[test]
    fn explicit_source_sampling_overrides_print_transfer_marker_without_duplication() {
        let mut source = encoded_color();
        mark_print_bitmap(&mut source);
        source[41] = 1;
        source[42] = 0;
        let crc = crc32fast::hash(&source[37..43]);
        source[43..47].copy_from_slice(&crc.to_be_bytes());
        let mut printed = encoded_color();
        mark_print_bitmap(&mut printed);
        preserve(&source, &mut printed);
        assert_eq!(interpolation(&printed), Some(false));
        assert!(!print_bitmap_reconstruction(&printed));
        let once = printed.clone();
        preserve(&source, &mut printed);
        assert_eq!(printed, once, "같은 계약을 두 번 싣지 않는다");
        assert_eq!(
            image::load_from_memory(&printed).unwrap().to_rgba8(),
            image::load_from_memory(&source).unwrap().to_rgba8(),
        );
    }
}

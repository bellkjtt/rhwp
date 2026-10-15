use skia_safe::{font_arguments::VariationPosition, Data, FontArguments, FontMgr, Typeface};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static UNEMBOLDENED_TYPEFACES: RefCell<HashMap<u32, Option<Typeface>>> = RefCell::new(HashMap::new());
}

/// 시스템의 합성 weight를 제거하되 같은 font program·collection·가변 축을 보존한다.
/// 실제 Bold 또는 판정 근거가 없는 프로그램은 변경하지 않는다.
pub(super) fn unemboldened_typeface(font_mgr: &FontMgr, typeface: &Typeface) -> Option<Typeface> {
    UNEMBOLDENED_TYPEFACES.with(|cache| {
        let id = typeface.unique_id();
        if let Some(cached) = cache.borrow().get(&id) {
            return cached.clone();
        }
        let result = regular_program_typeface(font_mgr, typeface);
        cache.borrow_mut().insert(id, result.clone());
        result
    })
}

fn regular_program_typeface(font_mgr: &FontMgr, typeface: &Typeface) -> Option<Typeface> {
    let coordinates = typeface.variation_design_position().unwrap_or_default();
    let weight = coordinates
        .iter()
        .find(|coordinate| coordinate.axis == u32::from_be_bytes(*b"wght").into());
    if weight.is_some_and(|coordinate| coordinate.value >= 600.0) {
        return None;
    }
    if *typeface.font_style().weight() < 600 {
        return Some(typeface.clone());
    }
    let mut os2 = [0; 6];
    if typeface.get_table_data(u32::from_be_bytes(*b"OS/2"), &mut os2) != os2.len() {
        return None;
    }
    let program_weight = weight.map_or(
        f32::from(u16::from_be_bytes([os2[4], os2[5]])),
        |coordinate| coordinate.value,
    );
    if !(1.0..600.0).contains(&program_weight) {
        return None;
    }
    // fontconfig의 embolden은 Font::set_embolden(false)로 해제되지 않는다.
    // SFNT 자체를 다시 열어 OS가 덧붙인 가상 weight만 제외한다.
    let (bytes, index) = typeface.to_font_bytes()?;
    let mut regular = font_mgr.new_from_data(Data::new_copy(&bytes), index)?;
    if !coordinates.is_empty() {
        let mut arguments = FontArguments::new().set_variation_design_position(VariationPosition {
            coordinates: &coordinates,
        });
        arguments.set_collection_index(index as usize);
        regular = regular.clone_with_arguments(&arguments)?;
    }
    // 시스템이 붙인 기울임은 SFNT에 남지 않는다. 독립적인 각도 근거 없이
    // 다른 기울임으로 바꾸지 않고 기존 paint를 유지한다.
    if regular.font_style().slant() != typeface.font_style().slant() {
        return None;
    }
    Some(regular)
}

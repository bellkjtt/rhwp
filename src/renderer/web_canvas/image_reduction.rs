//! 원본 전체 그림을 축소하는 동안에만 Canvas의 저역 통과 품질을 선택한다.

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub(super) struct ReductionQuality<'a> {
    context: &'a CanvasRenderingContext2d,
    previous: Option<JsValue>,
}

impl<'a> ReductionQuality<'a> {
    pub(super) fn for_full_image(
        context: &'a CanvasRenderingContext2d,
        source_width: f64,
        source_height: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let reduced = context.get_transform().is_ok_and(|matrix| {
            let scale_x = matrix.a().hypot(matrix.b());
            let scale_y = matrix.c().hypot(matrix.d());
            scale_x.is_finite()
                && scale_y.is_finite()
                && (source_width > width * scale_x || source_height > height * scale_y)
        });
        // 현재 web-sys는 imageSmoothingQuality 접근자를 제공하지 않는다.
        // 브라우저가 실제로 제공하는 속성만 바꾸고 원래 값을 보관한다.
        let key = JsValue::from_str("imageSmoothingQuality");
        let previous = reduced
            .then(|| js_sys::Reflect::get(context.as_ref(), &key).ok())
            .flatten()
            .filter(JsValue::is_string)
            .filter(|_| {
                js_sys::Reflect::set(context.as_ref(), &key, &JsValue::from_str("high"))
                    .unwrap_or(false)
            });
        Self { context, previous }
    }
}

impl Drop for ReductionQuality<'_> {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.as_ref() {
            let _ = js_sys::Reflect::set(
                self.context.as_ref(),
                &JsValue::from_str("imageSmoothingQuality"),
                previous,
            );
        }
    }
}

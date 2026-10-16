//! 실제 수식 face의 글리프와 전진을 측정 결과에 보존한다.
use super::{estimate_text_width, glyph_advances, EqLayout, LayoutBox, LayoutKind};

impl EqLayout {
    pub(super) fn layout_hancom_glyphs(
        &self,
        text: &str,
        role: super::super::hancom_glyphs::Role,
        fs: f64,
        pad: f64,
    ) -> Option<LayoutBox> {
        if !self.hancom_latin {
            return None;
        }
        let family = if self.hancom_eqn {
            "HancomEQN"
        } else {
            super::super::hancom_glyphs::FONT_FAMILY
        };
        let run = super::super::hancom_glyphs::measure(text, role, fs, self.dpi, pad, family)?;
        Some(LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: run.width,
            height: fs,
            baseline: fs * super::TEXT_BASELINE,
            kind: LayoutKind::HancomGlyphs(run),
        })
    }

    pub(super) fn layout_function(&self, name: &str, fs: f64) -> LayoutBox {
        if self.hancom_eqn {
            if let Some(body) = self.layout_hancom_glyphs(
                name,
                super::super::hancom_glyphs::Role::Variable { italic: false },
                fs,
                0.0,
            ) {
                return body;
            }
        }
        if self.hancom_math_face && !self.variable_bold {
            if let Some(body) = super::source_font_text::layout(
                name,
                fs,
                self.dpi,
                false,
                false,
                self.source_variable_glyphs,
            ) {
                return LayoutBox {
                    font_size: fs,
                    x: 0.0,
                    y: 0.0,
                    width: body.width,
                    height: body.height,
                    baseline: body.baseline,
                    kind: LayoutKind::FontStyle {
                        style: super::FontStyleKind::Roman,
                        body: Box::new(body),
                    },
                };
            }
        }
        let w = glyph_advances::text_width(name, fs)
            .unwrap_or_else(|| estimate_text_width(name, fs, false));
        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: w + fs * 0.02,
            height: fs,
            baseline: fs * 0.8,
            kind: LayoutKind::Function(name.to_string()),
        }
    }
}

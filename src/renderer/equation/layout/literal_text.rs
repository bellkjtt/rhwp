//! 인용 문자열은 명령을 해석하지 않고 문자별 글꼴 계약을 유지한다.

use super::{variable_style, EqLayout, LayoutBox, LayoutKind};

impl EqLayout {
    pub(super) fn layout_literal_text(&self, text: &str, fs: f64) -> LayoutBox {
        let mut x = 0.0;
        let mut children = Vec::with_capacity(text.chars().count());
        for ch in text.chars() {
            let literal = ch.to_string();
            // 라틴·그리스 변수는 현재 글꼴 전환을 따르고, CJK는 본문 face를 쓴다.
            // 숫자·문장 부호는 정체로 유지하며 수식 연산자로 재해석하지 않는다.
            let mut child = if ch.is_ascii_alphabetic()
                || variable_style::is_greek_variable(ch)
                || super::super::text_has_cjk(&literal)
            {
                self.layout_text(&literal, fs)
            } else {
                self.layout_number(&literal, fs)
            };
            child.x = x;
            x += child.width;
            children.push(child);
        }
        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: x,
            height: fs,
            baseline: fs * 0.8,
            kind: LayoutKind::Row(children),
        }
    }
}

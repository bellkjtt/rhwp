//! 부정 장식은 본체의 기준선 위에 겹치며 논리 줄 높이를 늘리지 않는다.
//!
//! 독립 한컴 Print는 관계 기호와 빗금을 같은 원점에 그린다. 평평한 등호와
//! 꺾인 부등호의 빗금 장평을 구분한다. 글리프 윤곽 대신 선분을 사용하며
//! 글자 크기와 논리 기준선으로 모든 backend의 표시 기하를 함께 정한다.

use super::layout::{LayoutBox, LayoutKind};

pub(crate) struct NegationGeometry {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub thickness: f64,
}

pub(crate) fn geometry(body: &LayoutBox) -> NegationGeometry {
    let horizontal_scale: f64 = if angular_relation(body) { 2.0 } else { 1.5 };
    let fs = body.font_size;
    // Print의 Tz는 끝점뿐 아니라 빗금의 획도 가로로 늘린다.
    // 기울어진 선의 수직 두께는 변환 전·후 길이와 행렬식으로 구한다.
    // 원 face의 수직 획 메트릭은 약 0.044em이며 글꼴 윤곽은 포함하지 않는다.
    let base_length = 0.36_f64.hypot(0.97);
    let stretched_length = (0.36 * horizontal_scale).hypot(0.97);
    // 표시 빗금의 끝은 논리 줄 상자 밖으로 나갈 수 있다. 본체와 같은
    // 기준선에서 겹쳐 그리며 상자 높이나 다음 문단의 위치를 늘리지 않는다.
    NegationGeometry {
        start: (
            body.x + fs * horizontal_scale * 0.07,
            body.y + body.baseline + fs * 0.18,
        ),
        end: (
            body.x + fs * horizontal_scale * 0.43,
            body.y + body.baseline - fs * 0.79,
        ),
        thickness: fs * 0.044 * horizontal_scale * base_length / stretched_length,
    }
}

fn angular_relation(body: &LayoutBox) -> bool {
    match &body.kind {
        LayoutKind::Symbol(text) | LayoutKind::MathSymbol(text) => {
            matches!(text.as_str(), "<" | ">" | "≤" | "≥")
        }
        LayoutKind::FontStyle { body, .. } => angular_relation(body),
        LayoutKind::Row(children) if children.len() == 1 => angular_relation(&children[0]),
        _ => false,
    }
}

//! 한컴 Print에서 확인한 신명 HFT 프로그램의 고유 ASCII 전진폭.
//!
//! Type3 /Widths와 ToUnicode를 읽은 값이며 글리프 윤곽선이나 글꼴 파일은 아니다.
//! 근거: pdf/issue7642-hft-char-style-diagnostic/char-style-ascii-2020.pdf,
//! Hancom 11.0.0.9136, Print method 0. 생성·해시는 Stage92 기록을 따른다.
//! 굵기 간격은 intrinsic 폭과 분리하여 문자 측정 walk에서 적용한다.

use super::{FontMetric, LatinRange};

static ASCII_0: [u16; 95] = [
    400, 500, 400, 660, 660, 1000, 1000, 400, 500, 500, 500, 1000, 400, 500, 400, 500, 660, 660,
    660, 660, 660, 660, 660, 660, 660, 660, 500, 500, 500, 1000, 500, 660, 1000, 782, 713, 764,
    771, 761, 730, 819, 771, 340, 585, 710, 689, 872, 769, 818, 719, 819, 767, 712, 746, 771, 726,
    988, 683, 716, 742, 500, 500, 500, 500, 500, 400, 662, 622, 565, 622, 618, 480, 620, 628, 294,
    294, 562, 294, 896, 627, 621, 623, 622, 467, 574, 456, 627, 567, 783, 573, 559, 649, 500, 500,
    500, 1000,
];
static RANGES_0: [LatinRange; 1] = [LatinRange {
    start: 0x0020,
    end: 0x007e,
    widths: &ASCII_0,
}];

static ASCII_1: [u16; 95] = [
    400, 500, 400, 620, 620, 1000, 1000, 400, 500, 500, 500, 1000, 400, 500, 400, 500, 620, 620,
    620, 620, 620, 620, 620, 620, 620, 620, 500, 500, 500, 1000, 500, 620, 1000, 779, 696, 756,
    759, 694, 663, 811, 755, 299, 558, 698, 619, 867, 752, 812, 719, 808, 758, 698, 656, 754, 725,
    1010, 720, 710, 641, 500, 500, 500, 500, 500, 400, 571, 619, 521, 619, 575, 400, 573, 578, 244,
    243, 519, 243, 860, 578, 574, 577, 577, 363, 528, 315, 577, 524, 747, 550, 502, 511, 500, 500,
    500, 1000,
];
static RANGES_1: [LatinRange; 1] = [LatinRange {
    start: 0x0020,
    end: 0x007e,
    widths: &ASCII_1,
}];

pub(super) static METRICS: [FontMetric; 2] = [
    FontMetric {
        name: "신명 디나루 (HFT)",
        bold: false,
        italic: false,
        em_size: 1000,
        latin_ranges: &RANGES_0,
        hangul: None,
    },
    FontMetric {
        name: "신명 신그래픽 (HFT)",
        bold: false,
        italic: false,
        em_size: 1000,
        latin_ranges: &RANGES_1,
        hangul: None,
    },
];

/// HFT로 해소한 선언 face에만 원본 프로그램의 메트릭 이름을 공급한다.
pub(crate) fn metric_face_name(requested: &str) -> Option<&'static str> {
    match requested {
        "신명 디나루" => Some("신명 디나루 (HFT)"),
        "신명 신그래픽" => Some("신명 신그래픽 (HFT)"),
        _ => None,
    }
}

pub(crate) fn is_metric_face(name: &str) -> bool {
    METRICS.iter().any(|metric| metric.name == name)
}

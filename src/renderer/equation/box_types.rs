//! 수식 측정·배치가 공유하는 결과 타입.

use super::ast::MatrixStyle;

/// 수식 레이아웃 박스
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutBox {
    /// 측정에 사용한 글자 크기. 각 backend가 첨자 비율을 다시 추정하지 않는다.
    pub font_size: f64,
    /// X 위치 (부모 기준 상대 좌표)
    pub x: f64,
    /// Y 위치 (부모 기준 상대 좌표)
    pub y: f64,
    /// 폭
    pub width: f64,
    /// 높이
    pub height: f64,
    /// 기준선 (상단으로부터의 거리, 텍스트 정렬의 기준)
    pub baseline: f64,
    /// 렌더링 요소
    pub kind: LayoutKind,
}

/// 레이아웃 요소 종류
#[derive(Debug, Clone, serde::Serialize)]
pub enum LayoutKind {
    /// 선택된 한컴 글리프와 측정에 사용한 같은 전진폭.
    HancomGlyphs(super::hancom_glyphs::GlyphRun),
    /// 수평 나열
    Row(Vec<LayoutBox>),
    /// 일반 텍스트 (이탤릭)
    Text(String),
    /// 숫자
    Number(String),
    /// 기호
    Symbol(String),
    /// 수학 기호 (Unicode)
    MathSymbol(String),
    /// 실제 선택한 대체 글꼴과 전진폭을 함께 전달하는 표시 글리프.
    FontGlyph {
        text: String,
        font_family: &'static str,
        /// 표시 글꼴의 차이를 배치 단계에서 해소한 공통 공개 윤곽.
        outline: Option<Vec<super::GlyphOutlineCommand>>,
    },
    /// 함수 이름 (로만체)
    Function(String),
    /// 분수
    Fraction {
        numer: Box<LayoutBox>,
        denom: Box<LayoutBox>,
        /// 측정에서 정한 기준선 위 수학 축. 각 출력 경로가 그대로 소비한다.
        axis_height: f64,
    },
    /// 위아래 배치 (분수선 없음)
    Atop {
        top: Box<LayoutBox>,
        bottom: Box<LayoutBox>,
    },
    /// 제곱근
    Sqrt {
        index: Option<Box<LayoutBox>>,
        body: Box<LayoutBox>,
    },
    /// 위첨자
    Superscript {
        base: Box<LayoutBox>,
        sup: Box<LayoutBox>,
    },
    /// 아래첨자
    Subscript {
        base: Box<LayoutBox>,
        sub: Box<LayoutBox>,
    },
    /// 위·아래첨자
    SubSup {
        base: Box<LayoutBox>,
        sub: Box<LayoutBox>,
        sup: Box<LayoutBox>,
    },
    /// 큰 연산자
    BigOp {
        symbol: String,
        op_font_size: f64,
        op_width: f64,
        op_x: f64,
        op_y: f64,
        op_baseline: f64,
        sub: Option<Box<LayoutBox>>,
        sup: Option<Box<LayoutBox>>,
    },
    /// 극한
    Limit {
        is_upper: bool,
        /// 한컴 이름 글리프와 측정에 사용한 같은 메트릭.
        name: Option<Box<LayoutBox>>,
        name_font_size: f64,
        sub_font_size: f64,
        sub: Option<Box<LayoutBox>>,
    },
    /// 행렬
    Matrix {
        cells: Vec<Vec<LayoutBox>>,
        style: MatrixStyle,
    },
    /// 관계식 (REL/BUILDREL) — 화살표 위/아래 내용
    Rel {
        arrow: Box<LayoutBox>,
        over: Box<LayoutBox>,
        under: Option<Box<LayoutBox>>,
    },
    /// 칸 맞춤 정렬 (EQALIGN)
    EqAlign {
        rows: Vec<(LayoutBox, LayoutBox)>, // (왼쪽, 오른쪽) 쌍
    },
    /// 괄호
    Paren {
        left: String,
        right: String,
        left_width: f64,
        right_width: f64,
        delimiter_height: f64,
        glyph_delimiters: bool,
        body: Box<LayoutBox>,
    },
    /// 장식
    Decoration {
        kind: super::symbols::DecoKind,
        body: Box<LayoutBox>,
    },
    /// 글꼴 스타일
    FontStyle {
        style: super::symbols::FontStyleKind,
        body: Box<LayoutBox>,
    },
    /// 공백
    Space(f64),
    /// 줄바꿈 (세로 쌓기용 마커)
    Newline,
    /// 빈 박스
    Empty,
}

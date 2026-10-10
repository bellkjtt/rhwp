//! 한컴 수학 대문자의 정체 표시 글리프와 독립 Print 기울임 계약.
//! 공개 Latin Modern Math A~Z 윤곽만 사용하며 사설 윤곽·바이트를 복제하지 않는다.
//! 파생 글꼴 출처와 GUST 라이선스는 assets/fonts의 해당 manifest를 따른다.

use std::sync::LazyLock;

use super::GlyphOutlineCommand as Command;

static FACE: LazyLock<ttf_parser::Face<'static>> = LazyLock::new(|| {
    ttf_parser::Face::parse(
        include_bytes!("../../../assets/fonts/LatinModernMath-UppercaseOutline-subset.otf"),
        0,
    )
    .expect("공개 대문자 윤곽 subset은 유효한 OpenType 글꼴이다")
});

// UPM 1024, HYhwpEQ cmap U+E000..U+E019의 em 잉크 프레임.
// 설치 원본 SHA-256: a3a3cd992a89ac38e7707b58143ebdb1da9511d3c462f019cda7f92c30467807.
// 독립 Print의 해당 glyph와 메트릭·기울임을 대조한다. 화면 좌표가 아니다.
const TARGET: [[f64; 4]; 26] = [
    [32.0, 2.0, 734.0, 735.0],               // A
    [37.0, 2.0, 666.0, 702.0],               // B
    [57.0, -20.0, 681.0, 724.0],             // C
    [36.0, 2.0, 723.0, 702.0],               // D
    [34.0, 2.0, 668.0, 702.0],               // E
    [34.0, 2.0, 625.0, 702.0],               // F
    [57.0, -20.0, 753.0, 724.0],             // G
    [34.0, 2.0, 734.0, 702.0],               // H
    [29.0, 2.0, 341.0, 702.0],               // I
    [42.0, -20.0, 476.0, 702.0],             // J
    [34.0, 2.0, 754.0, 702.0],               // K
    [34.0, 2.0, 596.0, 702.0],               // L
    [38.0, 2.0, 899.0, 703.0],               // M
    [34.0, 0.304347826086957, 734.0, 702.0], // N
    [57.0, -20.0, 738.0, 724.0],             // O
    [36.0, 2.0, 639.0, 702.0],               // P
    [57.0, -197.0, 744.0, 724.0],            // Q
    [36.0, -20.0, 749.0, 702.0],             // R
    [56.0, -20.0, 511.0, 724.0],             // S
    [37.0, 2.0, 701.0, 702.0],               // T
    [34.0, -20.0, 734.0, 702.0],             // U
    [20.0, -20.0, 748.0, 702.0],             // V
    [19.0, -20.0, 1033.0, 702.0],            // W
    [25.0, 2.0, 743.0, 702.0],               // X
    [11.0, 2.0, 755.0, 702.0],               // Y
    [56.0, 2.0, 573.0, 702.0],               // Z
];

struct Builder {
    commands: Vec<Command>,
    source: ttf_parser::Rect,
    target: [f64; 4],
    scale: f64,
    origin: f64,
    current: (f64, f64),
    contour_start: (f64, f64),
}

impl Builder {
    fn point(&self, x: f32, y: f32) -> (f64, f64) {
        let x = self.target[0]
            + (f64::from(x) - f64::from(self.source.x_min)) * (self.target[2] - self.target[0])
                / f64::from(self.source.x_max - self.source.x_min);
        let y = self.target[1]
            + (f64::from(y) - f64::from(self.source.y_min)) * (self.target[3] - self.target[1])
                / f64::from(self.source.y_max - self.source.y_min);
        // 대문자는 정체 윤곽에 Print의 기울임을 한 번 적용한다.
        // 실제 이탤릭 윤곽인 소문자에는 이 변환을 적용하지 않는다.
        (self.scale * (x + 0.333 * y), self.origin - self.scale * y)
    }
}

impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.current = self.point(x, y);
        self.contour_start = self.current;
        self.commands.push(Command::Move(self.current));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.current = self.point(x, y);
        self.commands.push(Command::Line(self.current));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let control = self.point(x1, y1);
        let end = self.point(x, y);
        let first = (
            self.current.0 + (control.0 - self.current.0) * 2.0 / 3.0,
            self.current.1 + (control.1 - self.current.1) * 2.0 / 3.0,
        );
        let second = (
            end.0 + (control.0 - end.0) * 2.0 / 3.0,
            end.1 + (control.1 - end.1) * 2.0 / 3.0,
        );
        self.commands.push(Command::Cubic([first, second, end]));
        self.current = end;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let first = self.point(x1, y1);
        let second = self.point(x2, y2);
        let end = self.point(x, y);
        self.commands.push(Command::Cubic([first, second, end]));
        self.current = end;
    }

    fn close(&mut self) {
        self.commands.push(Command::Close);
        self.current = self.contour_start;
    }
}

pub(super) fn commands(ch: char, fs: f64, dpi: f64, baseline: f64) -> Option<Vec<Command>> {
    if !ch.is_ascii_uppercase() {
        return None;
    }
    let glyph = FACE.glyph_index(ch)?;
    let source = FACE.glyph_bounding_box(glyph)?;
    if source.x_max <= source.x_min || source.y_max <= source.y_min {
        return None;
    }
    let mut builder = Builder {
        commands: Vec::new(),
        source,
        target: TARGET[(u32::from(ch) - u32::from('A')) as usize],
        scale: fs / 1024.0,
        origin: baseline - super::negation_geometry::math_origin_shift(fs, dpi),
        current: (0.0, 0.0),
        contour_start: (0.0, 0.0),
    };
    FACE.outline_glyph(glyph, &mut builder)?;
    (!builder.commands.is_empty()).then_some(builder.commands)
}

//! HYhwpEQ의 논리 폰트 프레임 메트릭. 사설 폰트 바이트를 포함하지 않는다.
//!
//! 0 크기·0 기준선·저장 줄 없음으로 작성한 98개 수식을 MCP 2020에서
//! 재생성한 독립 메트릭이다. 한컴 11.0.0.9136, job:
//! 909d6081-12ef-42be-8982-9d653aa1c464.
//! 생성 HWP SHA-256: f184279c6ec6d64665ba6d06814fd72673daa2fe26447631895dfa4c7103c223.
//! 입력·산출·Print 및 미지원 범위는 pr_7600_review_impl.md의 진단 기록을 따른다.
//! 글자 크기는 폰트 메트릭의 축이다. 지원하지 않는 크기를 문서 값에 맞춰 추정하지 않는다.

#[derive(Clone, Copy)]
pub(super) struct FontFrameMetrics {
    pub line: (f64, f64),
    pub prime_line: (f64, f64),
    pub bar: (f64, f64),
    pub prime_bar: (f64, f64),
    pub sup: (f64, f64),
    pub fraction: (f64, f64),
}

// BaseUnit, 각 구조의 높이(HWPUNIT)와 기준선(%). 픽셀 위치를 고정하지 않는다.
const METRICS: [[u32; 13]; 14] = [
    [600, 600, 86, 600, 86, 720, 88, 720, 88, 685, 87, 1380, 66],
    [700, 675, 87, 700, 86, 810, 89, 835, 88, 788, 89, 1560, 66],
    [800, 825, 84, 825, 84, 990, 87, 990, 87, 951, 87, 1890, 65],
    [
        900, 900, 86, 900, 86, 1080, 88, 1080, 88, 1050, 88, 2070, 66,
    ],
    [
        1000, 975, 86, 1000, 86, 1170, 89, 1195, 88, 1163, 89, 2250, 66,
    ],
    [
        1100, 1125, 85, 1125, 85, 1350, 87, 1350, 87, 1313, 87, 2580, 65,
    ],
    [
        1200, 1200, 86, 1200, 86, 1440, 88, 1440, 88, 1425, 88, 2760, 66,
    ],
    [
        1300, 1275, 86, 1300, 86, 1530, 89, 1555, 88, 1538, 89, 2940, 66,
    ],
    [
        1400, 1425, 85, 1425, 85, 1710, 87, 1710, 87, 1688, 87, 3270, 65,
    ],
    [
        1500, 1500, 86, 1500, 86, 1800, 88, 1800, 88, 1800, 88, 3450, 66,
    ],
    [
        1600, 1575, 86, 1600, 86, 1890, 88, 1915, 88, 1913, 89, 3630, 66,
    ],
    [
        1800, 1800, 86, 1800, 86, 2160, 88, 2160, 88, 2100, 88, 4140, 66,
    ],
    [
        2000, 2025, 85, 2025, 85, 2430, 88, 2430, 88, 2363, 87, 4650, 66,
    ],
    [
        2400, 2400, 86, 2400, 86, 2880, 88, 2880, 88, 2850, 88, 5520, 66,
    ],
];

/// 글자 크기 메트릭의 키는 원래 정수 BaseUnit을 복원한다.
/// 좌표 변환의 절삭을 사용하면 13pt가 부동소수점 오차로 1299HU가 된다.
pub(super) fn base_unit_at_dpi(fs: f64, dpi: f64) -> u32 {
    (fs * crate::renderer::HWPUNIT_PER_INCH / dpi).round() as u32
}

pub(super) fn for_size(fs: f64) -> Option<FontFrameMetrics> {
    let hu = base_unit_at_dpi(fs, crate::renderer::DEFAULT_DPI);
    let row = METRICS.iter().find(|row| row[0] == hu)?;
    let convert = |index: usize| {
        let height =
            crate::renderer::hwpunit_to_px(row[index] as i32, crate::renderer::DEFAULT_DPI);
        (height, height * f64::from(row[index + 1]) / 100.0)
    };
    Some(FontFrameMetrics {
        line: convert(1),
        prime_line: convert(3),
        bar: convert(5),
        prime_bar: convert(7),
        sup: convert(9),
        fraction: convert(11),
    })
}

pub(super) fn for_size_at_dpi(fs: f64, dpi: f64) -> Option<FontFrameMetrics> {
    let reference_dpi = crate::renderer::DEFAULT_DPI;
    let metrics = for_size(fs * reference_dpi / dpi)?;
    let convert = |(height, baseline): (f64, f64)| {
        (height * dpi / reference_dpi, baseline * dpi / reference_dpi)
    };
    Some(FontFrameMetrics {
        line: convert(metrics.line),
        prime_line: convert(metrics.prime_line),
        bar: convert(metrics.bar),
        prime_bar: convert(metrics.prime_bar),
        sup: convert(metrics.sup),
        fraction: convert(metrics.fraction),
    })
}

/// 일반 글줄의 수학 축을 글자 크기의 논리 프레임에서 구한다.
pub(super) fn line_at_dpi(fs: f64, dpi: f64) -> Option<(f64, f64)> {
    if let Some(metrics) = for_size_at_dpi(fs, dpi) {
        return Some(metrics.line);
    }
    // 기존 구조별 표에 없는 22pt의 일반 글줄만 독립 정상화로 확인했다.
    // job ee04e67a-b8cb-4297-9f26-716027f9eb7e, a/n/x/m 모두 동일하다.
    let hu = base_unit_at_dpi(fs, dpi);
    if hu != 2200 {
        return None;
    }
    let height = crate::renderer::hwpunit_to_px(2175, dpi);
    Some((height, height * 0.86))
}

/// 일반 글줄 본체와 첨자의 논리 프레임. 문서 좌표나 수식별 예외가 아니다.
/// 한컴 2020 정상화 job 57289556-fcb9-4edd-96df-f60e9d26ac53에서
/// a_{n}/x_{m} 두 표현식의 같은 크기 높이·기준선이 모두 일치했다.
pub(super) fn subscript_at_dpi(fs: f64, dpi: f64) -> Option<(f64, f64)> {
    const FRAMES: [[u32; 3]; 15] = [
        [600, 706, 73],
        [700, 805, 73],
        [800, 979, 71],
        [900, 1078, 71],
        [1000, 1177, 72],
        [1100, 1350, 71],
        [1200, 1450, 71],
        [1300, 1548, 71],
        [1400, 1723, 70],
        [1500, 1821, 71],
        [1600, 1920, 71],
        [1800, 2155, 71],
        [2000, 2428, 71],
        [2200, 2626, 71],
        [2400, 2899, 71],
    ];
    let hu = base_unit_at_dpi(fs, dpi);
    let row = FRAMES.iter().find(|row| row[0] == hu)?;
    let height = crate::renderer::hwpunit_to_px(row[1] as i32, dpi);
    Some((height, height * f64::from(row[2]) / 100.0))
}

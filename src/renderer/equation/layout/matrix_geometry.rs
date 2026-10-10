//! 행렬·cases·정렬 행과 세로 묶음의 공유 배치.

use super::{
    EqLayout, EqNode, LayoutBox, LayoutKind, MatrixStyle, PileAlign, MATRIX_COL_GAP, MATRIX_ROW_GAP,
};

impl EqLayout {
    pub(super) fn layout_matrix(
        &self,
        rows: &[Vec<EqNode>],
        style: MatrixStyle,
        fs: f64,
    ) -> LayoutBox {
        if matches!(style, MatrixStyle::Ladder | MatrixStyle::ShortLadder) {
            return self.layout_ladder(rows, style, fs);
        }
        if style == MatrixStyle::Plain && !rows.is_empty() {
            return self.layout_plain_matrix(rows, fs);
        }
        if rows.is_empty() {
            return LayoutBox {
                font_size: fs,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: fs,
                baseline: fs * 0.8,
                kind: LayoutKind::Empty,
            };
        }

        let col_gap = fs * MATRIX_COL_GAP;
        let row_gap = fs * MATRIX_ROW_GAP;

        // 모든 셀 레이아웃
        let mut cell_boxes: Vec<Vec<LayoutBox>> = rows
            .iter()
            .map(|row| row.iter().map(|c| self.layout_node(c, fs)).collect())
            .collect();

        let num_cols = cell_boxes.iter().map(|r| r.len()).max().unwrap_or(0);

        // 열 폭 계산
        let mut col_widths = vec![0.0f64; num_cols];
        for row in &cell_boxes {
            for (ci, cell) in row.iter().enumerate() {
                if ci < num_cols {
                    col_widths[ci] = col_widths[ci].max(cell.width);
                }
            }
        }

        // 행 높이 계산
        let row_heights: Vec<f64> = cell_boxes
            .iter()
            .map(|row| row.iter().map(|c| c.height).fold(fs, f64::max))
            .collect();

        // 셀 위치 배정
        let mut y = 0.0;
        for (ri, row) in cell_boxes.iter_mut().enumerate() {
            let rh = row_heights[ri];
            let mut x = 0.0;
            for (ci, cell) in row.iter_mut().enumerate() {
                let cw = if ci < num_cols {
                    col_widths[ci]
                } else {
                    cell.width
                };
                cell.x = x + (cw - cell.width) / 2.0;
                cell.y = y + (rh - cell.height) / 2.0;
                x += cw + if ci + 1 < num_cols { col_gap } else { 0.0 };
            }
            y += rh + row_gap;
        }

        let total_w: f64 =
            col_widths.iter().sum::<f64>() + col_gap * (num_cols.saturating_sub(1)) as f64;
        let total_h = y - row_gap;
        let bracket_pad = fs * 0.2;

        // 괄호 포함 폭
        let paren_w = match style {
            MatrixStyle::Plain => 0.0,
            _ => fs * 0.3,
        };
        let full_w = total_w + paren_w * 2.0 + bracket_pad * 2.0;

        // 셀 x 오프셋 (괄호 포함)
        let x_offset = paren_w + bracket_pad;
        for row in &mut cell_boxes {
            for cell in row.iter_mut() {
                cell.x += x_offset;
            }
        }

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: full_w,
            height: total_h,
            baseline: total_h / 2.0,
            kind: LayoutKind::Matrix {
                cells: cell_boxes,
                style,
            },
        }
    }

    pub(super) fn layout_cases(&self, rows: &[EqNode], fs: f64) -> LayoutBox {
        // HYhwpEQ의 논리 줄간격: hhea/OS2 lineGap 152 / UPM 1024.
        // 일반 cases 글자 줄의 독립 Print 기준선 간격도 이 leading과 일치한다.
        let row_gap = fs
            * if self.hancom_math_face {
                152.0 / 1024.0
            } else {
                MATRIX_ROW_GAP
            };
        let mut row_boxes = self.layout_cases_rows(rows, fs);

        let mut y = 0.0;
        for b in &mut row_boxes {
            b.x = fs * 0.3; // 왼쪽 중괄호 여백
            b.y = y;
            y += b.height + row_gap;
        }
        let total_h = y - row_gap;
        // 실제 행 원점과 점유 끝을 폭으로 공유한다. 중괄호의 예약은
        // 아래 Paren의 body.x에서 한 번 소비하며, 빈 오른쪽 여백을
        // Row 폭에 다시 넣으면 뒤 본문과 delimiter가 그만큼 밀린다.
        let full_w = row_boxes
            .iter()
            .map(|row| row.x + row.width)
            .fold(0.0f64, f64::max);
        // 행 구성과 관계없이 묶음의 수학 축을 가운데에 놓는다. 왼쪽 식의
        // 기준선은 일반 글줄의 축 거리만큼 내려간다. 분수 행을 앞뒤로
        // 옮겨도 두 글자 기준선의 평균으로 축이 바뀌지 않는다.
        let axis_offset = if self.hancom_math_face {
            super::super::font_frame_metrics::line_at_dpi(fs, self.dpi)
                .map_or(fs * 0.3, |(height, baseline)| baseline - height / 2.0)
        } else {
            0.0
        };
        let baseline = total_h / 2.0 + axis_offset;

        // 중괄호 포함 레이아웃 → Paren으로 래핑
        let inner = LayoutBox {
            font_size: fs,
            // 외곽 폭에 예약한 왼쪽 중괄호 폭을 실제 자식 원점에서도 소비한다.
            x: fs * 0.3,
            y: 0.0,
            width: full_w,
            height: total_h,
            baseline,
            kind: LayoutKind::Row(row_boxes),
        };

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: full_w + fs * 0.3,
            height: total_h,
            baseline,
            kind: LayoutKind::Paren {
                left: "{".to_string(),
                right: String::new(),
                left_width: fs * 0.3,
                right_width: 0.0,
                delimiter_height: total_h,
                glyph_delimiters: false,
                body: Box::new(inner),
            },
        }
    }

    pub(super) fn layout_eqalign(&self, rows: &[(EqNode, EqNode)], fs: f64) -> LayoutBox {
        let row_gap = fs * MATRIX_ROW_GAP;
        let align_gap = fs * 0.15; // & 기준 좌우 사이 간격

        // 각 행의 왼쪽/오른쪽 레이아웃 계산
        let mut laid_rows: Vec<(LayoutBox, LayoutBox)> = rows
            .iter()
            .map(|(l, r)| (self.layout_node(l, fs), self.layout_node(r, fs)))
            .collect();

        // 왼쪽 최대 폭 (& 정렬 기준)
        let max_left_w = laid_rows
            .iter()
            .map(|(l, _)| l.width)
            .fold(0.0f64, f64::max);

        let mut y = 0.0;
        let mut total_w = 0.0f64;
        for (left, right) in &mut laid_rows {
            // 왼쪽: 오른쪽 정렬 (& 기준으로 맞춤)
            left.x = max_left_w - left.width;
            // 오른쪽: & 기준 바로 뒤
            right.x = max_left_w + align_gap;

            let row_h = left.height.max(right.height);
            let row_bl = left.baseline.max(right.baseline);
            // 베이스라인 정렬
            left.y = y + (row_bl - left.baseline);
            right.y = y + (row_bl - right.baseline);

            total_w = total_w.max(right.x + right.width);
            y += row_h + row_gap;
        }
        let total_h = (y - row_gap).max(0.0);

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: total_w,
            height: total_h,
            baseline: total_h / 2.0,
            kind: LayoutKind::EqAlign { rows: laid_rows },
        }
    }

    pub(super) fn layout_pile(&self, rows: &[EqNode], align: PileAlign, fs: f64) -> LayoutBox {
        let row_gap = fs * MATRIX_ROW_GAP;
        let mut row_boxes: Vec<LayoutBox> = rows.iter().map(|r| self.layout_node(r, fs)).collect();

        let max_w = row_boxes.iter().map(|b| b.width).fold(0.0f64, f64::max);
        let mut y = 0.0;
        for b in &mut row_boxes {
            b.x = match align {
                PileAlign::Left => 0.0,
                PileAlign::Center => (max_w - b.width) / 2.0,
                PileAlign::Right => max_w - b.width,
            };
            b.y = y;
            y += b.height + row_gap;
        }
        let total_h = y - row_gap;

        LayoutBox {
            font_size: fs,
            x: 0.0,
            y: 0.0,
            width: max_w,
            height: total_h,
            baseline: total_h / 2.0,
            kind: LayoutKind::Row(row_boxes),
        }
    }
}

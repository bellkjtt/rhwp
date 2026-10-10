# Stage 95 — 수식의 공통 줄 프레임과 늘림 중괄호 표시

Issue: #7642. 변경 전 source는 `5654d84514780e7b07f15b9d7c1bb3bb099c03d0`다.
55개 원본 853쪽과 91개 요청 경로, #1139 전체 관계형 전환 범위를 유지한다.
수학 두 원본의 Skia 40쪽은 90% 이상이지만 확대 판독의 중괄호 차이와
독립 36개 진단의 90% 미만은 실제 보정을 계속할 사유다. 정식 회귀는 아직 바꾸지 않는다.

## 독립 근거와 범위

Stage94 정상화의 모든 EQALIGN 높이는 일반 줄 1125HU/2175HU, 빈 줄 1100HU/2200HU,
독립 정렬 묶음의 leading 110HU/220HU로 설명된다. 첫 기준선도 일반 줄의
85%/86% 프레임을 사용한다. 줄의 자연 표시 상자를 그 논리 프레임으로 대신하지 않는다.
첫 빈 줄은 독립 빈 수식의 86% 기준선을 따른다. 실제 정렬 시 자식 glyph 원점도
같은 줄 프레임으로 옮기며 parent 높이만 수정하지 않는다.

`stage95-brace-frame-probe/`의 8개 수식은 같은 크기의 CASES 한/두 행과
LEFT/RIGHT 중괄호 한/두 행이다. 변환 전후 개수·스크립트·BaseUnit과 정상화 크기를 확인했다.
11pt의 CASES 폭은 1125HU, 중괄호 쌍은 1650HU이며 x의 전진은 600HU다.
22pt에서는 각각 2400HU/3525HU이고 x 전진은 1275HU다.
중괄호 슬롯은 각각 525HU/1125HU이고 한/두 행 모두 같다.
원본 글자 크기의 반각 슬롯을 기존 75HU 전진 격자로 반올림하면 이 값과 일치한다.
CASES 안쪽 두 곳에 0.3em을 예약하던 처리를 한 슬롯의 원점·폭으로 공유한다.

독립 Print subset에서 일반 중괄호와 조립 부품의 전진·ink bbox만 읽었다.
private 글꼴 바이트나 윤곽을 저장·커밋하지 않는다. 표시에는 이미 배포한 공개
Latin Modern Math의 MATH 늘림 variant/assembly를 사용한다. 원본 asset SHA-256과
GUST Font License를 생성 데이터에 남긴다. glyph 크기 선택과 연결부 겹침은
MATH의 variant 높이·PartRecords·MinConnectorOverlap으로 계산하며 문서 좌표에 맞추지 않는다.

## 생산 결과와 실제 소비

공통 행 helper가 각 칸의 논리 높이·기준선을 만들고 EQALIGN/CASES가 같은 결과로
행 점유, 첫 행 기준선과 실제 자식 y를 배치한다. 빈 줄과 빈 오른쪽 칸은 구분한다.
일반 font frame이 없는 구조는 기존 자연 상자 계약을 사용한다.
CASES는 하나의 중괄호 슬롯을 예약하며 본체 원점에서 한 번 소비한다.
검증된 HYhwpEQ 중괄호 쌍에도 같은 슬롯을 적용한다. 혼합 구분자의 미검증 폭은 추정하지 않는다.

늘림 중괄호 공통 helper는 공개 glyph의 filled outline을 산출한다.
SVG·Rust Canvas·Skia의 중괄호 경로가 그 동일한 명령을 소비한다.
행 높이와 본체 위치를 paint에서 다시 계산하지 않으며 중괄호 때문에 뒤 내용을 숨기거나 clamp하지 않는다.
작은 variant와 큰 assembly를 같은 font 데이터로 선택한다. 일반 brace 문자와 다른 괄호는 별도 계약이다.

## 검증

한 번의 보정 뒤 포맷·Native/WASM Clippy·Skia check·Native 빌드를 순차 실행한다.
독립 36개와 새 8개, 수학 두 원본 전체를 직접 대조하고 같은 영역의 새 review PNG를 연다.
한 행/여러 행, 일반/조립 glyph와 빈 줄의 경계를 구분해 관측값을 기록한다.
같은 최종 source의 전체 Native/fresh WASM TSV와 확대 판독을 선행한 뒤 관계형 회귀를 변경한다.
전체 nextest·workspace/all-target lint·정책 검사와 PR lifecycle은 여전히 남은 단계다.


## 구현 후보

`matrix_row_frame`에서 셀의 논리 프레임을 공유한다. 일반 줄과 빈 줄을 구분하고,
프레임 기준선과 glyph의 자연 기준선 차이를 실제 자식 y에 반영한다.
`matrix_cell_frame → EQALIGN/CASES 행 점유·첫 기준선 → 자식 원점`이 같은 값을 소비한다.
22pt의 일반 글줄은 앞 단계에서 독립 확인한 `line_at_dpi`를 쓰며, 별도 prime 프레임을
일반 줄로 추정하지 않는다. 행이 보인다는 이유로 명목 fs까지 높이를 강제하지 않는다.

괄호 배치를 `paren_geometry.rs`로 분리해 layout 본체를 1000줄 이하로 유지했다.
HYhwpEQ의 검증된 중괄호 슬롯은 반각 전진 격자를 사용하며 한/두 행과 쌍의 폭에 대응한다.
일반 face는 기존 두 0.3em이 만든 0.6em 본체 원점·전체 폭을 한 슬롯으로 보존한다.
CASES 행의 추가 x 여백을 제거해 슬롯을 본체 원점에서 한 번만 소비한다.

`brace_stretch_data.rs`는 기존 공개 WOFF2의 8개 variant와 조립 부품 4개를
fontTools RecordingPen으로 추출했다. 새 글꼴 asset·사설 윤곽은 추가하지 않았다.
`brace_stretch::commands`는 glyph 높이와 슬롯 폭, variant/assembly 정보로
filled outline을 만들며 SVG·Canvas·Skia가 같은 명령을 그린다.
큰 중괄호는 위/아래 extender 수를 같게 하고 최소 연결부 겹침과 연결 가능한 길이를 확인한다.
작은 중괄호는 제공된 variant를 사용한다.
[OpenType MATH 규약](https://learn.microsoft.com/en-us/typography/opentype/spec/math)의
MathVariants·GlyphAssembly·GlyphPartRecord 계약을 대조한다. 규약은 단일 배치 알고리즘을
정하지 않으므로 이 구현의 실제 한컴 일치는 독립 Print와 재출력으로 별도 검증한다.

현재 후보는 아직 lint·빌드·시각 검증 전이다. 정식 회귀 추가·변경은 없다.

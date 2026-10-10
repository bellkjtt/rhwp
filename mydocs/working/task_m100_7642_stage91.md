# Stage 91 — 한 줄 미주 수용과 셀 도형의 기준 좌표계

## 독립 근거와 묶음 수정 계획

Issue: #7642. 변경 전 source는 `b770b1d2e01ddc3570771c313f9e952a8df4e97a`다.
Stage 90에서 독립 Print 전체 191쪽과 직접 Skia를 대조한 9개 원본 중
all-21/39/41만 미달이다. 전체 55개 원본/853쪽과 관계형 회귀 전환 범위를 유지한다.

### 한 줄 미주의 실제 수용

all-21 HWPX의 미주 내부 pi864는 앞 항목을 포함한 실제 점유가 가용 단 높이
안이다. 자매 HWP와 독립 Print 모두 현재 왼쪽 단에 남긴다. 앞선 분기에서
사용하는 저장 좌표 추정과 실제 결과가 다르며, 실제 query가 한 줄을 제외해
오래된 추정이 단 넘김을 결정한다. 원본과 PDF 해시는 Stage 90 receipt에 고정했다.

`query_reflowed_endnote_fit`은 비어 있지 않은 한 줄도 실제 점유로 Whole/Overflow를
반환한다. 다줄의 Split, 명시적 나눔/keep 그룹/별도 객체 경로는 유지한다.
`initial_fit → measure_endnote_column_with_item → 선행 tail 분기 → 최종 방출`에서
같은 단·같은 앞 항목 목록인 결과만 재사용한다. 빈 단, 새 미주 머리,
실제로 넘치는 한 줄, float, keep-with-next는 각각 기존 별도 계약을 따른다.

### 셀 도형의 PAPER/PAGE 축

`samples/exam_math.hwp` 구역1 문단68의 셀 쪽 번호 도형은 비-TAC,
IN_FRONT_OF_TEXT, 가로/세로 PAPER 기준, 가로 RIGHT·세로 TOP 정렬이다.
선언 크기 4252HU, 가로 offset 5201HU, 세로 offset 5853HU다.
독립 Print 17쪽은 같은 번호를 용지 오른쪽 위에 출력한다. 현재는 셀 원점과
셀 폭을 사용해 본문 쪽으로 이동한다. 이 값은 원본 진단이며 고정 px 회귀값이 아니다.

`build_render_tree/바탕쪽/단 fast probe → 현재 용지·본문 frame →
layout_cell_shape_with_parent_path → layout_shape_object → paint`를 대조했다.
가로 PAPER는 실제 용지 폭, PAGE는 본문 frame을 사용한다. 세로 PAPER는
실제 용지 높이, PAGE는 본문 frame을 사용한다. PARA/COLUMN과 TAC의
기존 셀/문단 흐름, CellPath·객체 layer는 유지한다. 비대칭 여백에서도
용지 폭을 본문 폭·여백으로 역산하지 않는다. 바탕쪽이 없는 머리말도 현재
페이지 frame을 소비하도록 페이지 시작에 먼저 공급하고 기존 fast probe와 공유한다.

## 검증 계획과 미검증 범위

두 원인을 한 배치로 수정하고 fmt·Native/WASM Clippy·Skia check를 순차 실행한다.
동결 Native binary로 앞선 실패 원본 9개를 전쪽 재비교한다. 미주 컷과
쪽 번호/제목을 같은 배율로 직접 판독하고 exam_eng/kor/science/social 및
셀 PARA/TAC 경로를 대조군으로 확인한다. 방향 확인 뒤 Native SVG/fresh WASM
전쪽 TSV와 공식 회귀 게이트를 진행한다.

Native/fresh WASM 90% 이상 및 확대 판독 전에 정식 회귀를 추가하거나
기대값을 변경하지 않는다. source fixture/PDF를 삭제해 통과시키지 않는다.
이 계획 시점에는 구현·빌드·시각 결과 모두 미검증이다.

## 구현 기록

`reflow_fit.rs`는 한 줄을 제외하지 않고 기존 실제 점유 조회로 Whole/Overflow를
반환한다. `paragraph.rs`의 선행 분기도 단과 앞 항목 수 일치를 확인한다.
`layout.rs`는 물리 용지·본문 frame 공급을 공통 helper로 모으고 페이지 시작·
바탕쪽·단 probe에서 같은 값을 사용한다. `table_cell_content.rs`는 비-TAC
도형의 PAPER/PAGE 축을 선언대로 해석한다.

`cargo fmt --all`과 `git diff --check`는 통과했다. 코드 source를 동결한 뒤
선행 타입 검사와 Native 빌드·원본 직접 비교를 순차 실행한다. 다른 필수
회귀·fresh WASM·전쪽 TSV는 아직 미검증이며 정식 테스트 기대값은 바꾸지 않았다.

# 이슈 #7642 — 원본 글자 크기 0과 떠 있는 수식의 배치 계약

이전 단계: [저장 조판과 배치 보정](task_m100_7642_stage7.md).
시작 head: `ecfc9361d458eb2a44bd727320090420a35e5173`.
실패 원본·시험지·수식 37개·첨부 회귀 목록·#1139 전체 범위를 유지한다.

## 독립 근거와 현재 차이

- 사용자가 제공한 HML 원본의 한컴 2024 수동 Print PDF는 유효한 기준이다.
- 한컴 HWP/HWPX 저장본은 실제 제품 2020 / 11.0.0.9136이며 각 원본의 2020 MCP
  Print도 확보했다. 두 저장본 모두 상대 크기·장평 0, 문단 줄간격 0%를 저장했다.
- 현재 Native는 HWP 6.34209%, HWPX 16.96783%로 미충족이다. RHWP가 0 크기를
  12px 기본 크기로 바꾸어 그리는 본문과 네 수식의 위치 차이를 직접 확인했다.
- HWPML 표 36/38의 정상 속성 기본값은 100이다. 자식 요소 전체가 없는 원본의
  호환 해석과 정상 저장의 스키마 기본값을 구분한다. 이를 이유로 전역 기본값을 0으로
  바꾸거나 정확한 기존 저장 계약 검사를 완화하지 않는다.

## 생산·측정·배치·출력 경로

글자 크기는 `CharShape.relative_sizes → ResolvedCharStyle.font_sizes →
resolved_to_text_style → TextStyle`에서 생산한다. 여기까지 실제 0을 보존하지만
`text_measurement::style_params`와 SVG·Canvas·Skia에서 0을 미지정으로 취급해
12px로 덮어쓴다. 미지정 스타일과 원본에서 해소한 0을 구분하는 상태를 전달하고,
측정·각 출력이 동일한 유효 크기를 소비하도록 보정한다. 0을 검출했다고 글자·문단
노드나 줄 상자를 삭제하지 않는다. 한컴 PDF의 작은 글리프는 Print 장치의 최소 크기
처리도 포함하므로 이 변경만으로 직접 확대 판독까지 완료했다고 주장하지 않는다.

떠 있는 수식은 `CommonObjAttr → format/typeset 흐름 → ShapeItem 원점 →
layout_shape`의 실제 소비를 추적한다. 현재 `layout_shape`의 Equation 단축은
공통 위치 계약을 사용하지 않고 문단 정렬과 `para_y`만으로 좌표를 정한다.
선언 프레임, 원점, 실제 수식 내용 크기를 구분하고 측정·배치가 같은 소유를 사용하는지
확인한다. HML의 명시적인 SHAPEOBJECT도 저장 속성과 같은 계약으로 읽어야 한다.

## 검증과 커밋 순서

1. 원본과 0/1/100 대조군의 Print 관측, 저장 줄의 유효성을 확인한다.
2. 독립 근거가 있는 생산 경로를 보정하고 Native/fresh WASM으로 영향 범위를 재검증한다.
3. 관련 전체 페이지 최저 90%와 확대 판독을 충족한 뒤에만 회귀 검사를 관계형으로 변경한다.
4. 보정과 실제 검증 기록을 일반 중간 커밋으로 고정한다. 진단 TSV·JSON·로그·nextrun은
   ignored output에만 보존한다. 전체 회귀·lint·base 정책과 PR·CI·merge·후속 처리는
   원래 범위를 유지하며 아직 완료되지 않았다.

## 첫 생산 출력과 공통 소비 지점 보완

첫 Native 빌드는 exit 0(5분 1초)이며 frozen `rhwp-explicit-zero-size`의 SHA는
`243ad857c348c86d8f0a80063b2e5cb95bf7fb27abd383cf8f6f804678be0ebd`이다.
이 source의 HWP는 6.34209→30.36053%, HWPX는 16.96783→100%다.
출력 크기와 원점을 고정하지 않고 해소된 0을 유지한 결과다. HWP의 네 수식은
기준보다 넓게 떨어져 있으므로 전체 개선 완료로 판정하지 않는다.

576dpi 같은 물리 영역 대조에서 HWPX의 위첨자 2와 +가 기준보다 낮은 것을
확인했다. 100% 관용 점수에도 확대 판독은 미충족이다. 수식의 표시 글꼴/메트릭과
위첨자 이동·수학 축을 실제 경로에서 분석한다. 본문의 0 크기와 한컴 Print의 최소
글리프 크기 차이도 출력 장치의 계약으로 남아 있으며 임의 좌표 보정으로 숨기지 않는다.

그 뒤 HTML·paint JSON·composer의 추가 12px 폴백을 같은 helper로 연결했다.
기본 탭 간격이 0이면 0으로 나누지 않도록 기본 탭 경로의 전진을 분리했다.
명시적인 물리 탭 정지점은 유지한다. 현재 전체 source의 Native Clippy와 WASM
Clippy는 각각 exit 0(1분 15초 / 1분 1초)이며 최종 Native 재출력과 fresh WASM은
추가 실행해야 한다. 첫 출력의 source SHA와 이 추가 수정의 source를 구분한다.

## 생산 보정 커밋과 최종 Native 재출력

보정과 분석 기록은 `4ea557a1d0b4aeab51f8e125390f2a60f4b89aed`로 커밋했다.
그 head의 Native 재빌드는 exit 0(4분 15초)이며 frozen binary SHA는
`ac18fd011ae24aa28b13918d70307da72e60a3e6f0a6c2ae76087d62e7bb595a`이다.
`hml-explicit-zero-final-native-review`의 두 원본 재출력은 HWP 30.36053%,
HWPX 100%로 첫 측정과 동일하다. HWP 미달과 HWPX 확대 판독 미충족은 유지한다.
fresh WASM wrapper는 `stage8-explicit-zero-size-wasm-build.log`로 실행 중이며
아직 통과로 기록하지 않는다. 최종 결과 전에는 Rust source를 수정하지 않는다.

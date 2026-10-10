# Stage 93 — 원본 HFT 메트릭과 굵기 전진·자간의 공통 계산

Issue: #7642. 변경 전 code source는 `dfb7748b51ad6fb0daa7057e26d37930be9e05e3`다.
55개 원본/853쪽, 복원한91파일/292검사, #1139 전체 관계형 전환의 범위를 유지한다.
Stage92의 원본9개/191쪽과 대조군5개/51쪽 직접 Skia는 완료됐다. 수학 두 원본은
최저86.08289%/85.26152%로 미달이다. 현재 단계에서 회귀·기대값은 바꾸지 않는다.

## 독립 근거와 보정 범위

`pdf/issue7642-hft-char-style-diagnostic/`의 두 한컴 정상 저장 HWP와 Print PDF를
사용한다. MCP engine2020, Hancom11.0.0.9136, method0,
`hancom2020_pdf_driver_one_up`을 실제 result에서 확인했다. 출처·job·입출력 해시는
Stage92 기록과 ignored `stage92-char-style-probe/`에 있다. 원본 시험지의 Print는 유지한다.

정체 ASCII94자씩의 Type3 /Widths와 ToUnicode를 읽어 신명 디나루·신명 신그래픽의
고유 폭을 확보했다. 숫자는 각각660/1000em,620/1000em이며 괄호는500/1000em이다.
TTF 대체 face를 표시 글꼴로 쓰더라도 원본 HFT의 폭을 버릴 근거가 없다.
이 고유 프로그램 폭은 HFT로 해소한 두 face에만 공급한다. 같은 이름의 TTF,
다른 HFT face와 기존 HWP3 반각 판단을 임의로 같은 프로그램으로 취급하지 않는다.

22행은 위치·자간·굵기·크기만 바꿨고 한컴 저장 뒤 속성이 모두 보존됐다.
굵게 해도 intrinsic Type3 /Widths는 같다. 실제 글자 전진에는 약0.05em의 별도
굵기 간격이 생긴다. 2100HU/4100HU, 장평85/92, 자간0/-5/+3/-39에서 확인한다.
원본 수학 제목도 굵기 간격과 원본 자간을 함께 적용하면 기호 사이 원점에 대응한다.
기존 신그래픽의 자간 무시 정책은 음수 자간과 굵기 전진의 상쇄를 분리하지 못했다.
고정 px 좌표·문서 ID·제목 문자열로 맞추지 않고 원본 프로그램·글자 크기·장평·자간으로 계산한다.

## 생산과 실제 소비 경로

`resolve_single_char_style`에서 원본 HFT metric face와 저장 자간을 보존한다.
`char_width_decision`의 intrinsic 폭과 굵기 전진을 분리하고 자간·장평은 그 실제
전진폭에 적용한다. `estimate_text_width_exact/unrounded`, 줄 나눔과
`compute_char_positions`가 이 공통 계산을 사용한다. 탭은 기존 소유·정렬을 유지한다.

`replay_positions_or_compute`의 실제 원점과 `glyph_fit_positions`의 glyph 폭을
구분한다. 독립 PDF의 intrinsic /Widths가 굵기에 따라 늘어나지 않으므로 굵기
간격을 SVG textLength/Canvas fit 폭에 포함하지 않는다. 동일 글자 walk에서
간격을 포함한 배치와 제외한 glyph-fit 결과를 산출한다. 기존 bold TTF·배분 간격·
커닝·첨자의 glyph-fit 계약은 보존한다. 저장 위치가 있으면 원점은 그 값을 소비한다.

## 검증 계약

한 번의 코드 보정 뒤 포맷·Native/WASM Clippy·Skia check·Native 빌드를 순차 수행한다.
새 source의 수학 두 원본과 앞서 영향을 받은 원본7개, 대조군5개를 직접 비교한다.
두 원본의 전체 페이지 수·최저값과 미달 쪽을 기록하고 대표 PNG를 새로 직접 판독한다.
22행 독립 대조군에서 굵기·자간·위치와 glyph 폭의 분리를 따로 확인한다.

대조군의 통과를 원본 시험지의 통과로 바꾸지 않는다. 전체 Native/fresh WASM TSV와
확대 판독 후에만 관련 정식 회귀를 관계형으로 변경한다. 새 helper의 계산값을
그대로 기대값으로 복제하거나 정식 회귀를 삭제하지 않는다. 전체 nextest·workspace/
all-target lint·정책 검사·PR/CI/trailing/merge 후속 처리는 전체 목표의 남은 단계다.

이 문서는 구현 전 계획이다. 구현·검증 완료 증거는 아직 없다.


## 표시 글꼴 소비의 추가 확인

Stage91의 공급 조사에서 Linux에 공식 Gulim이 설치돼 있음을 확인했다.
Skia의 `text_family_candidates`는 쉼표 체인을 전체 이름으로 조회하므로
첫 선언 face와 설치 별칭을 놓친다. 기존 공통 family parser와 설치 별칭을
사용해 원래 순서대로 조회한다. 일반 한국어/영문 family 별칭도 실제 동일
글꼴 이름으로만 연결한다. Dotum과 DotumChe를 같은 face로 취급하지 않는다.
이 수정은 intrinsic HFT 폭과 표시용 설치 face를 혼동하지 않기 위한 소비 보정이다.


## 구현

독립 Type3의95자 폭 표(ASCII94자와 별도22행의 공백400)를
`font_metric_legacy_hft.rs`에 분리했다. 원본 HFT로 해소한 두 face에만
내부 메트릭 이름을 공급하여 같은 이름의 TTF 프로그램과 구분한다.
글리프 윤곽선·개인 글꼴 byte는 넣지 않았다. CJK의 기존 전각 fallback과
HWP3 반각 판정은 기존 문자 측정 분기를 유지한다.

`char_width_decision_with_hft_tracking`은 intrinsic 폭을 보존하고 HFT 굵기
전진0.05em을 원본 자간·장평과 함께 실제 advance에만 더한다.
`estimate_text_width_exact/unrounded → char_width_decision →
compute_char_positions_walk → replay_positions_or_compute`의 실제 원점이
같은 규칙을 사용한다. glyph-fit은 같은 walk에서 그 전진만 제외한다.
`glyph_fit_positions → SVG textLength / Rust Canvas cluster_fit_scale`을 확인했다.
Skia는 cluster glyph를 font 자체의 자연 폭으로 그리고 char_positions를 원점으로
소비하므로 이 추가 간격으로 glyph를 늘리지 않는다. 기존 marker·장식·탭 배경은
배치 전진을 소비한다. generic TTF의 커닝·glyph-fit 경로는 이전 분기를 유지한다.

Skia 후보 함수는 기존 공통 family parser·설치 별칭·base-family 해소를 사용한다.
공통 별칭에 Gulim/GulimChe/Dotum/DotumChe/Batang/BatangChe의 동일 한국어 이름을
추가했다. 실제 조회 대상이 없으면 기존 fallback chain을 이어서 사용한다.
source의 세부 조건과 새 branch는 글꼴 프로그램과 설치 family로 결정하며
문서 번호·본문 문자열·페이지 좌표 조건을 넣지 않았다.

아직 새 source의 lint·빌드·시각 결과는 미검증이다. 정식 회귀 추가/변경은 없다.

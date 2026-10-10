# Stage 92 — HFT 대체 글꼴과 실제 glyph 공급의 분리 진단

Issue: #7642. 변경 전 code source는 `c6ccb1cb64452d444c154a69545bbc90804e395c`다.
Stage 91에서 실패 원본 7개와 대조군 5개는 전쪽 직접 Skia 90% 이상이다.
`samples/exam_math.hwp`, `samples/exam_math_no.hwp`의 글꼴 차이는 남아 있다.
55개 원본/853쪽과 기존 회귀 91파일, #1139 전체 관계형 전환의 범위를 유지한다.

## 독립 관측과 가정

- 같은 원본의 독립 한컴 2020 Print를 유지한다. 원본 구역1 큰 제목의 face는
  `신명 신그래픽`, bold=true, 4100HU, ratio92다. 작은 머리글은 `신명 디나루`,
  bold=false, 2100HU, ratio85다. PDF의 대응 T7/T6는 Type3이며 family 이름을
  복구할 수 없으므로 T7을 임의 TTF와 같은 face라고 주장하지 않는다.
- canonical Native SVG 17쪽은 78.96865%다. 실제 내장 face는 제목의
  Gulim Regular, 작은 머리글의 Noto Sans KR ExtraLight다. 제목에 적용된
  합성 stroke는 0.02em이며 원본의 bold와 같은 결과인지 독립 출력으로 확인해야 한다.
- Linux에는 Gulim과 HYGulim이 설치되어 있다. `굴림`이라는 한국어 family는
  `fc-match`에서 Noto로 떨어진다. SVG의 local 별칭은 Gulim을 알고 있으나
  Skia 후보에는 공통 설치 별칭과 family chain 처리가 누락됐는지 실제 경로를 확인한다.
  이 누락이 현재 두 문서의 모든 차이를 설명한다고 단정하지 않는다.

## 진단 계약

현재 SVG의 원점·장평·본문·수식·줄 구성을 보존한 glyph 공급/합성 굵기 대조군을
ignored output에 만든다. 일반/합성 bold, 실제 설치 face를 각각 분리해 독립 Print와
대조한다. 수정 SVG는 RHWP의 수정 후 실제 출력이나 공식 통과 증거로 사용하지 않는다.
조판 기대값·회귀 테스트·원본·기준 PDF는 바꾸지 않는다.

원본 속성 해소 → 공통 스타일의 측정 메트릭과 paint face → SVG/Skia/Canvas의
실제 공급과 굵기 적용을 추적한다. 설치 별칭의 의미와 실제 글리프 공급 문제,
사양이 필요한 HFT 대체 정책을 구분한다. 독립 근거가 없는 수치 보정이나
특정 문서 ID/제목 문자열 분기는 추가하지 않는다.

이 계획 시점에는 진단과 구현·검증 결과 모두 미검증이다. 적절한 일반 보정이
확인되면 한 배치로 적용하고 선행 타입 검사/Native 빌드, 원본·대조군 직접 비교,
Native/fresh WASM 전쪽 TSV와 확대 판독을 이어간다. 정식 회귀는 해당 선행 조건
통과 뒤 소유·내용 순서·보존·포함 관계로 바꾼다.

## 진단 결과와 실제 누락 속성

`stage92-font-diagnostics/receipt.json`의 6개 대조군은 전체 쪽 78.411~79.754%다.
폰트 교체·합성 굵기 분리만으로 현재 차이를 설명하지 못했다.
다음 소스 추적에서 `CharShape.char_offsets`가 파싱·저장되지만 `ResolvedCharStyle`과
`TextStyle`에는 전달되지 않는 것을 확인했다. 텍스트 stream의 UTF-16 위치를
의미하는 `Paragraph.char_offsets`와는 다른 속성이다.

독립 Print T7의 실제 glyph 기준선은 큰 제목 185.44, 괄호 183.04다.
현재 SVG 기준선에 원본의 10%×4100HU, 7%×3300HU 위치를 적용하면 각각
185.48과 183.04로 대응한다. 이는 원본 비율의 진단 관측이며 고정 px 값으로
회귀를 만들지 않는다. 그 대조군의 전체 17쪽은 85.34440%로 개선됐지만 미달이다.
`stage92-source-offset-diagnostic/receipt.json`은 수정한 glyph와 원본 퍼센트를 기록한다.
대조군을 RHWP 실제 수정 후 출력이나 공식 통과로 보고하지 않는다.

## 구현과 소비 경로

언어별 글자 위치 퍼센트를 `ResolvedCharStyle.font_positions`에 보존하고
언어 슬롯을 선택할 때 `TextStyle.font_position`으로 전달한다. 기본 0은 직렬화에서
생략한다. 원본 글자 위치는 글자 크기에 비례하는 기준선 이동으로 소비한다.

`resolve_single_char_style → resolved_to_text_style → TextStyle.script_draw_metrics →
SVG/HTML/Skia/CanvasTextFont/paint JSON의 실제 glyph 기준선`을 대조했다.
단순 glyph replay에는 위치 변환 계약이 없으므로 그 조건을 명시하고 원본 위치를
소비하는 positioned text 경로를 사용한다. 줄 원점·저장 줄간격·가로 advance는
글리프 위치와 구분한다. font position 0, 양수/음수, 다른 언어 슬롯과 기존
첨자·그림자·회전의 소비도 후속 검증 대상이다. 다른 저장 메트릭이나 자간 정책은
독립 근거를 더 확보한 뒤 판단한다.

## 독립 합성 입력 산출 상태

원본 HFT/CharShape에서 재사용한 22개 글자 위치·자간·굵기 대조 행을 만들었다.
첫 MCP 2020 저장 job `ae9b6820-ad35-44b4-9878-409cdf47ad0b`는 opening_document에서
worker exit3221225477로 terminal failed다. 초기 합성 입력의 미사용 charPr가
기존 template의 다른 언어 font id를 참조하던 잘못을 확인하고 모두 유효한 참조로
고쳤다. 원본 v1과 실패 job 기록을 보존했다. 별도 v2 job
`a9251eac-c0e0-48b2-a37a-171d47455206`도 terminal failed다. 이 입력은 정상 한컴
저장본·Print로 산출되지 않았으며 독립 시각 기준이나 새 정식 fixture가 아니다.
기존 수학 시험지의 정상 Print 증거는 계속 유지한다.

이번 구현은 실제 누락 속성의 복원이며 현재 결과를 90% 통과로 선언하지 않는다.
정식 회귀·기대값은 아직 변경하지 않았다. 선행 검사·Native 빌드와 같은 원본 전체
직접 비교, Native/fresh WASM·전체 회귀의 필수 검증은 이어서 실행한다.

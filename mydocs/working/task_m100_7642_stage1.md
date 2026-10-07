---
kind: plan
status: active
canonical: mydocs/working/task_m100_7642_stage1.md
last_verified: 2026-10-07
---

# 수식 글꼴 측정과 페이지 배치 보정 1단계

Issue: [#7642](https://github.com/edwardkim/rhwp/issues/7642)

## 새 작업의 기준과 범위

- PR #7600은 2026-10-07에 merge commit `22ddc2ab81c405b0c3e9b1f25a397a87de109948`으로 통합됐다.
  원 PR의 `dyad` 기여 변경을 다시 제출하지 않는다. 최신 upstream/devel 위의 새 브랜치와 새 PR에서
  수식·본문 글꼴의 측정, 실제 점유·간격, 원본 페이지 배치의 보정을 진행한다.
- 사용자 승인은 새 브랜치·새 PR, 오늘 할 일 포함, CI 모니터링·조건 충족 후 merge·후속 처리를 포함한다.
  시각 미달·미실행 검증을 완료로 표시하지 않는다. 오늘 할 일은 최종 제출 묶음에서 갱신한다.
- 원본 문서와 수식 96개의 동일 입력·독립 Print PDF로 관련 전체 페이지의 Native/fresh WASM
  최저값 90% 이상, 쪽수 일치 및 수식·간격의 확대 직접 판독을 충족해야 한다.
- `nov-original-resaved-2020.hwp`의 미달 8쪽은 별도 [#7640](https://github.com/edwardkim/rhwp/issues/7640)으로
  분리했다. 원본과 재저장본의 입력·실행·결과를 섞지 않는다. 96개 입력은 진단이라는 이유로 미달을 면제하지 않는다.
- 기존 #1139 파일 전체의 px 고정 회귀 변경 요청은 독립 Print 검증의 선행 조건을 충족한 범위에서
  이어간다. 새 회귀·기대값은 해당 원본을 먼저 보정하고 관련 모든 페이지의 Native/fresh WASM을
  확인한 뒤 소유·순서·포함·겹침 관계로 정한다.

## 실행 순서

1. 이전 로컬 작업과 입력·PDF·증적을 일반 커밋과 백업 브랜치로 보존한다.
2. 최신 upstream/devel을 고정하고 이미 통합된 #7600 원 기여를 제외해 미통합 보정을 새 브랜치로 이관한다.
   원 PR 검토 문서는 upstream의 실제 merge 판정을 보존하며 신규 작업 기록은 이 파일에서 작성한다.
3. 실제 한컴산뜻돋움의 메트릭을 텍스트 측정·TAC 위치·최종 페인트 원점이 공통으로 소비하게 연결한다.
   미등록 다른 face, Regular/Bold 및 원본 수학 문서를 대조군으로 삼는다. 문서 ID·절대 px 보정은 추가하지 않는다.
4. Native의 96개 입력 전체 페이지와 원본·대조군을 먼저 재출력해 방향을 확인한다. 수식 내부의 glyph,
   대체 글꼴, 간격과 프레임 폭도 독립 Print에 맞춰 보정한다. 미달이면 원인을 수정한 뒤 재캡처한다.
5. 새 code head의 fresh WASM을 빌드하고 Native/fresh WASM 전체 TSV와 대표 review·overlay를 직접 판독한다.
   이미 실행한 이전 head의 74.88730% 증적을 새 head의 결과로 사용하지 않는다.
6. 시각 선행 조건 뒤 관계 회귀의 수정 전 FAIL/수정 후 PASS와 정상 대조군, 정식 lint·전체 회귀·Skia
   게이트를 완료한다. HWP·PDF·대표 PNG와 해당 단계 기록을 일반 커밋으로 묶고 TSV·JSON·로그는 output에 둔다.
7. 검증 완료 후 새 PR을 생성하고 실제 PR 번호의 검토·오늘 할 일 묶음을 같은 브랜치에 연결한다.
   정확한 최신 head의 CI 확인·merge·devel 반영·SHA 고정 증적·이슈 상태·소유 브랜치 정리를 순서대로 처리한다.

## 이관 전 진단과 보정 후보

아래 source SHA와 수치는 이관 전 코드에서 수행한 결과다. 새 base와 새 코드의 재검증을 대신하지 않는다.

### 원본 재저장 진단에서 확인한 수식 폭의 변화

- 원본 11월 HWP를 변경하지 않고 MCP 2020의 별도 HWP 재저장으로
  입력의 상자 메트릭을 대조했습니다. job
  `f633635e-aecc-41d9-94d4-08b8bdecf38e`, 한컴 11.0.0.9136, 18초에
  성공했습니다. 이 재저장본은 원본 기준 PDF나 정식 회귀 입력을 대체하지
  않으며 output의 원인 진단 자료입니다.
- owner 340 미주 30 내부 문단 26의 세 수식은 같은 스크립트·BaseUnit
  900·바깥 여백 56HU를 유지하지만 폭이 각각
  11010/4804/3541HU에서 11460/4954/3691HU로 바뀝니다. 높이는
  1125HU에서 900HU로 재발행됩니다. 내부 문단 27도 4804→4954HU,
  내부 문단 28의 첫 수식은 14725→15025HU입니다. 전체 수식을 일률적인
  픽셀 보정으로 맞추지 않고 원본 메트릭과 글리프·script의 소비를 대조합니다.
- 재저장본에서도 해당 문단의 저장 LineSeg는 한 줄로 남습니다. 이
  사실은 새 수식 폭의 저장과 실제 Print 재조판이 같은 작업이 아님을
  보여 줍니다. 재저장본 진단의 통과를 원본 입력의 통과로 승격하지 않으며
  유효 저장 정보·표시 상자·줄 구성의 수용 조건을 계속 확인합니다.

### 수식 96개 및 재저장본의 독립 Print 대조

- 사용자 요청에 따라 수식 폭 진단 HWPX·한컴 재생성 HWP와 대응 Print PDF,
  11월 별도 재저장 HWP와 대응 Print PDF를 함께 보존합니다. 입력 생성 방식과
  전체 해시는 `tests/fixtures/equation-font-metrics/PRIME_WIDTH_DIAGNOSTIC.md`에
  기록했습니다. TSV·MCP 응답·중간 JSON·진단 실행 파일은 ignored output에만 둡니다.
- 수식 96개는 BaseUnit 600/700/900/1000/1200/1800별 16개입니다. HWP 생성 job
  `9f0280ee-6cc2-4429-9bc0-aac469821b95`, Print job
  `a22fb305-56a8-4349-ae25-f337a1860a64`입니다. 한컴 11.0.0.9136의 engine 2020,
  `pdf_print_method=0`, `hancom2020_pdf_driver_one_up`을 확인했습니다.
- 재저장본 Print job은 `7c7c7945-708e-44ba-822c-897a3a8f860a`이며 같은 인쇄 계약을
  사용했습니다. 원본 Print와 재저장 Print의 21쪽을 PDFium 96dpi로 대조했고 모든
  쪽의 RGB 픽셀이 같았습니다. 원본 입력을 재저장본으로 바꾸지는 않았습니다.
- 코드 `3020c479878aaed0d05c8d200f36f2b4b2587807`의 Native와 head
  `8e413838cc573918a18199f4aeb19ba9c5459c30`에서 새로 만든 WASM을 전체 페이지로
  비교했습니다. fresh WASM 빌드는 16분 37초에 완료했으며 pkg와 Studio public의
  SHA-256 일치를 확인했습니다. JS 해시는
  `70cde06a369fa7fd4fc8bc8f3d6acaee158596ba1a116a2c72159002b0b5654e`,
  WASM 해시는 `57f46c50ec5dcca352f71151803c862f729666d367f2d3b09e885a342b3c1111`입니다.

| 입력 | 전체 쪽수 RHWP/PDF | Native 최저 | fresh WASM 최저 | 미달 쪽 |
| --- | --- | ---: | ---: | --- |
| 수식 96개 | 3/3 | 74.88730% | 74.88730% | 1, 2, 3 |
| 11월 재저장본 | 21/21 | 37.71223% | 37.71223% | 8, 11, 12, 17, 18, 19, 20, 21 |
| 기존 원본 11월 | 21/21 | 41.16275% | 41.16275% | 18, 19, 20, 21 |

- 두 진단 입력의 TSV는 `output/pr-review/pr7600/prime-diagnostic-{native,wasm}/`에,
  새 WASM 원본 11월 TSV는 `prime-diagnostic-wasm/nov-original/`에 보존했습니다.
  각각 `re_review_required`입니다. 수식 96개의 2쪽 대표 review·overlay는
  `mydocs/pr/assets/issue_7642_equation_font_metrics/`에 포함합니다.
  수치 미달을 진단 자료라는 이유로 완료 처리하지 않고 이 입력 자체도 보정합니다.
- 사용자 요청에 따라 **재저장본의 8쪽 미달 문제**는 파일명을 명시한
  [이슈 #7640](https://github.com/edwardkim/rhwp/issues/7640)으로 분리했고
  `jangster77`에 할당했습니다. 이 검토에서는 원본과 수식 96개 입력의 비율·간격·쪽
  배치를 계속 개선합니다. 별도 이슈의 통과나 해결 완료를 미리 기록하지 않습니다.

### 한컴산뜻돋움의 실제 측정 메트릭 연결

- 수식 96개 문서의 2쪽 본문은 `한컴산뜻돋움`입니다. 공식 설치본
  `HanSantteutDotum-Regular.ttf`/`-Bold.ttf`가 시스템 글꼴로 등록되어 있음을
  확인했습니다. MCP PDF의 `HanSantteutDotumRegular` subset도 같은 UPM 1000과
  문자 전진폭을 사용합니다. 그런데 RHWP 내장 측정 색인은 해당 family를 찾지 못해
  기본 반각 폭을 발행하고 실제 페인트에는 이 공식 글꼴을 선택했습니다.
- 수정 전 Native에서 본문 접두어 뒤 수식 시작은 x=154.1, Print의 수식은 약
  x=167.1입니다. 이 관측 좌표는 진단 근거이며 회귀 기대값이나 새 보정 상수가 아닙니다.
  측정과 글리프 배치가 같은 실제 전진폭을 소비하게 해야 합니다.
- 새 `font_metric_han_santteut.rs`는 공식 Regular/Bold의 ASCII·Latin-1 cmap/hmtx와
  한글 전진폭 932/1000을 내장 메트릭 색인에 연결합니다. 글꼴 바이트·윤곽은 포함하지
  않으며 특정 파일·문단·쪽·픽셀에 대한 분기는 추가하지 않습니다.
  `font_metrics_data` → 공통 글자 폭 측정 → composer의 텍스트·TAC 위치 →
  paragraph layout의 글리프 원점·수식 원점이 이 폭을 소비합니다. Native와 WASM은
  같은 정적 메트릭을 사용합니다. 다른 family의 기존 메트릭과 미등록 fallback은 유지합니다.
- 이 변경의 Native 재출력·대조군 및 이후 fresh WASM은 아직 미실행입니다.
  기존 74.88730% 결과를 수정 후 결과로 사용하지 않으며, 새 회귀 기대값도 추가하지 않습니다.
  원본 PDF의 미분 기호는 `HYhwpEQ`에 없는 `′`를 `Haansoft Batang`으로 출력하므로,
  본문 폭 연결 이후 수식 자체의 glyph·간격과 해당 대체 글꼴도 계속 대조합니다.

## 새 브랜치 이관 완료

- 작업 브랜치: `fix/equation-font-metrics-pagination-20261007`.
- 새 base: `b34fa18d74391a583efe8f5b4db8e84d36e130dd` (`upstream/devel`).
- 이관 전 보존: `backup/equation-followup-before-upstream-20261007` 및 기존
  `review/phantomn-pr7600-20261006`, 마지막 보존 commit `c5d1416fc`.
- 이미 통합된 원 기여 `fe30ba9b7`을 제외하고 미통합 보정 115개 commit을 새 base 위로
  rebase했다. 새 수식 메트릭·HWP·PDF 보존 commit은 `4fa6dc9f9`다.
- 문서 add/add 충돌은 과거 로컬 검토 이력을 순차 재적용한 뒤 원 PR review를 upstream의
  실제 병합 판정으로 복원했다. 원 PR의 완료 판정과 새 작업의 미완료 상태를 분리한다.
- 최신 base의 #7608 및 해당 table command·회귀 변경을 포함한 ancestry를 확인했다.
  새 font metric과 새 base의 Native 재출력은 아직 수행하지 않았다.

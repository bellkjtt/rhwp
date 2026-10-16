---
kind: report
status: working
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-16
---

# PR #7433 검토 — 저장 줄 없는 TAC 표의 내용 성장과 중첩 분할

## 판정

**머지 보류 — devel 대비 전쪽 일대일 대응 미검증, 추가 저장 줄 혼재 반례 보정 필요.**
새 PR 생성은 사용자의 별도 승인 전까지 보류한다.
`visual_verification_governance.md`의 devel 전쪽 일대일 비교도 별도로 충족해야 한다.
독립 Print와 일치하는 쪽수 복구를 base TSV 누락의 자동 예외로 처리하지 않는다.

- 원 PR: #7433, `imsebeom`, 원 head `d895e51ac5fd1621abb99f0a20aa52034e01e083`.
- 관련 이슈: #7419. 현재 GitHub 이슈 종료·원 PR 종료는 하지 않았다.
- 기여자 변경: `52a51c0a97d0df7baab1e815ed820c6dff7ae2bb`와 `4905a31d0746fa86e23fe03655f4538079e2d0a1`을 체리픽했다.
- 기준: `4662c069e557e928ed7968d061121fde90b1f027`.
- integration code: `59d87766f96a72b8502a49aff31c66dd16b9742c`, branch `review/imsebeom-7433-20261016`.
- 기여자의 fork를 rewrite하지 않는다. 승인·통합 대상은 메인터너 보정이 포함된 통합 후보이며 원 contributor head가 아니다.

## 기여자 수정과 메인터너 보정

기여자는 NO_LS HWPX의 TAC 표에서 내용 성장분을 보존하는 분기와 두 입력을 추가했다.
메인터너는 선언 높이 합의0.5px 일치로 입력 생성 출처를 추정하는 조건을 제거했다.
중첩 표의 실제 본체 높이·바깥여백과 UTF-16 슬롯의 줄 소속을 현재 프레임이 발행하고,
높이 측정, 셀 유닛, whole 셀 배치, partial eager/lazy 조각이 같은 결과를 소비한다.

1×1 wrapper는 배치 소유자가 단일 자식 표와 폭 중립 마커일 때만 해제한다.
두 자식 표를 첫 표 하나로 축소하지 않는다. Print가 wrapper를 통째로 다음 쪽에 보낸다는
독립 결과에 따라 내부 행을 부모의 줄 경계로 승격하지 않았다.
실제 개체 순회는 원본 모델 참조를 유지해 주소 기반 캐시에 임시 Table/Cell을 넣지 않는다.
앵커의 본체 높이는 원본 선언으로 되돌리지 않고 현재 프레임 값을 소비한다.

구체적인 생산→측정→예약/컷→배치 호출과 적용/비적용 범위는
`../../working/task_m100_7419_stage1.md`와 `../../working/task_m100_7419_stage3.md`에 연결했다.
문서 ID·한계값 완화·출력 숨김을 추가하지 않았다. HWP5·legacy·유효 저장 줄의 조판 계약은
재조판 분기의 새 수용 조건으로 완화하지 않는다.

## 독립 출력과 회귀

입력11개는 원본2개 및 선언 높이·짧은 용지·동일 줄 두 표·명시적 개행·폭 부족·앞뒤 텍스트·
바깥여백·분할 셀·wrapper 반례9개다. 입력은 수동 XML이며 실제 한컴 저장본으로 주장하지 않는다.
같은 입력에 대한 한컴2024 13.0.0.3901의 PrintMethod0, frame_print_to_pdf_ex_one_up PDF를 사용한다.
최종 source Native/fresh WASM 양쪽 모두11문서15쪽 최저100%, 미달·누락0, 독립 Print 쪽수 일치를 확인했다.
이는2px 관용 실루엣이며 엄격 픽셀100%가 아니다. 전쪽 직접 판독과 여백/이월/뒤 본문 확대 판독도 마쳤다.

정식 검사는 `tests/cases/issue_7419_no_ls_tac_row_growth.rs`다. 원본2개의 고정 영역 선택을
셀 내용·순서·포함·비겹침 관계로 교체하고, 같은 줄/개행/폭 부족/텍스트/여백/이월/조각 종료를
검사한다. 완료 페이지의 제목과 후속 본문 원점은 독립 PDF span.origin을96dpi로 환산해±2px로
보호한다. 관계 검사도 유지한다. 최종 출력 생산자를 실행하며 helper의 계산값만 재인용하지 않는다.

초기10개는 devel0통과/10실패, 원 체리픽3통과/7실패, 최초 보정10통과를 같은 검사 실행 파일로
확인했다. 추가 분할2개를 포함한12개에서 여백 앵커 오류는10통과/2실패를 냈고, 같은 실행 파일과
기대값을 유지한 수정 뒤12통과를 확인했다. 리베이스 후 실행·시각·전체 결과는 별도로 기록한다.

## 최종 source 검증과 남은 게이트

- 보정 전 `fafd0d378` 전체:10,447건 중10,444통과/3실패/45skip, exit100.
- 실패는 Native NO_LS의 기존 줄 소유권1건, 가상융합 시행령의 셀 쪽 밖2줄 및 off-canvas2건이다.
  일반 문단의 수용 조건까지 넓혔던 원인을 고쳤으며 기대값·baseline·허용치는 변경하지 않았다.
- 보정 후 `59d87766f`: 기존 실패3개+신규 관계/완료 쪽 좌표12개를 실제 Cargo로 실행해15/15통과.
  같은 시행령의74쪽 render tree JSON도 devel과 바이트 동일하다. 이 문서를 새로 시각 보정했다고
  주장하거나 전체90% 이상 문서로 분류하지 않는다.
- 최종 source의 fmt·host Clippy·WASM32 Clippy·workspace build·workspace all-target Clippy·
  suite manifest6종 모두 통과했다. release-test 전체 빌드는 아래 추가 혼재 반례의 보정을 위해 중단했다. Native Skia3종은 미실행이다.
  이 두 묶음은 실행 큐의 실제 종료 뒤 갱신한다.
  첫 전체 결과와 집중 통과를 합산해 현재 source의 전체 통과로 표시하지 않는다.
- Native/fresh WASM 전체15쪽 직접 대조, 대표 review/standalone overlay의 한글 라벨·수치 판독 완료.
  새 회귀는 관계 검사11개와 독립 Print 기준±2px 검사1개로 완료14쪽의 중요한 위치를 보호한다.
- Studio 원본2개의 Canvas2D·CanvasKit compat/default software6캡처를 추가로 실행했다.
  실제 초기 WASM 응답6개가 fresh package SHA와 일치하며 replay hard violation0이다.
  backend 간 엄격 픽셀 비교4건은 report-only 불일치(최대0.01088193)다. 별도 parity 통과로
  승격하지 않는다. 독립 PDF의 같은 내용 영역에서 줄바꿈·제목·뒤 표·겹침을 직접 확인했다.

실측 높이 목록이 전달된 셀만 새 빈 host/다중 TAC 프레임을 만들고, 일반/Native는 기존
보이는 글자+단일 TAC 계약을 유지한다. 이 분기의 생산·소비와 실패 전후 로그는
[Stage3](../../working/task_m100_7419_stage3.md)에 연결한다.

### devel 전쪽 대응 판정

비교 가능한8문서의 각 backend8쪽에서는0.01%p 이상 하락0·조건 불일치0이다.
아래3문서는 후보/Print의 정상 쪽수 복구 때문에 base 전체 TSV가 없다.
사용자가 선택한 현행 일대일 기준을 유지하므로 최저100%라도 **re_review_required**다.

| 입력 | devel Native/WASM | 후보 Native/WASM | 독립 Print |
| --- | --- | --- | --- |
| page-height-28000 | 2/2 | 3/3 | 3 |
| partial-nested-width-wrap | 1/1 | 2/2 | 2 |
| partial-nested-wrapper | 1/1 | 2/2 | 2 |

비교 불가를 실제0.01%p 하락으로 부르지 않는다. 정상 반례를 제거하거나 기대값을 바꿔
보류 조건을 회피하지 않는다. 전쪽 대응 입증 전에는 PR 생성·승인·통합을 하지 않는다.

### 주장과 증거 판정

| 항목 | 판정 | 근거 |
| --- | --- | --- |
| 원본2개와 내용 성장/동일 줄/개행/폭 부족/여백/분할/wrapper/뒤 본문 보존 | 충족 | Native/fresh WASM15쪽, 독립 Print, 관계/좌표12개 |
| HWP5 NO_LS 정상 대조군과 가상융합 시행령의 무회귀 | 충족 | 기존 실패3개 재실행,74쪽 render tree 동일 |
| 완료 페이지98% 이상과 독립±2px 좌표 보호 | 충족 | 양쪽 최저100%, 완료14쪽·관계 검사 유지 |
| source59d의 전체 회귀·lint·Native Skia 최종 묶음 | 미검증 | 실행 중, 완료 로그로 갱신 필요 |
| devel 전쪽 일대일 무회귀 | 미검증 | 쪽수 복구3건의 base TSV 누락 |
| rowspan·캡션·각주가 함께 있는 신규 시각 일치 주장 | 비해당 | 이번 입력과 신규 주장에 없음, 기존 전체 검사는 유지 |
| GitHub CI·PR·merge·후속처리 | 미검증 | 별도 승인 전, 미실행 |

### 재현 산출물

[자산 출처·SHA](../assets/pr_7433_maintainer/README.md),
[계획](../../plans/task_m100_7419.md), [Stage1](../../working/task_m100_7419_stage1.md),
[Stage2](../../working/task_m100_7419_stage2.md), [Stage3](../../working/task_m100_7419_stage3.md).

![최종 Native 중첩 원본 review](../assets/pr_7433_maintainer/native-nested-source-review-001.png)

![최종 fresh WASM 분할 wrapper review](../assets/pr_7433_maintainer/wasm-wrapper-review-002.png)


## 추가 혼재 경계의 직접 실패 — 최종 후보 재보정 필요

한컴2024가 실제 저장한 HWP/HWPX를 만든 뒤 부모 host와 첫 자식의 LineSeg만 제거했다.
둘째 자식의 실제 저장 줄은 그대로 유지한다. 두 자식 줄을 모두 제거한 대조군도 만들었다.
두 변형의 독립 Print는 모두2쪽이며, 쪽별 PDF PNG가 바이트 동일하다.
59d87766f에서 전부 NO_LS인 대조군은100%/100%, 저장 줄 혼재 반례는55.80081%/68.31124%다.
혼재 반례에서 첫 자식이1쪽에 나오지 않고2쪽의 두 자식이 겹치는 것을 직접 확인했다.
이는 이전11문서의100%와 별도의 새 실패이며, 그 통과로 면제하지 않는다.
`FrameGeometry`가 NoLS 자식만 소유하여, 같은 프레임 안의 저장 줄 자식이 셀 유닛에서 따로
예약되는 생산/소비 불일치다. 실측 프레임을 만들 때 모든 TAC 형제를 함께 소유하게 보정한다.
이 검사를 추가하기 전 양쪽90% 이상과 직접 판독을 다시 확보한다. 생성 과정·SHA·한컴job·PDF·
수정 전 PNG는 ignored `output/pr-review/pr7433-current-20261016/regression-fix/mixed-storage/`에 보존한다.
59d의 최종 전체 빌드는 이 결함 수정 전에 중단했다. 외부 pipeline exit130이며 테스트 summary가
없으므로 전체 회귀의 성공/실패 건수로 기록하지 않는다. lint6종과 기존15개 집중 성공은 당시
source의 증거로 남기고 다음 source에서 필요한 검증을 다시 한다. Native Skia3종은 미실행이다.

원시 증거의 최종 하위 경로는 `output/pr-review/pr7433-current-20261016/regression-fix/`다.
`producer-provenance.json`, `candidate-{native,wasm}-sweep/<입력>/silhouette.tsv`,
`final-devel-comparison.json`, `final-direct-read.json`, `decree-tree-comparison.json`,
`studio-initial-responses.json`, `pipeline.log`, `{lint,full,native}-validation.json`을 사용한다.
집중15개 결과는 한 단계 위 `regression-fix-focused.log`다. 실행 전 파일은 아직 없을 수 있으며
존재 자체를 완료 근거로 쓰지 않는다.

Docker가 없는 이 호스트에서는 locked wrapper의 --dev --no-opt 경로를 사용한다.
Docker 표준 빌드 또는 아직 실행하지 않은 GitHub CI를 통과했다고 표시하지 않는다.

원본/Print 입력 SHA와 전체 명령·로그·TSV는 ignored
`output/pr-review/pr7433-current-20261016/`에 보존한다. TSV·로그·중간 JSON은 커밋하지 않는다.
기본 작업공간 `rhwp-studio/public/rhwp.js`의 사용자 dirty 변경은 보존했다.
PR 생성·push·CI·merge·원 PR 종료·post_merge 정리는 아직 수행하지 않았다.

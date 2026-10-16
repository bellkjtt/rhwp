# PR #7918 — feat: add native IME composition and undo transactions (P55)

## 최종 판정

**머지 보류 — 통합 적용·코드·필수 회귀·시각 검증 진행 전.** 아래 원 head와 통합 후보를 구분해 검증하고 필수 증거가 완성되면 판정을 갱신한다.

## 접수 정보

- [원 PR](https://github.com/edwardkim/rhwp/pull/7918), 기존 contributor `seo-rii`, base `devel`, non-draft.
- 원 head `91a46f90f987cdec23a78fdf9c7463a77db38ed2`, 검토 base `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- 접수 시 규모 +21248 / -1327 / 143 files는 누적 source diff다. 선행 PR을 중복 적용하지 않는다.
- reviewer `jangster77` 지정. 작성 시 mergeability는 참고값이며 실제 통합 후보와 최신 head를 다시 확인한다.
- [구현 순서](pr_7918_review_impl.md), [전체 계획](../../plans/pr_review_seo_rii_20261017.md).

## 변경·검증 범위

원 PR 본문과 직전 단계 대비 실제 diff의 주장·소비 경로·독립 기대값을 대조한다. 현재 검사는 미실행이며 기존 source CI나 과거 이미지를 통합 head의 검증으로 세지 않는다.

## 조판 원칙과 입력 확인

렌더러/paint/font/편집 소비 경로는 영향을 분석하고 Native/fresh WASM·실물 backend·독립 Print 증거를 연결한다. 적용 여부와 입력 commit/SHA 일치는 현재 미검증이다.

## Merge 후 contributor PR comment 계획

통합 PR의 실제 merge SHA·최신 CI·로컬 결과·이 PR의 원 기여와 메인터너 보정 이유를 한국어 존댓말로 기록한다.
시각 증거가 사용되면 해당 review/overlay의 merge SHA 고정 raw URL과 실제 지표·직접 판독 결과를 포함한다.
통합 PR merge와 문서/asset의 devel 존재 확인 뒤 이 원 PR에 통합 링크·감사 comment를 남기고 close한다. fork branch는 삭제하지 않는다.

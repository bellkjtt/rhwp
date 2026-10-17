# PR #7985 적용 기록

## 신원과 범위

- 원 PR: https://github.com/edwardkim/rhwp/pull/7985
- 원 author: donggyun112 <ssddgg99@daum.net>.
- 원 head `5dd0c86ac1a86c977e9d80a7fe263ac60358f720` → 적용 `c575ae384aa4ac343d1b64531f2490206ee9ccfe` (`-x` 원 SHA 보존).
- 최신 base `7d01ef610f378157b79d36ef23e8ed4a3c8ab051` · 검증 head `2dee7730306dcc7f57615c87c25cb09af3c90f63`.
- 기본 작업공간 `review/donggyun112-20261017`에서 검토했으며 별도 worktree는 만들지 않았다.
- [최종 검토](pr_7985_review.md) · [누적 계획](../../plans/pr_review_donggyun112_20261017.md).

## 충돌과 보정

manifest 충돌은 #7977 HF/FN 설명과 #7983 신규 항목을 모두 유지했다.
메인터너 제품 보정 `607f1f7b515553d2c1b618448d345b1d72e8e3e3`는 #7985의 `onInput`에서 실제 빈 누름틀 정규화보다
선택 허용 검사를 먼저 수행한다. #7978의 여섯 scalar 수정은 원 기여 그대로다.
기존 가운뎃점 E2E의 manifest 누락 한 행을 보완했고 기존 검사의 조판 정확성을 새로 주장하지 않는다.
추가 #7988의 실제 UI Unicode 치환·undo/redo 회귀는 `2dee7730306dcc7f57615c87c25cb09af3c90f63`에 별도로 보강했다.

## 검증과 리베이스

[공통 검증 결과](pr_7985_review.md#공통-로컬-검증)에 실제 명령·수치·WASM 해시와 한계를 기록했다.
초기 2개 PR만의 성공과 WASM은 이력이며 최종 3개 PR 검증으로 대체했다.
검증 중 upstream의 #7995 바인딩·문서가 추가되어 clean 상태에서 rebase했다. Rust 입력 Git object는
동일하다. 컴파일 중 중단한 이전 전체 회귀는 PASS로 계산하지 않았고 최신 head에서 다시 완료했다.

## 남은 원격 단계

사용자는 push·통합 PR 생성을 승인했다. 로컬 검증과 원격 code head CI는 구분한다.
code CI 성공 뒤 같은 통합 PR의 원 PR별 review·오늘할일 trailing을 확정하고 최신 CI로 merge를 판단한다.
원 PR 결과 댓글·close는 통합 merge 뒤 처리하며 contributor fork는 보존한다.

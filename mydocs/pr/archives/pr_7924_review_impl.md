# PR #7924 통합 검토 구현 계획

- 원 head `1fce487fc80401c17f43599c02c3e0a2ae01057c`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 3개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7924_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `170c08d5272fcdb5373a5d926374eafabbdddae2`
- `570a2ef9ff9394e0fc052da463e5bb523f31dc41`
- `9f724e0b3ab2dfb90e714af7a1337cef6a7236cf`

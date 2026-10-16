# PR #7893 통합 검토 구현 계획

- 원 head `01254c417ed0cf0333015d0d57099fd9207d6fbc`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 5개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7893_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `32c269935bfe76cfaaa7234a30438e5e79097d4d`
- `91e8a5ed9bc9e2e5bc6eb784f1fd7aa33b740259`
- `a9db18517a8e2e781fa89a6b57ca6e95f5d8bd71`
- `862cd97d9be8779e96b305e209ee7e7a61a7af9a`
- `66f170891742c19d51731405d4cb2a4367b72b61`

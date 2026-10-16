# PR #7931 통합 검토 구현 계획

- 원 head `b4003f89fceb8df6e64652dfdf9ec1cfcd642cf1`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 5개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7931_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `2522f686c8d157a74fb1cb45bc07d0e02eea4b4d`
- `d9687ef02f7b6e147a422ae5c997632042363cda`
- `bfa642725592d3eb672ae8202bad1b149e57f90d`
- `56e05b8c0146722756930579adc1b00d05a83324`
- `a619b9cc8e5fe42c3d6c662c436a9ff761588350`

## draft 제외

다음 commit은 draft #7930의 Qt 창 기능이므로 적용하지 않는다.

- `a3aabfecb0d9b398f6d5c5acd800296dd4cb2842`
- `3880a3345d029f96f69c80d518ac1ba9c3d0a4d3`
- `f5fd95c51e5e7a13bb8fec1568940a4e6dbca306`
- `862957ec2ad738a3f3446e7e7fbee63eaab98a3f`
- `1b40b1ce1b1ad1e6064d6d3e478a009ae8aa0912`

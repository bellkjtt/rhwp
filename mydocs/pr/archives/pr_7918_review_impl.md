# PR #7918 통합 검토 구현 계획

- 원 head `91a46f90f987cdec23a78fdf9c7463a77db38ed2`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 4개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7918_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `5dd42060564baa506fbe24cccb362ecd58a5ea62`
- `9f3631ceefe3f23d28ab26009553c7309fc0867a`
- `c53d069ac665cc93ad05cbfb82dee4db621b33df`
- `201559691de6ff48aed905d28c762d03099d068c`

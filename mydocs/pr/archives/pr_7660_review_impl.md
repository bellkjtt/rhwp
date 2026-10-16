# PR #7660 통합 검토 구현 계획

- 원 head `860f8c22ddd892e74030fb13bae8c8af07ef2aff`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 15개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7660_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `2766cf6aeeddf4171c974d795fe579b9db141f64`
- `e619ac08e0dbb70642543bba15aa5a1c0392c198`
- `1580d078c387b55eebf211a04c95242bfea6d508`
- `02f39d0fc294fe4d1b0027cb97a6942fe0c7aec2`
- `0c167cf6cf1aa1bfff7222e1ce98c52b7adf9126`
- `90a407b393c32372dbaada1b69f103e04bfa2e76`
- `77cf734cec6464e3e36ac8980ae88dd169ffc9b9`
- `d7ae3de95bfcc85d5deec1c773b8ceb5d703cf22`
- `7bf446974c06cef72966f15afc9b42f48c034f65`
- `2ea7173b420809e36dcbef455c39fdea0ff21e03`
- `c6864b49c1d2134a55f45cca1756d0ef8d0d11f8`
- `24a453f889a7d28ac8fddb59dec87603bc1d27cb`
- `5a91f5ff67c12ac51e1e9eca3e56c82b7c3bbce5`
- `a860adef06c679ff49a31f8a0c4a2c8011098788`
- `5476fcea7b46ee7ad190f103b681ead3c3badc9c`

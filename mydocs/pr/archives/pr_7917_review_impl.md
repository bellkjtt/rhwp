# PR #7917 통합 검토 구현 계획

- 원 head `a0ecf253a78c2ccf106c91e9cba19115c88317f2`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 3개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7917_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `0c4241de217ce971fcb68faaf991b7825bb3dac0`
- `1af307ecd54175786e7bada4b11a64e9ae0e7162`
- `429490838288d9ddd015ae70799b98c1d7803b14`

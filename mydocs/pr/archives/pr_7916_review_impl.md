# PR #7916 통합 검토 구현 계획

- 원 head `94a7f2da7d712b0f20abc37c1981bc32d55dfd71`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 3개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7916_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `324b330f50a1bffb2596e5fb14ee0b6a29656662`
- `cb1bc8899e1bf4fbb39446ddb0a523c1d866546f`
- `db2af7602b85825bca30b7d6ff22e6e89e0334b2`

## 실제 적용 결과

- 원 `324b330f50a1bffb2596e5fb14ee0b6a29656662` → 통합 `973ec5052e1c06a8f5491eecc98c7c70be12e0d1` (applied).
- 원 `cb1bc8899e1bf4fbb39446ddb0a523c1d866546f` → 통합 `2cbbfbacce02877699810ed9a16bb69a18e05e3f` (applied).
- 원 `db2af7602b85825bca30b7d6ff22e6e89e0334b2` → 통합 `b6853731545ccdd335e6ac8b51ae19c0670bee62` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 필수 검증은 진행 중이다.

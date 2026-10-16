# PR #7927 통합 검토 구현 계획

- 원 head `73f2f180608f3088bb7a1fd32c0595b5d8fb6797`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 5개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7927_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `33969f8ca0d24b4fd6b7c6a8714b7495ad5b986d`
- `3b83c3c38b45c674795b293769e5f1c2b605517e`
- `cb037437146ae30431817ff84ada1fd56a604bbd`
- `90a249f43868e926923581133579f4014617d0bf`
- `3954cba72930293d79589c2ba2f5ced715a50f2c`

## 실제 적용 결과

- 원 `33969f8ca0d24b4fd6b7c6a8714b7495ad5b986d` → 통합 `17ee717f0de175bf36221773c7dba2be94128065` (applied).
- 원 `3b83c3c38b45c674795b293769e5f1c2b605517e` → 통합 `b0e8ae66b8385298afaa280b5bdaf8530011357b` (applied).
- 원 `cb037437146ae30431817ff84ada1fd56a604bbd` → 통합 `503358c2281cf2725ce522e10f2925927af868b3` (applied).
- 원 `90a249f43868e926923581133579f4014617d0bf` → 통합 `9388faf32a1b19bd2da525d319da51ea8dcbebf1` (applied).
- 원 `3954cba72930293d79589c2ba2f5ced715a50f2c` → 통합 `6c12a20d9fa284f182a660b7779dee222f301e27` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 필수 검증은 진행 중이다.

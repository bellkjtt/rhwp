# PR #7919 통합 검토 구현 계획

- 원 head `25c56fde48d4afc500782f296cb120d5b42b139f`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 4개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7919_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `45815cebe617eece7d1cd05f85c6fe2241b60a92`
- `ebe37071fc8cc6e86a6ba4ed9bb8e245d0ee8c3d`
- `538208de33a093eb26a65a55bbe8efef60a8d259`
- `8a560f40aa9a005091b6729de84dacc34e89f501`

## 실제 적용 결과

- 원 `45815cebe617eece7d1cd05f85c6fe2241b60a92` → 통합 `03bac9ffe097f86e6751d45a2e484f0ab4f87fe0` (applied).
- 원 `ebe37071fc8cc6e86a6ba4ed9bb8e245d0ee8c3d` → 통합 `b1bc6b7cd300bcbd75e3d3aaad0b5b8a4224a95e` (applied).
- 원 `538208de33a093eb26a65a55bbe8efef60a8d259` → 통합 `a47578af312a5ffa8724630b7885a12e0bd61d91` (applied).
- 원 `8a560f40aa9a005091b6729de84dacc34e89f501` → 통합 `36e8c8c8613c49bba41e08e10c6243640e1c4871` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 필수 검증은 진행 중이다.

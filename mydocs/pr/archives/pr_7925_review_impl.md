# PR #7925 통합 검토 구현 계획

- 원 head `c90789077afe74e595abaa672db0afd591e28c1b`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 5개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7925_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `b771805e13a2e6fac700ee72deab2ee3373a3f87`
- `82507bf82d3a295d3963a81ee248755798224817`
- `b84122bdad0bf6d00ec1fbefbc61871699ac732a`
- `6ccd160af7911238993457af9f12557b03917389`
- `a3c8dc36693e816561787f85f2a92596a035b810`

## 실제 적용 결과

- 원 `b771805e13a2e6fac700ee72deab2ee3373a3f87` → 통합 `5c675f121467fea05f1199da4d97518094b466a5` (applied).
- 원 `82507bf82d3a295d3963a81ee248755798224817` → 통합 `5ac4f0f51a088fb67f240a74de25716a60a131af` (applied).
- 원 `b84122bdad0bf6d00ec1fbefbc61871699ac732a` → 통합 `337924329f7cbb652b9c98873a974b73c72d5684` (applied).
- 원 `6ccd160af7911238993457af9f12557b03917389` → 통합 `b344430556ff6bd0d3d50d345304dc331a1d95e4` (applied).
- 원 `a3c8dc36693e816561787f85f2a92596a035b810` → 통합 `5b021e07488471c505847134265714ad0f646ac3` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 필수 검증은 진행 중이다.

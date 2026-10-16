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

## 실제 적용 결과

- 원 `2522f686c8d157a74fb1cb45bc07d0e02eea4b4d` → 통합 `eb2b92e8059c304498df18a14c1e497a5d927f82` (applied).
- 원 `d9687ef02f7b6e147a422ae5c997632042363cda` → 통합 `695fbbdbe196a0e6ffc3097102751d6398d4637b` (applied).
- 원 `bfa642725592d3eb672ae8202bad1b149e57f90d` → 통합 `baf3efb6428c62d38db07c15838eb3b0b8bbd38d` (applied).
- 원 `56e05b8c0146722756930579adc1b00d05a83324` → 통합 `52ee9a9ff1aac16a58857da9bc558a79eb6b128b` (applied).
- 원 `a619b9cc8e5fe42c3d6c662c436a9ff761588350` → 통합 `52ee9a9ff1aac16a58857da9bc558a79eb6b128b` (empty-preserved-by-current-tree).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 로컬 필수 검증을 완료했다. 원격 CI는 실제 통합 PR 생성 뒤 확인한다.

## 통합 후보 검증 연결

- 원 commit 적용과 merge 해소는 위 실제 SHA mapping 및 [1단계](../../working/task_m100_536_integration_review_stage1.md)를 따른다. 누적 source diff를 같은 PR의 고유 변경으로 반복 적용하지 않았다.
- 런타임/검사 분리는 `456ac3585`, 독립 Print 근거의 공통 극한 이름은 `d9a0b1c79`, 정식 원본 회귀는 `1ed02ebff`, font-request 반례·Print 보존은 `068769e9d`에 연결한다. 이 보정들을 원 contributor가 작성한 것으로 표시하지 않는다.
- 실제 실행 결과·source/binary/font/PDF hash와 직접 판독은 [5단계](../../working/task_m100_536_integration_review_stage5.md)·[6단계](../../working/task_m100_536_integration_review_stage6.md)에 있다. 기본 전체 nextest는 10,643 PASS / 0 FAIL / 45 skipped로 끝났다. Native 집중 277 PASS, 공식 Skia lib 4,043 PASS와 focused 2+4 PASS, format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 완료했다. 원격 exact-head CI는 미실행이며 이후 실제 결과로 trailing 기록을 갱신한다.
- 최종 판정과 merge 후 원 PR comment 계획은 같은 번호의 개별 review에 유지한다. 통합 PR 번호의 별도 review를 만들거나 contributor branch를 rewrite하지 않는다.

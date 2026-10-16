# PR #7928 통합 검토 구현 계획

- 원 head `e2ddab5377f828dda0260cf7dc10d2b41d9271d7`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 9개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7928_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `cd1c65a24aefdedff3b23446be8b6a20555c3265`
- `423fd61def0e39e63edf6cb65ae0be9b50ab5a1f`
- `54106622db4bda8c98a828f3cb155754664160cf`
- `2fb70b4c0a58a72e115a74c9681d41af1cfa9cb4`
- `ae4d5454058e4f1a60b751c58ae9e367413f4e7b`
- `50b8f915576d43bafc303203c80275c7615181c1`
- `3247b22747f4e67327d4f765a6a4a950da1e867b`
- `04fd395c0338c275c374c1165417957423e6aeaf`
- `edb66027512bbd3dc398f5c6a37d54ab60973de8`

## 실제 적용 결과

- 원 `cd1c65a24aefdedff3b23446be8b6a20555c3265` → 통합 `c625fad8634665ca1fb7a091335556d5d4682a30` (applied).
- 원 `423fd61def0e39e63edf6cb65ae0be9b50ab5a1f` → 통합 `583ff53ef95a82758dd2d6c590020194f48226d0` (applied).
- 원 `54106622db4bda8c98a828f3cb155754664160cf` → 통합 `c986ec2741f8fb82f452b6f721c604f6e26bf218` (applied).
- 원 `2fb70b4c0a58a72e115a74c9681d41af1cfa9cb4` → 통합 `3d084a97aa828338ec794a79564cf2c11223e87b` (applied).
- 원 `ae4d5454058e4f1a60b751c58ae9e367413f4e7b` → 통합 `d779bdf6f25bab23db97a8411fe977763a8b5d1d` (applied).
- 원 `50b8f915576d43bafc303203c80275c7615181c1` → 통합 `0ed1f56bf3609ca6b308bd015fce2538dc82b819` (applied).
- 원 `3247b22747f4e67327d4f765a6a4a950da1e867b` → 통합 `8532ed9cf6f936853ed2bcf73c6ba2347bd8582c` (applied).
- 원 `04fd395c0338c275c374c1165417957423e6aeaf` → 통합 `6fec66d4097bb42fb5d756bb3d29d7e70b12637b` (applied).
- 원 `edb66027512bbd3dc398f5c6a37d54ab60973de8` → 통합 `6fec66d4097bb42fb5d756bb3d29d7e70b12637b` (empty-preserved-by-current-tree).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 로컬 필수 검증을 완료했다. 원격 CI는 실제 통합 PR 생성 뒤 확인한다.

## 통합 후보 검증 연결

- 원 commit 적용과 merge 해소는 위 실제 SHA mapping 및 [1단계](../../working/task_m100_536_integration_review_stage1.md)를 따른다. 누적 source diff를 같은 PR의 고유 변경으로 반복 적용하지 않았다.
- 런타임/검사 분리는 `456ac3585`, 독립 Print 근거의 공통 극한 이름은 `d9a0b1c79`, 정식 원본 회귀는 `1ed02ebff`, font-request 반례·Print 보존은 `068769e9d`에 연결한다. 이 보정들을 원 contributor가 작성한 것으로 표시하지 않는다.
- 실제 실행 결과·source/binary/font/PDF hash와 직접 판독은 [5단계](../../working/task_m100_536_integration_review_stage5.md)·[6단계](../../working/task_m100_536_integration_review_stage6.md)에 있다. 기본 전체 nextest는 10,643 PASS / 0 FAIL / 45 skipped로 끝났다. Native 집중 277 PASS, 공식 Skia lib 4,043 PASS와 focused 2+4 PASS, format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 완료했다. 원격 exact-head CI는 미실행이며 이후 실제 결과로 trailing 기록을 갱신한다.
- 최종 판정과 merge 후 원 PR comment 계획은 같은 번호의 개별 review에 유지한다. 통합 PR 번호의 별도 review를 만들거나 contributor branch를 rewrite하지 않는다.

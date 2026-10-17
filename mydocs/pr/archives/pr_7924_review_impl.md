# PR #7924 통합 검토 구현 계획

- 원 head `1fce487fc80401c17f43599c02c3e0a2ae01057c`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 3개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7924_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `170c08d5272fcdb5373a5d926374eafabbdddae2`
- `570a2ef9ff9394e0fc052da463e5bb523f31dc41`
- `9f724e0b3ab2dfb90e714af7a1337cef6a7236cf`

## 실제 적용 결과

- 원 `170c08d5272fcdb5373a5d926374eafabbdddae2` → 통합 `c9998b03b40fb948518cca289157a050fe734c81` (applied).
- 원 `570a2ef9ff9394e0fc052da463e5bb523f31dc41` → 통합 `4572fc18063b83d6ff5bc188fd20afb09ab147a7` (applied).
- 원 `9f724e0b3ab2dfb90e714af7a1337cef6a7236cf` → 통합 `09cf619e41ce0365dac3d9f419b35ee248171ac6` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 로컬 필수 검증을 완료했다. 원격 CI는 실제 통합 PR 생성 뒤 확인한다.

## 통합 후보 검증 연결

- 원 commit 적용과 merge 해소는 위 실제 SHA mapping 및 [1단계](../../working/task_m100_536_integration_review_stage1.md)를 따른다. 누적 source diff를 같은 PR의 고유 변경으로 반복 적용하지 않았다.
- 런타임/검사 분리는 `456ac3585`, 독립 Print 근거의 공통 극한 이름은 `d9a0b1c79`, 정식 원본 회귀는 `1ed02ebff`, font-request 반례·Print 보존은 `068769e9d`에 연결한다. 이 보정들을 원 contributor가 작성한 것으로 표시하지 않는다.
- 실제 실행 결과·source/binary/font/PDF hash와 직접 판독은 [5단계](../../working/task_m100_536_integration_review_stage5.md)·[6단계](../../working/task_m100_536_integration_review_stage6.md)에 있다. 기본 전체 nextest는 10,643 PASS / 0 FAIL / 45 skipped로 끝났다. Native 집중 277 PASS, 공식 Skia lib 4,043 PASS와 focused 2+4 PASS, format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 완료했다. 원격 exact-head CI는 미실행이며 이후 실제 결과로 trailing 기록을 갱신한다.
- 최종 판정과 merge 후 원 PR comment 계획은 같은 번호의 개별 review에 유지한다. 통합 PR 번호의 별도 review를 만들거나 contributor branch를 rewrite하지 않는다.

## 최신 base 리베이스

새 base `b7a5388034ee66f58a7e1d5363974e83b6374f57`, 직후 head `a7aac1a9ce5b2b902471ecd8f234f7d72834812d`. [7단계](../../working/task_m100_536_integration_review_stage7.md)의 새 head 검증은 진행 중이다. 위 실제 적용 SHA는 리베이스 전 기록이며 아래는 현재 대응이다.

- 이전 통합 `c9998b03b40fb948518cca289157a050fe734c81` → 현재 `2d13efdb892b3aaee0ed951b3dffe8b66625578d`.
- 이전 통합 `4572fc18063b83d6ff5bc188fd20afb09ab147a7` → 현재 `06821a3f7e17f0ff31b7a9e0c26676db6418d486`.
- 이전 통합 `09cf619e41ce0365dac3d9f419b35ee248171ac6` → 현재 `415fcdd619020d269c08d46336ea95fd0814a739`.

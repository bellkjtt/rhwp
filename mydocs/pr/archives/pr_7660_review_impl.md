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

## 실제 적용 결과

- 원 `2766cf6aeeddf4171c974d795fe579b9db141f64` → 통합 `2f4d36449e5a39f2547638bb312641c15077015f` (applied).
- 원 `e619ac08e0dbb70642543bba15aa5a1c0392c198` → 통합 `dccf3524cebebbf3833fdd1457044f6fbd56a4fb` (applied).
- 원 `1580d078c387b55eebf211a04c95242bfea6d508` → 통합 `eb2ccded4ee2f2d9447786f809db6006df76a1c7` (applied).
- 원 `02f39d0fc294fe4d1b0027cb97a6942fe0c7aec2` → 통합 `283e579b3cfe50c33e60992e5aa7d04d1e16f2f8` (applied).
- 원 `0c167cf6cf1aa1bfff7222e1ce98c52b7adf9126` → 통합 `e64f2dc04c93d935a3a381bdafb8c218380bab2e` (applied).
- 원 `90a407b393c32372dbaada1b69f103e04bfa2e76` → 통합 `5d18a568e4e39004d31d1098628b96068f16315c` (applied).
- 원 `77cf734cec6464e3e36ac8980ae88dd169ffc9b9` → 통합 `37e4ecb82dd725ecb6358876f723f70d25e89c07` (applied).
- 원 `d7ae3de95bfcc85d5deec1c773b8ceb5d703cf22` → 통합 `820f6747a53a978e2bef18c8f92df133f2c84154` (applied).
- 원 `7bf446974c06cef72966f15afc9b42f48c034f65` → 통합 `e19892d48c75091efd9814e7adc1251267a2290a` (applied).
- 원 `2ea7173b420809e36dcbef455c39fdea0ff21e03` → 통합 `c37bc966845e8a297202aef5425fdc91873cc93b` (applied).
- 원 `c6864b49c1d2134a55f45cca1756d0ef8d0d11f8` → 통합 `1a92c9dc539d0de1dee19f85f6aaaa80b4709a79` (applied).
- 원 `24a453f889a7d28ac8fddb59dec87603bc1d27cb` → 통합 `fe69d6af72376ac1da0fe7c94cfe0306fbc53092` (applied).
- 원 `5a91f5ff67c12ac51e1e9eca3e56c82b7c3bbce5` → 통합 `5eab754737a8a80a4f4a7680338a398c8fba49ed` (applied).
- 원 `a860adef06c679ff49a31f8a0c4a2c8011098788` → 통합 `978d7e3dd7f68fe7ece6adde8096cb72bf497cf8` (applied).
- 원 `5476fcea7b46ee7ad190f103b681ead3c3badc9c` → 통합 `1c41caaa96bd4d580940783c55a467c63bb4959a` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 로컬 필수 검증을 완료했다. 원격 CI는 실제 통합 PR 생성 뒤 확인한다.

## 통합 후보 검증 연결

- 원 commit 적용과 merge 해소는 위 실제 SHA mapping 및 [1단계](../../working/task_m100_536_integration_review_stage1.md)를 따른다. 누적 source diff를 같은 PR의 고유 변경으로 반복 적용하지 않았다.
- 런타임/검사 분리는 `456ac3585`, 독립 Print 근거의 공통 극한 이름은 `d9a0b1c79`, 정식 원본 회귀는 `1ed02ebff`, font-request 반례·Print 보존은 `068769e9d`에 연결한다. 이 보정들을 원 contributor가 작성한 것으로 표시하지 않는다.
- 실제 실행 결과·source/binary/font/PDF hash와 직접 판독은 [5단계](../../working/task_m100_536_integration_review_stage5.md)·[6단계](../../working/task_m100_536_integration_review_stage6.md)에 있다. 기본 전체 nextest는 10,643 PASS / 0 FAIL / 45 skipped로 끝났다. Native 집중 277 PASS, 공식 Skia lib 4,043 PASS와 focused 2+4 PASS, format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 완료했다. 원격 exact-head CI는 미실행이며 이후 실제 결과로 trailing 기록을 갱신한다.
- 최종 판정과 merge 후 원 PR comment 계획은 같은 번호의 개별 review에 유지한다. 통합 PR 번호의 별도 review를 만들거나 contributor branch를 rewrite하지 않는다.

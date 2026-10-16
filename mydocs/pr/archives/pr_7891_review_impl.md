# PR #7891 통합 검토 구현 계획

- 원 head `8273ece22860b5793767b9888dd320b3bdc414b8`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 29개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7891_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `9c20ab17c08806fea8327eafdfc53775f18ee9e7`
- `8360eed403f14c0bc271643edc512adf9e2705ff`
- `8adbbac59cbadd02e49afcba6f416ff251308509`
- `2743e925a211ea8753e82d88896b57203f8fbd41`
- `1b5b5a5213ec9333033e6241301b8c899df0337f`
- `5458b4d7f9281449bf23e879cb7848f17afcffbe`
- `3c14e8ca9f6ea3f75f63a1e7f55a3a4b12b2d70f`
- `bc8ab82e8e5a060ebee67bea4d25146ff4ce37ec`
- `8b3debe2950f89c7bf8e2c40532b1b4db7ed4650`
- `65ac4f709af96b35a127bb8ed0689620bf2f9e91`
- `abd57190925e99cd12d5570c24af5bb1fb55f68c`
- `f47d6fd9e0de0eeb27f135456902385078f0bae8`
- `a8d60bb85df67bd74528277273cf7edd19ba46b0`
- `987b47874a40487f2b6e531e472f4f28d39aef83`
- `c6c6d43a1ad38bdce4793787a6934e8873a5dcc1`
- `f559227abc6f8818a356ffd995fc5727e3e30dac`
- `95857610ca11acde429a0b031d9d11e9845005b3`
- `5cb8bf46451abda43f031b4934e07fa22b76004c`
- `a4d61aa1758570052824e64946e0fc0b0be512d1`
- `60db4c7a47f3a17af324a81b645cd260e1fd86d7`
- `d4b39efd31808524a19043b3fe6e73acb0ef1182`
- `59c6fd56d2ee25826bd4cd78374727c36074529c`
- `a22d902ae8081ae6ec58d8d833527f79b7fa014f`
- `6f4de1630156fb6c772bb6a8aa94d938838dfa27`
- `3a025062849a80115ab0e1dd1e0d9b9a70f8eb96`
- `2525591a822c472eecc19dbda53e817730c6e7dc`
- `342b17371c3e98b8f87fa0d98ccaf7f26d687fe5`
- `f58eb3ee4998dabcd7373a2b54f06b2bde2c57b9`
- `8273ece22860b5793767b9888dd320b3bdc414b8`

## 실제 적용 결과

- 원 `9c20ab17c08806fea8327eafdfc53775f18ee9e7` → 통합 `a50c25b3aa332a46a14a1f349d03fe82b8ad5053` (applied).
- 원 `8360eed403f14c0bc271643edc512adf9e2705ff` → 통합 `991701fa59b4575fa8be45a0d0f7a6280d95c122` (applied).
- 원 `8adbbac59cbadd02e49afcba6f416ff251308509` → 통합 `5e550e12b679403ba11fa6d4299791234f1d5758` (applied).
- 원 `2743e925a211ea8753e82d88896b57203f8fbd41` → 통합 `4707a41a973ab272784a5c3f991e1bbf15728fa8` (applied).
- 원 `1b5b5a5213ec9333033e6241301b8c899df0337f` → 통합 `e467f08c4daa7f5db9b0fe73850795180ad698b5` (applied).
- 원 `5458b4d7f9281449bf23e879cb7848f17afcffbe` → 통합 `b72a5f2f67acff72238feadd110857d00eb5ac2c` (applied).
- 원 `3c14e8ca9f6ea3f75f63a1e7f55a3a4b12b2d70f` → 통합 `bda7a209a8b9c2a068f2b78488120042180a3ac9` (applied).
- 원 `bc8ab82e8e5a060ebee67bea4d25146ff4ce37ec` → 통합 `1677240efaf4be79c6bebb75b1358b423220326f` (applied).
- 원 `8b3debe2950f89c7bf8e2c40532b1b4db7ed4650` → 통합 `caa893cd846a38da65cbc906e67bc6d5d3236879` (applied).
- 원 `65ac4f709af96b35a127bb8ed0689620bf2f9e91` → 통합 `fc2335062aba5ebe847a21861121e27968ecd7ef` (applied).
- 원 `abd57190925e99cd12d5570c24af5bb1fb55f68c` → 통합 `b3af1d45f6fa989e43753ab04ce30e72e3da243f` (applied).
- 원 `f47d6fd9e0de0eeb27f135456902385078f0bae8` → 통합 `9060f3d5af21eb3a2fd8f749e7054fddb1a65ade` (applied).
- 원 `a8d60bb85df67bd74528277273cf7edd19ba46b0` → 통합 `a566d272936f7ab52a7d650ee544330761312688` (applied).
- 원 `987b47874a40487f2b6e531e472f4f28d39aef83` → 통합 `5880e26495c31c4bac7fceb93ede75c0632c81f2` (applied).
- 원 `c6c6d43a1ad38bdce4793787a6934e8873a5dcc1` → 통합 `e57e38e1a0c3e18cd6907e2b9defa22f3a32e9d9` (applied).
- 원 `f559227abc6f8818a356ffd995fc5727e3e30dac` → 통합 `b56c132beccf4d14a82b3a0f68ad3a5d631a6729` (applied).
- 원 `95857610ca11acde429a0b031d9d11e9845005b3` → 통합 `3b06b73a0c2ff9392cb4ee34dcdba2bd04829ee0` (applied).
- 원 `5cb8bf46451abda43f031b4934e07fa22b76004c` → 통합 `eb1fc7a32c1728d946d6cd28ac66396e4619187d` (applied).
- 원 `a4d61aa1758570052824e64946e0fc0b0be512d1` → 통합 `56c2f1a984d87d46e4b9b9366b4a74cf38b3a070` (applied).
- 원 `60db4c7a47f3a17af324a81b645cd260e1fd86d7` → 통합 `c840bc89fd85a592f51cee9d79e86ace20f9c8a1` (applied).
- 원 `d4b39efd31808524a19043b3fe6e73acb0ef1182` → 통합 `8c270745c966a0a9c67853fdd790959fa61669d7` (applied).
- 원 `59c6fd56d2ee25826bd4cd78374727c36074529c` → 통합 `e4f63853c0205a1489e832d2f84219ca1e2a33fa` (applied).
- 원 `a22d902ae8081ae6ec58d8d833527f79b7fa014f` → 통합 `420aa5d2a7e33c403a1bc257820217dad8f7223e` (applied).
- 원 `6f4de1630156fb6c772bb6a8aa94d938838dfa27` → 통합 `36ceb87890b4134e5df64890156753c1f86485fa` (applied).
- 원 `3a025062849a80115ab0e1dd1e0d9b9a70f8eb96` → 통합 `dc282d4b42cea652ad5239401c2ecf39b707d1b6` (applied).
- 원 `2525591a822c472eecc19dbda53e817730c6e7dc` → 통합 `cbcdeb7e5a266fdce4399b4c4f78239c3354518d` (applied).
- 원 `342b17371c3e98b8f87fa0d98ccaf7f26d687fe5` → 통합 `3c73244f7a0906d9997bf931d22472df7cc22a58` (applied).
- 원 `f58eb3ee4998dabcd7373a2b54f06b2bde2c57b9` → 통합 `5d6ff65610cc142be8a84a586ae8cff66cf9980b` (applied).
- 원 `8273ece22860b5793767b9888dd320b3bdc414b8` → 통합 `c67a3c2d5e7db31ab934c1c7b8aefeff746d8356` (applied).

[충돌·원 merge 해소 대조](../../working/task_m100_536_integration_review_stage1.md). 로컬 필수 검증을 완료했다. 원격 CI는 실제 통합 PR 생성 뒤 확인한다.

## 통합 후보 검증 연결

- 원 commit 적용과 merge 해소는 위 실제 SHA mapping 및 [1단계](../../working/task_m100_536_integration_review_stage1.md)를 따른다. 누적 source diff를 같은 PR의 고유 변경으로 반복 적용하지 않았다.
- 런타임/검사 분리는 `456ac3585`, 독립 Print 근거의 공통 극한 이름은 `d9a0b1c79`, 정식 원본 회귀는 `1ed02ebff`, font-request 반례·Print 보존은 `068769e9d`에 연결한다. 이 보정들을 원 contributor가 작성한 것으로 표시하지 않는다.
- 실제 실행 결과·source/binary/font/PDF hash와 직접 판독은 [5단계](../../working/task_m100_536_integration_review_stage5.md)·[6단계](../../working/task_m100_536_integration_review_stage6.md)에 있다. 기본 전체 nextest는 10,643 PASS / 0 FAIL / 45 skipped로 끝났다. Native 집중 277 PASS, 공식 Skia lib 4,043 PASS와 focused 2+4 PASS, format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 완료했다. 원격 exact-head CI는 미실행이며 이후 실제 결과로 trailing 기록을 갱신한다.
- 최종 판정과 merge 후 원 PR comment 계획은 같은 번호의 개별 review에 유지한다. 통합 PR 번호의 별도 review를 만들거나 contributor branch를 rewrite하지 않는다.

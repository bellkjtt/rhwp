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

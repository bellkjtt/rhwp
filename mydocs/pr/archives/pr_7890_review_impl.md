# PR #7890 통합 검토 구현 계획

- 원 head `79942a3807d4066dd7cb44abda7697c19ffb614e`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 24개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7890_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `9df25c8f22985486fb0e045f693b5663e8f90609`
- `447cb32e896a380f9c61e328b89a35e67dfbdba3`
- `7cf3f14ddd80d991bf75d6fbe006d4451547c3b4`
- `88cff3560f5052a446fb33b358b3c3f2e32f9f93`
- `084964c5dc54884834d56105ff4429a74c5ac13e`
- `b827287a5a7fdb9bf1968e880148763e48d13772`
- `65adfe356305eca0eac59b73e5a696c5a52758f5`
- `fe3cbd62ce2b49bc4ba10e27af132c338133f3b8`
- `7c4ae0e5383a55168b9cb689f35d247222bba24e`
- `dc6ce6fee23caebf03605496b49084ab47232795`
- `4e2741f70c4e5fe232b6fe4b5985a1b05a50e76a`
- `266117ee941cdccb48d4c039ac372b9a75ed1c6b`
- `493819d8b67a9adac3f881635aa8fabf545e5dc0`
- `ebb1bf918f9006ea7ef4b086018f6a4a988f1dc5`
- `e0604f0d206d84aceebc600ac555704feeda6edb`
- `f5892664440aa3e5a6d940882f3c1250cfc990af`
- `7f93f337c6a79719bdb14c261bc7eb2ae43a3719`
- `f10c215725926f1550cd92ef4eededb3a668e42e`
- `4a53eacd8a77fc18b92470bf32b6809aa22ca6dd`
- `e743bbda044290bac21421c3252479560009b4c1`
- `994e8a7835f22b6c0433d826c490dfb827532e42`
- `c533d85d03499faab411fe41eb64cd7d6a3b34f1`
- `1eade8b0dc6a82d8d4d1874868389bf5ec89276d`
- `6086d76e1e00ad078c769fc6e9b5d5faadf769b3`

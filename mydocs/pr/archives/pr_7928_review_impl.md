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

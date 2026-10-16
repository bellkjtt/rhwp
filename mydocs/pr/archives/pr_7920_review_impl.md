# PR #7920 통합 검토 구현 계획

- 원 head `4d8c9014ba12db6b7c22806f364d16b0a81deaa3`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 6개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7920_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `7444b97280caa96a318f90880486437af3c9cff1`
- `e28961dde6c99023f41cd637db8a1422171d295b`
- `69a1347db41a9f94d2a31c2003f6612d75048784`
- `30e97062dc32e200150dd81618df90ae53954eb1`
- `762b7d2fd709dd205cd28a6a21c069c69b05da3d`
- `a28d839311d2e7fe88824b7afe2df08a33c58a35`

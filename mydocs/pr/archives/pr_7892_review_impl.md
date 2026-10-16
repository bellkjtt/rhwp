# PR #7892 통합 검토 구현 계획

- 원 head `6abf22be0cb0fa10f0a5b9dfe57bdd2539e5df4f`, 기준 devel `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- [공통 실행 계획](../../plans/pr_review_seo_rii_20261017.md)의 순차 gate를 따른다.
- 고유 적용 대상 30개. 아래 SHA는 원 author를 보존해 오래된 순서로 적용한다.
- merge commit과 다른 PR에 이미 속한 commit은 중복 적용하지 않는다.
- 충돌·보정·검증 결과는 [개별 review](pr_7892_review.md)에 연결하며 검증 전 완료를 단정하지 않는다.
- rollback은 통합 PR 제출 전 이 작업 branch에서만 판단한다. 사용자 source/fork/history를 재작성하지 않는다.

## 원 commit 순서

- `5e92cc590df9912f12785f877edf008c8927c344`
- `73a20886174be7be999d1e8ac6f5bf6437c06d01`
- `6ef8c29ab6e91d29c75c5a1b710cd012c0aff653`
- `5486c3d18f14c864a5eb1080b630ba37c3f22536`
- `17702bc78382f86c5a44b89f88634429d218a204`
- `2b9e7d646228094ba0738a5ae1e3c4f91b31a62a`
- `11e0ab0ca4b9e6f7fb17060f41cb94a6fafb0970`
- `2155dae0eb5940108e84ffce866a01cb9c4eb8c0`
- `3325e67e572e6b5a064800df15628119cc9b613a`
- `ab896205e10f53a5fe077da587eaa23d61fc28c8`
- `5b9f3383f104d48d56122aedc65760d4bda40b42`
- `1af3cb865997cc74346af1882a0566f7d8403179`
- `09e53a528cee6da89eba0e197155d08121941b49`
- `4acab69184ad703728f2e78532d3572ba20d7ee6`
- `ace448b4fb726c378ad3d8b849d428b6ad0715d2`
- `b344cac53a20d73cf575ce34a705c6cabd72ecf6`
- `e8d3ea804c5204f5cbc44c000590cb75e4baac34`
- `40f17906614703bfe6ab15194d1548f89d1321cc`
- `d10f419c6b10caf3f7022ed4f4bf50d46b83a534`
- `369c2efa70fc6ce90d8b5a05ada1b3fd835f6b85`
- `592d6aa4f86f54038eb5c0ca833d5b184ecf7b82`
- `83d136f59b7e6221e5e776913a37cc4bad6c6a49`
- `68f7f2ab01b72ba98b20d57e1c20a14857014ae7`
- `189fe1fe9725b4a63059987937a59fffeeeeff47`
- `1916c847d5fd4f6ba9a6737530884b87adba4094`
- `0cb7f9daad6f3137b2df1214c1d58a12c02ff178`
- `b2235e75ee154e3deccc2ebf811c0e4d5eff8c83`
- `36f7d36e728ea9b9a68ec2ad07d353b21ff565ac`
- `eb95f78925f3d29430d51868b862f2679a6fc6ac`
- `a4b2f5ef082554c549f76d1e51e08a95b2d623d2`

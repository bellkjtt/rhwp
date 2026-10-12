# 정렬 기준 출력 충돌의 독립 진단

Issue: #7642

한컴 Office 2020의 기존 Print와 새 구조 대조군에서 동일 수식의 열 정렬이 달라 원인 후보를 대조했다. 실제 출력은 `11.0.0.9136`, `PrintMethod=0` (1-up)이다. 원본 기준 PDF를 변경하거나 이번 결과로 원본의 시각 통과를 선언하지 않는다. 정식 회귀 fixture가 아니다.

## 관측

- 기존 구조/카탈로그 HWP를 정상 HWP로 다시 저장한 뒤 Print해도 기존 로컬 행 정렬이 재현됐다.
- 구조 입력의 첫 equation ID만 18210000에서 18410000으로 바꾸어도 정렬은 바뀌지 않았다. XML의 다른 부분과 archive의 다른 entry는 보존했다.
- 첫 식·라벨·스타일·ID를 유지하고 두 번째 식을 & 있음/& 없음/빈 EQALIGN/중첩 행렬로 바꾼 네 문서는 첫 식의 공통 열을 모두 같은 위치로 출력했다. 이 문맥 후보로 기존 파일의 로컬 정렬을 설명할 수 없다.
- 기존 단어 식의 Print 등호 x는 90.655/160.685 CSS96, 네 새 문서에서는 160.365/160.365다. 값은 진단 관측이며 구현 조건·회귀 좌표로 사용하지 않는다.
- source/code/문서 간 원인은 아직 미확인이다. 파일 ID·타입·폭 수치로 조건을 추가해 상충하는 출력을 흉내 내지 않는다.

## 실행 job

### resave

- `structural-resaved-hancom-2020.hwp`: `7668a8b4-a084-495e-bdde-af7b596d11f7`, 입력 SHA `9cb3eef2652d2caf9c786c7c4cc69ecdb1b1621eb0d749ca8c89e3fbd93cf3fd`
- `structural-resaved-hancom-2020.pdf`: `cc713186-cdbc-4281-8bb5-513d288ca38b`, 입력 SHA `af0f0c33d6fbb37d8e49d91e8d8a3591688e5f3fcf4beb8a152cd868961c4d7e`
- `catalog-resaved-hancom-2020.hwp`: `4f80d7f8-1935-4a75-b80a-1cc88c56d83c`, 입력 SHA `5821913b3311590a0a724f93a8f0c5d84966c06bbf42cdd3d8e30fe5664ad3b5`
- `catalog-resaved-hancom-2020.pdf`: `e1ab7b48-0e61-4792-a0fe-57e85beed55a`, 입력 SHA `028d5b5f4e316b5f1506900bc35f6394c3d77e72b168ef7ca01a60aa20e8c1de`

### instance-id

- `instance-id-hancom-2020.hwp`: `697bf3f9-2958-423c-8cb5-96171bbca7e0`, 입력 SHA `99d2819305eecbe407bb143676274b9ac73a005f8e9322942452fad5785a1bd9`
- `instance-id-hancom-2020.pdf`: `980c6d67-1f95-4e06-8ac4-d4496027171c`, 입력 SHA `6ad9d80a8d9f03145a611df25db0fa78021442dccc7d78b20b051560d812bdbc`

### marked

- `alignment-context-marked-hancom-2020.hwp`: `17e79a56-6a07-4e97-a8ce-6220b8c69dd7`, 입력 SHA `045f76df6c794c4c627af24866410805aadde5ea2db1f03e2de4ee0db5fd6798`
- `alignment-context-marked-hancom-2020.pdf`: `940417dd-3435-4bef-95ef-c616d6854d75`, 입력 SHA `b5eb99e8c3928a6b69af2fb39edb9a1827a1501f28e6b929227483f7463495b7`

### unmarked

- `alignment-context-unmarked-hancom-2020.hwp`: `75b8137b-96c3-4f79-9d41-b982f24af500`, 입력 SHA `26e79a9a70cb5331d719cb97ba40e8bcc39be7b68f3048c3dfbcd26312515ea8`
- `alignment-context-unmarked-hancom-2020.pdf`: `9c421819-afb4-4ac3-86a8-5f835ca4701f`, 입력 SHA `e058b6dd6c5eef9a871f1b903afa1a1ceb6ec20cb81c67435248ed3a91337804`

### empty

- `alignment-context-empty-hancom-2020.hwp`: `a1491656-c989-478e-a6fb-4a87f789ed96`, 입력 SHA `15d6c1ab7e174e5d30314150ad65d1575f18183c5658a1987235953d1c9b5cee`
- `alignment-context-empty-hancom-2020.pdf`: `f3f4251a-e601-4cc5-95b6-3a3e4d1a0ebf`, 입력 SHA `9f0818e4332d3964e500d31e9486003dc37e1738ee185118ef0938e9855a32b0`

### nested_matrix

- `alignment-context-nested_matrix-hancom-2020.hwp`: `535dada7-8b29-4faa-adde-9aef3f890716`, 입력 SHA `7b9186d5bbda97286e5076cf22f8297860938be40a20994cdc2649be1d75474e`
- `alignment-context-nested_matrix-hancom-2020.pdf`: `b58fa2ca-8b80-42c3-84fb-17d6215ef55d`, 입력 SHA `cc03b149781843e518da2bb1a69e301d8ef68a02fa4d09349c131d5e1f231b56`

## 파일 SHA-256

- `alignment-context-empty-hancom-2020.hwp`: `9f0818e4332d3964e500d31e9486003dc37e1738ee185118ef0938e9855a32b0`
- `alignment-context-empty-hancom-2020.pdf`: `7277cb7f30b39846a80f978ca4f87eea69eb320ee47f2f4eb73ea1a29ef9ae07`
- `alignment-context-empty-input.hwpx`: `15d6c1ab7e174e5d30314150ad65d1575f18183c5658a1987235953d1c9b5cee`
- `alignment-context-marked-hancom-2020.hwp`: `b5eb99e8c3928a6b69af2fb39edb9a1827a1501f28e6b929227483f7463495b7`
- `alignment-context-marked-hancom-2020.pdf`: `a55a1c4fcb023ac9a37201da277291dd44d5eac9dfa4f759c7b5fcbcb967bd6f`
- `alignment-context-marked-input.hwpx`: `045f76df6c794c4c627af24866410805aadde5ea2db1f03e2de4ee0db5fd6798`
- `alignment-context-nested_matrix-hancom-2020.hwp`: `cc03b149781843e518da2bb1a69e301d8ef68a02fa4d09349c131d5e1f231b56`
- `alignment-context-nested_matrix-hancom-2020.pdf`: `44a9467e8ce22924487681cdfc5057d6d8cb2cd731e6abcab71d90a51837e1a6`
- `alignment-context-nested_matrix-input.hwpx`: `7b9186d5bbda97286e5076cf22f8297860938be40a20994cdc2649be1d75474e`
- `alignment-context-unmarked-hancom-2020.hwp`: `e058b6dd6c5eef9a871f1b903afa1a1ceb6ec20cb81c67435248ed3a91337804`
- `alignment-context-unmarked-hancom-2020.pdf`: `a06d03702636b005369b92f733c5adbc76488dab262bb5bac0327593b5e90a98`
- `alignment-context-unmarked-input.hwpx`: `26e79a9a70cb5331d719cb97ba40e8bcc39be7b68f3048c3dfbcd26312515ea8`
- `catalog-resaved-hancom-2020.hwp`: `028d5b5f4e316b5f1506900bc35f6394c3d77e72b168ef7ca01a60aa20e8c1de`
- `catalog-resaved-hancom-2020.pdf`: `cc71f764abe130b9fbf50d043edc5652b41d990f2f24f174c9a6992776af47bf`
- `instance-id-hancom-2020.hwp`: `6ad9d80a8d9f03145a611df25db0fa78021442dccc7d78b20b051560d812bdbc`
- `instance-id-hancom-2020.pdf`: `c3dbba198ddbda9c6819e71a03659b1d63d43f683815700d55e989ca6f4f3776`
- `instance-id-input.hwpx`: `99d2819305eecbe407bb143676274b9ac73a005f8e9322942452fad5785a1bd9`
- `structural-resaved-hancom-2020.hwp`: `af0f0c33d6fbb37d8e49d91e8d8a3591688e5f3fcf4beb8a152cd868961c4d7e`
- `structural-resaved-hancom-2020.pdf`: `e0abc2b656cfa6e11603a39f26ca6f277595ca4f272751d223d4da19fac3cc9e`

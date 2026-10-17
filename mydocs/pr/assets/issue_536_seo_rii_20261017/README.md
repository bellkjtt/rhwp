# seo-rii PR 통합 검토의 시각 증적

## 전체 페이지 TSV 원본

최종 TSV는 생성 원본의 header·전체 행·열·수치를 바꾸지 않은 파일이다. 모든 입력은 전체 1쪽,
96 DPI·print profile·2px 이웃 관용이며 Native와 fresh WASM을 구분했다.

- 기준 source: `b7a5388034ee66f58a7e1d5363974e83b6374f57`.
- 후보의 실제 Native/WASM producer: `751e34e344d371d276917dccb06b93fa8e79eff7`.
- 후보 code와 별도 문서/asset commit의 역할을 구분한다. Docker 없는 host `--no-opt` fresh WASM이며 배포용 Docker 최적화 빌드 검증은 아니다.
- Native/WASM 모두 90% 이상·누락/쪽수 차이 없음·새 devel 대비 0.01%p 이상 하락 없음. 직접 판독과 API/전체 회귀 완료 여부는 [7단계](../../../working/task_m100_536_integration_review_stage7.md)를 따른다.
- PR 본문에 전체 원문을 넣을 수 있다. 본문 크기 제한에 걸리면 아래 원본의 정확한 head SHA raw 링크로 제출하며, 본문에 원문과 다운로드 링크를 함께 제공할 수도 있다.

| 입력 | 후보 Native | 후보 fresh WASM | 기준 Native | 기준 fresh WASM |
| --- | --- | --- | --- | --- |
| lseg-01-basic | [원본](native/lseg-01-basic_silhouette.tsv) | [원본](wasm/lseg-01-basic_silhouette.tsv) | [원본](base/native/lseg-01-basic_silhouette.tsv) | [원본](base/wasm/lseg-01-basic_silhouette.tsv) |
| lseg-02-mixed | [원본](native/lseg-02-mixed_silhouette.tsv) | [원본](wasm/lseg-02-mixed_silhouette.tsv) | [원본](base/native/lseg-02-mixed_silhouette.tsv) | [원본](base/wasm/lseg-02-mixed_silhouette.tsv) |
| pic-crop-01 | [원본](native/pic-crop-01_silhouette.tsv) | [원본](wasm/pic-crop-01_silhouette.tsv) | [원본](base/native/pic-crop-01_silhouette.tsv) | [원본](base/wasm/pic-crop-01_silhouette.tsv) |
| shape-group-02 | [원본](native/shape-group-02_silhouette.tsv) | [원본](wasm/shape-group-02_silhouette.tsv) | [원본](base/native/shape-group-02_silhouette.tsv) | [원본](base/wasm/shape-group-02_silhouette.tsv) |
| equation-lim-print2020 | [원본](native/equation-lim-print2020_silhouette.tsv) | [원본](wasm/equation-lim-print2020_silhouette.tsv) | [원본](base/native/equation-lim-print2020_silhouette.tsv) | [원본](base/wasm/equation-lim-print2020_silhouette.tsv) |
| form-hwpx | [원본](native/form-hwpx_silhouette.tsv) | [원본](wasm/form-hwpx_silhouette.tsv) | [원본](base/native/form-hwpx_silhouette.tsv) | [원본](base/wasm/form-hwpx_silhouette.tsv) |
| form-hwp-manual-crossformat | [원본](native/form-hwp-manual-crossformat_silhouette.tsv) | [원본](wasm/form-hwp-manual-crossformat_silhouette.tsv) | [원본](base/native/form-hwp-manual-crossformat_silhouette.tsv) | [원본](base/wasm/form-hwp-manual-crossformat_silhouette.tsv) |
| native-ime | [원본](native/native-ime_silhouette.tsv) | [원본](wasm/native-ime_silhouette.tsv) | [원본](base/native/native-ime_silhouette.tsv) | [원본](base/wasm/native-ime_silhouette.tsv) |

### 파일별 원본 해시

| 파일 | 실제 producer SHA | SHA-256 | bytes |
| --- | --- | --- | ---: |
| [base/native/lseg-01-basic_silhouette.tsv](base/native/lseg-01-basic_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `254df30578cc655814a3c76e0c1fe040fd6e8bfb9425a464b0dd3e8db4c9359a` | 136 |
| [native/lseg-01-basic_silhouette.tsv](native/lseg-01-basic_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `254df30578cc655814a3c76e0c1fe040fd6e8bfb9425a464b0dd3e8db4c9359a` | 136 |
| [base/native/lseg-02-mixed_silhouette.tsv](base/native/lseg-02-mixed_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `bbcc43941531cb3b694b9ccbe557f3cdab16acb4b4ff4fcce195de8e8c99ee3a` | 136 |
| [native/lseg-02-mixed_silhouette.tsv](native/lseg-02-mixed_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `bbcc43941531cb3b694b9ccbe557f3cdab16acb4b4ff4fcce195de8e8c99ee3a` | 136 |
| [base/native/pic-crop-01_silhouette.tsv](base/native/pic-crop-01_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `14f3dfd6d178218a5b707d5b125fe4383994226f2d38efb81f7d1b6930611785` | 136 |
| [native/pic-crop-01_silhouette.tsv](native/pic-crop-01_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `14f3dfd6d178218a5b707d5b125fe4383994226f2d38efb81f7d1b6930611785` | 136 |
| [base/native/shape-group-02_silhouette.tsv](base/native/shape-group-02_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `1d1dc181233097557609523a8cdd6a8ede011a960706dc08154a7829391dcb86` | 137 |
| [native/shape-group-02_silhouette.tsv](native/shape-group-02_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [base/native/equation-lim-print2020_silhouette.tsv](base/native/equation-lim-print2020_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `693028c71033061f511671bf726c87cb537377c0916d87ad733fa3216c3f8b5a` | 136 |
| [native/equation-lim-print2020_silhouette.tsv](native/equation-lim-print2020_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `886eabe3adfeafd320e9366221c4ed5f4bef15a081c1753614645842c5c5db24` | 136 |
| [base/native/form-hwpx_silhouette.tsv](base/native/form-hwpx_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `a4ae0656fa7a0665e3cab843c6647633a4ac98d32a30e333129b247ffdd5740c` | 136 |
| [native/form-hwpx_silhouette.tsv](native/form-hwpx_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `a4ae0656fa7a0665e3cab843c6647633a4ac98d32a30e333129b247ffdd5740c` | 136 |
| [base/native/form-hwp-manual-crossformat_silhouette.tsv](base/native/form-hwp-manual-crossformat_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `a4ae0656fa7a0665e3cab843c6647633a4ac98d32a30e333129b247ffdd5740c` | 136 |
| [native/form-hwp-manual-crossformat_silhouette.tsv](native/form-hwp-manual-crossformat_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `a4ae0656fa7a0665e3cab843c6647633a4ac98d32a30e333129b247ffdd5740c` | 136 |
| [base/native/native-ime_silhouette.tsv](base/native/native-ime_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [native/native-ime_silhouette.tsv](native/native-ime_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [base/wasm/lseg-01-basic_silhouette.tsv](base/wasm/lseg-01-basic_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `54b306f061953b06b61c34f2c999a62d68bf2293e69e7774baf4d248d6adeb7c` | 136 |
| [wasm/lseg-01-basic_silhouette.tsv](wasm/lseg-01-basic_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `54b306f061953b06b61c34f2c999a62d68bf2293e69e7774baf4d248d6adeb7c` | 136 |
| [base/wasm/lseg-02-mixed_silhouette.tsv](base/wasm/lseg-02-mixed_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `265c21b915268cf6494554760c4cd116f45d9e949e871a42ee068f006c2de087` | 136 |
| [wasm/lseg-02-mixed_silhouette.tsv](wasm/lseg-02-mixed_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `265c21b915268cf6494554760c4cd116f45d9e949e871a42ee068f006c2de087` | 136 |
| [base/wasm/pic-crop-01_silhouette.tsv](base/wasm/pic-crop-01_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `7ad26872b92517b9a41a2b38f010e3b005e3f56f57ce3989a1cca93ece6bbe09` | 136 |
| [wasm/pic-crop-01_silhouette.tsv](wasm/pic-crop-01_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `7ad26872b92517b9a41a2b38f010e3b005e3f56f57ce3989a1cca93ece6bbe09` | 136 |
| [base/wasm/shape-group-02_silhouette.tsv](base/wasm/shape-group-02_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [wasm/shape-group-02_silhouette.tsv](wasm/shape-group-02_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [base/wasm/equation-lim-print2020_silhouette.tsv](base/wasm/equation-lim-print2020_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `ce8c9b8c67d2df44d724a629921e2be911bcc64671e314795e98d065d1f031ae` | 136 |
| [wasm/equation-lim-print2020_silhouette.tsv](wasm/equation-lim-print2020_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `63061d97515d2a09016bf904184b28e2207401f70a675f56a7db35655f240bc4` | 136 |
| [base/wasm/form-hwpx_silhouette.tsv](base/wasm/form-hwpx_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `b62edbd235584ed8a81745d76985a6ac02a52e5477b551231cfd174229f6be58` | 136 |
| [wasm/form-hwpx_silhouette.tsv](wasm/form-hwpx_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `b62edbd235584ed8a81745d76985a6ac02a52e5477b551231cfd174229f6be58` | 136 |
| [base/wasm/form-hwp-manual-crossformat_silhouette.tsv](base/wasm/form-hwp-manual-crossformat_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `b62edbd235584ed8a81745d76985a6ac02a52e5477b551231cfd174229f6be58` | 136 |
| [wasm/form-hwp-manual-crossformat_silhouette.tsv](wasm/form-hwp-manual-crossformat_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `b62edbd235584ed8a81745d76985a6ac02a52e5477b551231cfd174229f6be58` | 136 |
| [base/wasm/native-ime_silhouette.tsv](base/wasm/native-ime_silhouette.tsv) | `b7a5388034ee66f58a7e1d5363974e83b6374f57` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |
| [wasm/native-ime_silhouette.tsv](wasm/native-ime_silhouette.tsv) | `751e34e344d371d276917dccb06b93fa8e79eff7` | `76292db165fc1a04b17dbbd1b6444ba8cf19f40aeff5fc42900007225280ac3b` | 138 |

## 입력·Print 출처와 알려진 차이

원본과 독립 Print·MCP job·글꼴 해시·렌더 명령은 [5단계](../../../working/task_m100_536_integration_review_stage5.md), [6단계](../../../working/task_m100_536_integration_review_stage6.md), [7단계](../../../working/task_m100_536_integration_review_stage7.md)에 연결한다.
폼 HWP의 기준은 사용자가 수동 출력한 HWPX Print이며 원본 속성과 실제 렌더 대응을 별도로 확인했다.
극한 이름 face/em은 보정했지만 기존 분수 간격·폼 Edit bevel 차이를 해결했다고 주장하지 않는다.
별도 단독 극한의 낮은 진단 결과는 [진단 기록](diagnostics/README.md)에 남기며 정상 fixture나 부모 #536 전체 해결로 표시하지 않는다.

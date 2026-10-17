# PR #7494·#7751 14단계 Native/fresh WASM 시각 증거

최종 실제 렌더링 소스는 `465fd10fded3bec623d3b884461b3aa6bba9e227`다. 새 devel `b7a538803`의 기본 PDF feature를 포함해 Native와 fresh WASM을 모두 새로 빌드했다. compat2024, Print,96dpi. 입력·기준 PDF는12단계와 같다. Native/fresh WASM 대표 review·overlay는 해당 소스로 새 출력한 Skia PNG·Canvas PNG이며 SVG 래스터가 아니다. Direct PDF3쪽은 별도 실제 PDF를 래스터화했고, DOM 전후 비교의 출처는 아래에 구분한다.

원본: `samples/2025 행정업무운영 편람(최종).hwp`, SHA-256 `40d6d05eac4d55bdc4b0c62c42d93af104d5123b447581246f36fd15de7bd46f`.

독립 한컴2024+KoPub Print: `pdf/2025 행정업무운영 편람(최종)-hwp-kopub-2024.pdf`, SHA-256 `3cdf329004cffad0ebacd122586e5e1702aff36b3015cbcb401d038857019593`,383쪽. 기존 기준 PDF를 재사용했으며 다시 변환하지 않았다.

전383쪽 raw 최저는 Native93.07066%,fresh WASM92.98362%다. 90% 미만·누락·고정 devel 대비0.01%p 이상 하락은 양쪽 모두0쪽이다. 평균이나 색상 경계 보정 후 진단값으로 판정하지 않았다.

검증 기준 devel은 `b7a5388034ee66f58a7e1d5363974e83b6374f57`이다. 기준 산출물의 실제 생산 SHA는 `552514f423ec91f1b7171f6e6b0882a4efe034f2`이며, 두 SHA 사이 renderer/paint/model/document_core/serializer와 HWP parser 소스가 같음을 확인했다. PDF parser·PDF API와 기본 feature의 차이는 구분했다. 실제로 실행하지 않은 SHA로 기준 산출물의 출처를 바꾸지 않았다.

## 직접 판독의 재검토 결과

289쪽을 추가 확대 판독해 묶음 화살표의 가로 원점이 독립 Print보다 약26.3px 왼쪽인 것을 확인했다. 묶이지 않은 선은 정상 위치다. 고정 devel에도 같은 가로 차이가 있지만, 현재 #7751 보정 대상 문서이므로 이관하거나 점수로 면제하지 않는다. **raw99.6%와 전체 테스트 통과만으로 PR 준비 완료로 판정하지 않으며 그룹 원점을 보정한 뒤 재검증한다.**

## 대표 페이지 raw와 PNG

| 쪽 | Native raw | fresh WASM raw | Native review | WASM review |
| ---: | ---: | ---: | --- | --- |
| 3 | 100.00000 | 100.00000 | [PNG](native_review_003.png) | [PNG](wasm_review_003.png) |
| 90 | 93.07066 | 92.98362 | [PNG](native_review_090.png) | [PNG](wasm_review_090.png) |
| 130 | 98.69152 | 98.45828 | [PNG](native_review_130.png) | [PNG](wasm_review_130.png) |
| 224 | 99.77352 | 99.75272 | [PNG](native_review_224.png) | [PNG](wasm_review_224.png) |
| 226 | 99.79541 | 99.81998 | [PNG](native_review_226.png) | [PNG](wasm_review_226.png) |
| 241 | 98.23048 | 98.27806 | [PNG](native_review_241.png) | [PNG](wasm_review_241.png) |
| 289 | 99.59859 | 99.57324 | [PNG](native_review_289.png) | [PNG](wasm_review_289.png) |
| 290 | 99.45606 | 99.44102 | [PNG](native_review_290.png) | [PNG](wasm_review_290.png) |
| 297 | 99.23751 | 99.24374 | [PNG](native_review_297.png) | [PNG](wasm_review_297.png) |
| 375 | 99.37858 | 99.42850 | [PNG](native_review_375.png) | [PNG](wasm_review_375.png) |
| 383 | 100.00000 | 100.00000 | [PNG](native_review_383.png) | [PNG](wasm_review_383.png) |

130·375쪽의 standalone overlay도 각각 `native_overlay_*.png`·`wasm_overlay_*.png`로 보존한다. 파랑은 RHWP만,빨강은 PDF만,주황은 양쪽 색 차이,회색은 일치다. 전쪽 TSV·로그·중간 JSON은 ignored output에 보존하고 PR 본문에 전쪽 수치를 직접 표시한다.

## 고정한 완료 페이지

관계 검사와 함께3·241·290·297·375쪽의 기존 독립 좌표를 유지한다. 130쪽은 두 backend 모두 raw98% 이상인 것을 확인한 뒤 바탕쪽 그림 프레임±2px를 추가했다. 본문 WMF·동적 바탕쪽 글자의 원본 좌표 보존도 검사한다. 100%→99%대→98%대 순으로 같은 동작의 증거를 우선하되, WMF crop은 고유한130쪽에서 검사한다.

## 직접 PDF와 색조 대조

편람3쪽은 현재 Direct PDF로 다시 변환·래스터화해 같은 독립 Print와 raw100%였다.
[Direct PDF review](direct_pdf_review_003.png)와 [overlay](direct_pdf_overlay_003.png)를 보존한다.
이는 전383쪽 Direct PDF 지원을 뜻하지 않는다.63쪽 연결선은 #7501의 기존 손실 방지
계약으로,130쪽의SVG payload도 기존 미지원 계약으로 명시적으로 거부됐다.
미지원 효과 guard를 완화하지 않았다. 일반 흑백의 색조는 Native/Canvas/SVG에서
확인했으며 13단계 공통 producer가 실제 bake한 일반BlackWhite 색조는5개 실제 입력의Direct PDF에서도 전쪽raw100%로 확인했다. unbaked 또는미지원legacy조합의거부는 유지하며 이값으로지원범위를추정하지 않는다.

실제 한컴 저장·Print한5개 색조 대조군도 최종 소스에서 다시 전쪽 비교해 Native와
fresh WASM 각각5쪽 모두 raw100%였다. 대응 입력·PDF·독립RGB는
[pdf/issue7751](../../../../pdf/issue7751/README.md)에 고정했다.

## 실제 DOM의 색조 소비

[이전 DOM·독립 Print·보정 DOM 비교](dom_color_review.png)는 같은 실제 저장 입력의
밝기20·대비0 행이다. 실제 Studio 함수를 같은 SHA로 실행해 이전(96,120,144),
보정(131,151,171)을 screenshot에서 읽었으며 후자가 독립 Print와 같다.
Canvas550패치와 DOM550패치를 별도로 검사해 브라우저8건이 통과했다.
일반 raster의 PNG 확정을 공통 producer로 옮기고 B/C·효과를 포함한지연로딩키로
같은 바이트를 받는다. frontend에 색조식을 복제하지 않았다.

Native383쪽의raw는12단계와모두같다. WASM은101/103/290/304쪽의이미지영역에
최대0.00029%p의미세차이가있고render tree는같았다.90%미만이나devel대비0.01%p
이상하락은없었다. 새source의PNG로위대표review·overlay를다시생성했다.

직접 PDF3쪽 PNG와 일반 그림 색조5개도14단계 실제 제품으로 새로 변환·비교했다. 모두 raw100%였다. 전383쪽 Direct PDF 지원을 뜻하지 않는다.

전383쪽 Native와 WASM의 새 PNG·render tree는 각각13단계 생산본과 모두 byte-identical이었다.
최신 실제 DOM/Canvas 브라우저8건도 다시 통과했다. DOM 색조의 이전/Print/보정 비교 PNG는
13단계 실제 screenshot 증거이며,14단계의550개 DOM 픽셀 관측과 같은 결과임을 확인했다.

## 실제 빌드의 소스 지문

실제 source `465fd10fded3bec623d3b884461b3aa6bba9e227`, 기본 devel `b7a5388034ee66f58a7e1d5363974e83b6374f57`.
`stage14-native-provenance.json`·`stage14-fresh-wasm-provenance.json`에 명령·환경·종료코드·출처를 보존했다.
공용 `target/pr-review`·jobs8·WASM codegen16/no-opt host 경로다.

- Native binary SHA-256: `6f3a4fb6d556b4d3e51fb4e85b7c43c4488e9e14474dd187b80853d5b5aa4662`.
- fresh WASM SHA-256: `fb816b2b654458cec0b5a6a7ecdc20b9be5ed64eed04e169c6f8628507790ad6`.
- 전체 TSV·내용/소유 감사·브라우저·Direct PDF 증거는 ignored 실행 폴더에 보존하고,
  merge 후 핵심 증거는 `/home/tsjang/rhwp-review-evidence/pr7494-20261016/`에 이관한다.

## PNG SHA-256

| 파일 | SHA-256 |
| --- | --- |
| direct_pdf_overlay_003.png | `734ee88c16a6bbb7cc48e7aeb93bfa3706c394c8bad114b28c761cdbacd1c7f0` |
| direct_pdf_review_003.png | `7fe8677c20e068c6db85ad773cc14a1731235f2574e1f5d5fb96a7e0b749b97e` |
| dom_color_review.png | `e32b21aebb8fa57f3ddaf51fcfd7b6225419d6e325fd765fc8aadc03759d1361` |
| native_overlay_130.png | `44bb001b395fc8fe4288bedf9f0033158caa4c851c033e59403de31c1ed6bf66` |
| native_overlay_375.png | `1406e515da28a0ad0348ba3be976376dac52fa7428fe6fe0cc85ca2b7136700a` |
| native_review_003.png | `d31eef9249a1b433004096394a194eda56e6b7450821955c7810ddec35158990` |
| native_review_090.png | `dac1f0354fce9a8f5883422dd5b426ed79542243a7a2f1bf1d1dbe071ffde8ec` |
| native_review_130.png | `c196082088b96e002fa00f464f5bda7f747053bc529aa7ded0c9cd400d60901f` |
| native_review_224.png | `e20dbe004e8d3d75086af1df735040be77c8c35c5db0894ade7d441aeb7526b6` |
| native_review_226.png | `d8fff850e80708f6d09ff0916f74da33b33a9fe75bf88023408c7f342c410cca` |
| native_review_241.png | `4a26e9f0013d48ea24275856983fbed3a5429bf3528531a057e0aa2e33963995` |
| native_review_289.png | `eae4e9026078abe91d108cd06073aac35828cd553addab2ec59b22396132d0dc` |
| native_review_290.png | `e2f8c451cbd406bbaeaa2e8fa4608a5cc36c4065b9131e731cfb8eb2a3a9691c` |
| native_review_297.png | `ca49bf2fcbdf8cccd00d7afc6716ddeb5e20ac417b6a76c9e5a3ff85cdd02a43` |
| native_review_375.png | `47e4651e94ae87669251171ac171be1c7f48071f46b217b3ca22e7b415b92e01` |
| native_review_383.png | `1b548781d30ab8ab803381decb5830d1602f8b0d1d99afb1a3ce7fc6227677b8` |
| wasm_overlay_130.png | `36de77132a41b0685f207f4a4f4496b724c796ba0db62153a0001781ea97276c` |
| wasm_overlay_375.png | `99cd8e0db7d19b586b951b7237a457f9e38911cb8ee3a454876ba497b2b0f855` |
| wasm_review_003.png | `6009dcc964449b111cc78cfcf470cbc4814f7042093106ea21e97e13892f1e7c` |
| wasm_review_090.png | `eed38f796703de405337b16df621603e424a7c8b0e4294491cce1a6d2af63a1a` |
| wasm_review_130.png | `d69e8e046a7eeee83d7d3a64ac21d38fed866a57da113a42e7c76a72a13bb83e` |
| wasm_review_224.png | `6395aafe8e94f324cf52bbe08fed69b90549c11d9b41e7a92680d89d4e02dd22` |
| wasm_review_226.png | `bc6461db1b946995d69eaa4e927f29022e76b005bdcaf6a35130882bca88a7ed` |
| wasm_review_241.png | `a389343daed190d4f66d2b13f92ebe2b7a1fa162c9b324e7e186ed49058763d8` |
| wasm_review_289.png | `40c846eaac4c53543191f71f29463d0f95d0878e4047eebae0a620c28827fe8f` |
| wasm_review_290.png | `a50254b654484ba1362d4393f95595c3cc6133cdc03264e701de692646bab36b` |
| wasm_review_297.png | `e1bc2fdfefe2b4d2be1552aa01b6772c15e235a22d1bfa1abff51fa973a97bb8` |
| wasm_review_375.png | `c66b4e5541911850d1e2baf51f63940c5e224f6fdb4ce69772e118be5c966432` |
| wasm_review_383.png | `402b5da08027088c15bd70464ce14285d4d2d40b8d0b9f4be9805409215d6d02` |

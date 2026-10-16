# #7699 시각 증적과 페이지별 px 기준

## 검증 신원

- 기준 devel: `375aff29498166260a22796d54e67f508c84aada`. 원 PR source: `11f3e5838af0c5b5bf97da8a672ad934e1510b04`, 체리픽 `ef981eb7d96533fdca0b179505881d78c90b01d0`.
- 메인터너 production source: `5d2035bb0bb130cdae790b4f9c4f8b275cf92deb`. 이후 fixture·test·기록만 추가하며 생산 소스의 SHA-256 동일성을 확인한다.
- Ubuntu, release-test/native-skia CLI와 fresh web WASM. Cargo jobs8, 공유 `/home/tsjang/rhwp/target/pr-review`, WASM codegen-units16. Docker 실행 파일이 없어 canonical host `scripts/wasm-pack-locked.sh --target web --out-dir pkg --no-opt` 대체 경로를 사용했다.
- 동일 입력·독립 한컴2020 PrintMethod0·compat2022·96dpi·2px 내용 실루엣. Native는 실제 Skia PNG, WASM은 실제 browser Print Canvas2D PNG이며 SVG를 산출하지 않았다.
- 시스템 원본 글꼴 디렉터리 `/usr/local/share/fonts/hancom-office-2020/{All,Install,Hwp}` 및 `/usr/local/share/fonts/hwp-convert-mcp-survey`를 직접 공급했다. `휴먼명조·휴먼고딕`은 Hwp/HMKMM.TTF·HMKMG.TTF다. 글꼴 파일이나 family 변조를 PR에 추가하지 않았다. Hwp 누락 상태의 초기 진단 수치는 최종 근거에서 제외했다.
- 새 입력·기준 PDF의 바이트·SHA와 독립 생성 출처는 [fixture README](../../../../tests/fixtures/issue_7699/README.md)에 있다. 저장 LineSeg를 수동 작성하지 않았다.

## 전쪽 결과

4개 보정 대상 각3쪽과 기존 정상 대조군5개 각1쪽: **9문서·17쪽씩 Native/fresh WASM**, 페이지/backend 비교34건. 기준 대비0.01%p 이상 하락·90% 미만·누락·중복·전체 쪽수 불일치0. Decimal 원수치로 비교했다.

| 입력 | Native p1 / p2 / p3 | fresh WASM p1 / p2 / p3 |
| --- | --- | --- |
| kiit-standard | 99.98326 / 99.91808 / 99.99692 | 99.90991 / 99.71153 / 99.97540 |
| kiit-hancom | 99.98326 / 99.91808 / 99.99692 | 99.90991 / 99.71153 / 99.97540 |
| kiit-overlap | 99.98326 / 99.91808 / 99.99692 | 99.90991 / 99.71153 / 99.97540 |
| kiit-narrow85 | 99.98678 / 99.90805 / 99.99692 | 99.93899 / 99.69093 / 99.97540 |

원본/정상 HWPX/allowOverlap의 기준 p1은 Native54.09142%, WASM53.68568%였다. 좁은 표의 기준 p1은 Native91.87685%, WASM91.79435%였다. 표 폭80%만으로 예약하던 원 PR의 좁은 표 Native61.24616%는 초기 진단이며 최종 근거는 위 같은 조건의 base/head 대조다.

| 정상 대조군 | Native | fresh WASM | 기준 대비 |
| --- | ---: | ---: | --- |
| stored-offset | 99.86301 | 99.87148 | 동일 |
| reflow-wide | 99.87061 | 99.87779 | 동일 |
| reflow-narrow | 99.73828 | 99.76357 | 동일 |
| reflow-overlap | 99.87061 | 99.87779 | 동일 |
| reflow-para | 100.00000 | 100.00000 | 동일 |

TSV 원본·실행 로그·입력/PDF/글꼴/producer hash는 ignored `output/pr-review/pr7699-20261016/`에 보존한다. `base-head-page-comparison.json`은 원수치34건 및 각 TSV 경로를 기록한다. `base/fixed-{native,wasm}[-controls]/<key>/silhouette.tsv`가 전쪽 원본이다. Git에는 TSV·로그·중간 JSON을 넣지 않는다.

## 직접 판독과 범위

- 원본 p1의 표 아래 첫 본문, 두 단 시작, 타이틀과 안내 글상자, 표 바깥 여백을 Print와 직접 비교했다. 안내 글상자는 Print에도 표와 겹치므로 본문과 구분해 유지한다.
- 좁은 표 p1은 양옆 차선과 제목 뒤 빈 글줄을 확인했다. 본문을 표 아래로 강제하지 않고 빈 줄의 높이를 보존한다.
- p2 수식은 Σ·φ·괄호·이탤릭/첨자·식 번호와 양쪽 본문 간격을 같은 배율로 확대 판독했다. 표/그림·캡션·참고문헌도 확인했다. 좁은 표로 한컴이 재조판한 p2는 별도 Print와 읽었다.
- p3 전체 서식 안내 표와 쪽번호를 확인했다. 원본/정상 HWPX/allowOverlap은 backend별 전3쪽 실제 RGB 동일, 좁은 표 p3도 동일하다. 동일 raster의 판독을 중복 실행한 것으로 세지 않는다.
- 실루엣 점수는 전체 픽셀 정확도100%를 뜻하지 않는다. 기존 래스터·획 굵기 차이는 남지만 주장한 본문/표 위치·줄 소속·내용 순서·수식 기호와 간격에서 새 차이는 확인되지 않았다.
- #7548의 모든 형상·모든 편집 경로·중첩 표 이어받기 일반성을 해결했다고 확대하지 않는다. 기존 정상 관계 검사는 유지한다.

## px 회귀의 독립 기준

완료 p1의 양쪽 최솟값은 원본3종99.90991%, 좁은 표99.93899%다. 변경의 핵심이 p1에 있으므로 무관한 높은 p3를 선택하지 않았다. 같은 핵심 동작을 검증할 수 있는99%대 완료 페이지를 사용했다. 아래72pt 문자 origin을96dpi로 환산하고 **±2px**로 검사한다. 문서 전3쪽의 내용 순서·누락·중복·쪽수 및 포함/비겹침 관계를 함께 유지한다.

| 입력 | Print 페이지/문자 | 독립 y 기준선(px) |
| --- | --- | ---: |
| 원본 HWP·정상 HWPX·allowOverlap | p1 `서  론` | 692.9195963541666 |
| 동일3종 | p1 첫 단 `비접촉…` | 733.877197265625 |
| 동일3종 | p1 둘째 단 `가진다…` | 691.7996419270834 |
| 좁은 표 | p1 첫 단 `비접` | 179.34954833984375 |

정식 검사: `tests/cases/issue_7699_stored_square_top_band.rs`, 총5함수. 원 PR 라이브러리로 같은 정식 소스를 빌드·실행하여2 PASS/3 FAIL을 확인했다. allowOverlap의 본문 겹침, 좁은 표의 과도한 밀어내기, 빈 줄 누락이 각각 독립적으로 실패한다. 보정 후보의 대응 검사5 PASS 뒤 공식 suite 결과는 개별 review에 연결한다.

## 대표 PNG

![Native 원본](native-standard-review.png)
![fresh WASM 원본](wasm-standard-review.png)
![좁은 표](native-narrow85-review.png)
![수식 확대](zoom-equation.png)

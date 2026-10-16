# #7419 Stage 3 — 전체 검증과 제출 판정

제품 코드 `a08cf5cc6`, 관계 회귀 `fd8ed2451`.
Stage1 전쪽 시각 증거와 Stage2 수정 전후 실행 증거를 재사용한다. Rust 제품 코드는 바뀌지 않았다.

## 원본 코퍼스 등록

원 PR에서 새로 넣은 `samples/issue7419/`의2개 원본은 독립 Print1쪽과 일치하며 전쪽 양쪽100%다.
이2개만 `oracle_page_count_baseline.tsv`에1/1로 신규 등록했다. 기존 허용치를 변경하지 않는다.
원본 XML은 수동 작성이며 `lastSavedWith` 표기는2024지만 실제 한컴 생성 출처로 인정하지 않는다.
페이지 기대값은 해당 바이트에 대한 새 한컴2024 Print에서 정했다. 입력·PDF SHA는 Stage1에 있다.
추가 반례7개는 `tests/fixtures/`의 목적별 입력이며 Stage2의 직접 내용·쪽 소유 검사가 담당한다.

`layout-anomaly --json`의 기본 네 방향 overflow는 중첩 원본에서 base/head 모두1이다.
이는 오른쪽4.7733px이며 본문 바닥(`body_overflow_baseline`)의 집계 대상이 아니다. 다른 래칫의
기존 수치를 이 값으로 추정하거나 허용치를 추가하지 않았다. 전체 회귀의 실제 결과로 확인한다.

## 실행 중인 필수 검증

`output/pr-review/pr7433-current-20261016/validate.py`에서 Cargo 잡8개, 고정 공용 target으로
fmt·native/WASM clippy·workspace build/all-target clippy·suite manifest, release-test 전체,
Native Skia3종을 순서대로 실행한다. Cargo 계열 명령은 겹치지 않는다. 명령·head·exit·시간은
`{lint,full,native}-validation.json`, 원시 결과는 각 이름의 log에 남긴다.

시작 전 원격 devel도 다시 확인했고 `375aff29498166260a22796d54e67f508c84aada`로 변경이 없었다.
따라서 불필요한 rebase는 수행하지 않았다. PR 직전에는 다시 확인한다.

전체 회귀·lint·Native Skia의 완료 결과와 PR gate 판정은 실제 종료 뒤 추가한다.

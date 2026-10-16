# #7419 Stage 3 — 전체 검증과 제출 판정

제품 코드 `a08cf5cc6`, 관계 회귀 `fd8ed2451`.
아래 추가 보정 전의 기록은 Stage1·Stage2에 있다. 이 단계에서는 분할 경로 대조로 제품 코드도 추가 보정했으므로 최종 source의 시각·전체 게이트를 다시 수행한다.

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

## 분할 경로 대조에서 추가 보정한 범위

Stage1은 원본·선언 높이·TAC 전체 이월을 확인한 시점의 기록이다. 후속 코드 대조에서 분할 셀의
`CellComposedStore` eager/lazy는 다른 조판 호출을 사용하고 있었으므로, 셀 컷과 실제 조각 배치가
`compose_cell_unit_paragraph`를 함께 사용하도록 바꿨다. 다음은 같은 입력의 직접 반례다.

| 입력 | 독립 Print 계약 | 보호할 경계 |
| --- | --- | --- |
| `partial-nested-width-wrap.hwpx` | 2쪽. 첫 자식은1쪽, 둘째는2쪽, 후속 본문은2쪽 | 줄 소속0/8 UTF-16, 시작·끝 컷, 실제 수용 높이와 내용 순서 |
| `partial-nested-wrapper.hwpx` | 2쪽. 부모 제목은1쪽, wrapper 전체와 두 자식·뒤 본문은2쪽 | 한 쪽에 들어가는 TAC wrapper의 통째 이월, 여러 자식 보존·종료 |

두 입력은 폭 부족 원본에서 용지45000HU, 부모 표를 비TAC로 바꾸고 후속 본문을 붙였다.
둘째는 그 두 자식을1행1열 wrapper에 넣었다. 표·문단 id는 충돌하지 않게 바꾸고 실제 wrapper
셀 주소를(row0,col0)으로 확인했다. 초기 ZIP 제작 오류 및 rowAddr1인 잘못된 초안은
`invalid-input-proof/`에 분리했고 검증 근거에서 제외했다. 잘못된 초안 변환 job2개는 failed로
끝났으며 정정 입력의 새 Print job `cd8d7b6f-d27e-48fd-a7cd-91d0d7da19dd`는 succeeded였다.
서버 endpoint/token은 출력하지 않았다.

### 현재 결과의 생산과 소비

- `CellComposedStore`의 양쪽 구성이 공통 셀 조판을 호출한다. 원래 문단과 분할 줄 창은 보존한다.
- 여러 TAC를 소유한 현재 프레임의 각 물리 줄이 `CellUnit.vis_start/vis_end`로 전달된다.
  같은 줄의 두 표 높이를 합산하지 않고 다른 줄의 표를 하나의 atomic 합으로 바꾸지 않는다.
  `advance_row_cut_for_physical_band`의 실제 수용 유닛과 원래 줄 창을 paint가 함께 소비한다.
- 부모의 단일 TAC wrapper는 독립 Print처럼 통째 이월한다. 내부 두 줄이 존재한다는 이유로
  부모를 분할 가능으로 승격하지 않는다. 그 가정을 실제 Print에 따라 제거했다.
- `ComposedParagraph::control_line_indices`가 저장 줄 또는 현재 프레임의 원시 슬롯 소유를
  발행한다. partial의 줄별 폭, 선택 창의 개체 보존, 인라인 pen과 `resolved_table_left`가
  이를 소비한다. 이 pen 뒤의 재앵커 선택까지 직접 확인했다.
- `layout_paragraph_in_frame_impl`은 들여쓰기 표시를 반환용 LineSeg뿐 아니라 확정할 실제
  RowSegment.source_tag에도 보존한다. 프레임을 다시 읽는 소비자가 내어쓰기 후속 줄의
  가로 원점을 잃지 않는다. 보정 도중 기존 완료 좌표 검사가 이 누락을 FAIL로 잡았고
  반환 사본만 고치는 대신 생산자를 수정했다.
- 상세 `MeasuredCell`의 NO_LS HWPX 가로 문단도 같은 현재 프레임을 쓴다. line_heights,
  para_line_counts, 자기 프레임의 분할 줄 수가 실제 셀 컷과 대응한다. HWP5·legacy·실제 저장 줄의
  상세 원장은 이 변경의 대상이 아니다. 이미 흡수한 본체를 원본 합성 높이로 다시 더하지 않도록
  미흡수 높이 보정도 현재 프레임 문단으로 수행한다.
- `sole_nested_table_in_paragraph`는 단일 표와 배치 상자가 없는 마커만 허용한다. 측정·전체 배치·
  partial wrapper·typeset의 해제 후보가 이 계약을 사용한다.1×1 모양만으로 첫 표를 고르면
  두 번째 자식이 사라지므로, 여러 배치 소유자가 있으면 wrapper 셀을 유지한다.
  기존 단일 표+폭 중립 마커의 허용 계약은 정적 대조했고 전체 회귀에서도 확인한다.

기존 컷의 rowspan/저장 프레임/캡션/각주 알고리즘을 별도로 재작성하지 않는다. 새 입력은
rowspan·캡션·각주가 없으며 그 조합의 신규 시각 일치 주장을 하지 않는다. 분할 뒤 원문 전체와
후속 본문, 마지막 자식의 소유·중복·누락과 불필요한 빈 쪽까지 직접 비교한다.

추가 구현으로 Stage3 최초 lint 묶음의 workspace build를 중단했다. 그 전 fmt/native/WASM clippy의
성공은 당시 source의 기록이며 새 최종 source의 성공으로 재사용하지 않는다. 중단 exit -2는
제품 테스트 실패나 완료로 계산하지 않고 최종 source에서 필수 묶음을 다시 실행한다.

## 원본 식별자와 최신 기준 확인

재조판 `paragraph_with_frame_geometry`는 줄 좌표·높이를 읽기 위한 임시 뷰다.
`cell_units_cache`는 원본 `Cell` 주소를 key로 유지하므로 그 복사본의 Table/Cell을 넘기지 않는다.
whole 셀 배치, partial 조각 배치, 셀 유닛의 실제 개체 순회는 원본 controls를 유지하고,
줄 상자·슬롯 소속·pen은 재조판 프레임을 소비한다. 모델 식별자와 줄 좌표의 계약을 구분했다.

추가2개 반례를 포함한 Native/fresh WASM 11문서·15쪽 모두100%를 확인한 후 분할 관계2개와
그4쪽의 독립 Print 제목/후속 본문 ±2px 검사를 보완했다. 원본 식별자 보완 직후 Native 전쪽을
다시 산출했다. fresh WASM과 전체 Rust의 최종 결과는 리베이스 후 실행 결과로 별도 고정한다.

원격 head가 `375aff294`에서 `4662c069e`(#7913)로 변경됐다. 현재 보정을 커밋한 뒤
리베이스하며, 새 기준 producer와 후보 producer를 다시 비교한다. 사용자의 최신 지시에 따라
PR 생성은 검증 결과와 본문을 준비한 뒤 별도 승인을 받아 진행한다.

현재 기준의 page-count gate도 유지한다. 이전 base375aff는 `page-height-28000`에서2쪽/Print3쪽,
`partial-nested-width-wrap`·`partial-nested-wrapper`에서 각각1쪽/Print2쪽이었다.
의도한 복구라도 base TSV 누락·쪽수 차이를 승인으로 바꾸지 않는다. 새 base의 동일 조건을
확인한 후 `visual_verification_governance.md`의 전쪽 일대일 비교 판정에 기록한다.

### 여백 앵커 회귀의 검출과 보정

원본 식별자 보완만 적용했을 때 `nested-outer-margins` Native가93.64997%로 낮아졌고
관련12개 중 여백 관계·완료 쪽 ±2px 검사2개가 실패했다. 새 기대값을 만들지 않고 원인을 고쳤다.
실제 표는 원본 주소를 유지하되, `table_band_hu`와 바깥여백을 포함한 줄 높이 비교에는
`FrameGeometry::inline_table_height_hu`의 현재 본체 높이를 소비하도록 수정했다.
`expanded-targeted-tests.log`와 `frame-height-final-regression-proof.log`는 같은 검사 실행 파일로
수정 전10통과/2실패 → 수정 후12통과/0실패를 대조한다. 최종 시각 증거는 새 source에서 전쪽 재산출한다.

# PR #7699 구현 검토와 메인터너 보정

## 적용 규칙

- Square는 글 배제이며 allowOverlap은 개체끼리의 겹침 허용이다. 동일 HWPX에서 allowOverlap만 0→1로 바꾼 독립 Hancom Print 전3쪽이 래스터 동일하다.
- 본문 폭80%라는 비율로 전폭 띠를 강제하면 실제 옆 차선이 남는85%폭 표에서도 본문을 모두 아래로 보낸다. 정상 Hancom 저장으로 LineSeg를 다시 만든 반례는 실제로 옆 차선을 사용한다.
- 재조판과 동일한 LayoutFrame::carve가 본문 상단에서 사용 가능한 차선을 판정한다. 원본 저장 줄의 시작이 표·바깥여백 하단을 소유하는 경우에만 전단 상단 예약을 만든다. 중간/본문 밖·인라인·문단 기준·캡션·편집/합성 저장 줄은 새 경로를 사용하지 않는다.
- 빈 줄의 앞/뒤 저장 사다리가 실제 줄 높이+뒤 간격을 닫으면 본문 점유가 있다. 화면 글자가 없다는 이유로0높이로 흡수하면 안 된다.
- 저장 원점 비교는 HWPUNIT 정수로 수행한다. 동일한43240HU 경계도 px 변환 합산 순서에서1e-13px 차이가 나므로 부동소수점 비교는 줄 소유 증거가 아니다.

## 생산→측정→실제 배치

1. float_placement/stored_square_top_band.rs::table_reserved_height_hu: 기존 calculate_shape_reserved_heights의 셀/패딩 높이 producer를 그대로 이동해 공유한다. 입력 table IR을 변형하지 않는다.
2. stored_square_top_band: 실제 수평 anchor와 absolute_table_vertical_geometry의 동일 세로 원점·바깥여백 → LayoutFrame의 기존 차선 배제 → 원본 LineSeg의 시작 소유를 결합해 절대 하단을 반환한다.
3. typeset.rs::compute_body_wide_top_reserve_for_para → section/post_flow → reserve_body_wide_top → state/transition::advance_column_or_new_page: 다음 단의 current_height/current_start_height는 같은 하단에서 body.y를 뺀 상대값이다. zone 원점은 이동하지 않는다.
4. layout/shape_layout.rs::calculate_shape_reserved_heights 및 calculate_body_wide_shape_reserved는 같은 절대 하단을 받는다. 신규 Square 경로는 legacy TopAndBottom의80%/상단1/3 및 별도높이 재계산으로 내려가지 않는다.
5. layout.rs::build_single_column은 body_wide_reserved로 y_offset/col_anchor_y를 같은 하단에서 시작하고, 기존 저장 줄의 상대 간격을 소비한다. 타이틀 표 자체의 Page/Paper anchor는 변경하지 않는다.
6. controls/wrap_flow::place에서 앞뒤 원본 저장 줄이 닫는 빈 줄은 기존 wrap anchor를 등록한 채 일반 paragraph fit/paint로 보낸다. 0높이 wrap absorption에 넣지 않는다. 원본이 아닌 입력·편집 세션·dirty ladder는 새 소유 증거로 인정하지 않는다.

## 분할/이어받기 적용 여부

이번 변경은 표 셀 내용 컷/rowspan/continuation을 변경하지 않는다. 실제 반례의 본문은 PartialParagraph로 단·쪽을 이어받으므로 원본 pi0/pi2 전내용 순서·누락·중복, 둘째 단 소유 및 전체 Print3쪽을 최종 render tree에서 검사한다. 표 내부 분할의 일반성까지 새로 해결했다고 주장하지 않는다.

## 수정 전 관측

- 최신 devel375aff: 원본/정상 HWPX/allowOverlap p1 Native54.09142%, WASM53.88339%.
- 원 PR ef981: 원본/정상 HWPX Native99.98326%, allowOverlap54.09142%.
- 좁은 표: devel Native91.87685% → 원 PR61.24616%. devel에도 빈 줄20px 점유 누락이 존재한다.
- 보존한 원 PR 라이브러리에서 동일 회귀 후보4건: 원본/정상 HWPX2 PASS, allowOverlap/좁은 표2 FAIL. 환경/빌드 실패를 재현으로 세지 않는다.
- 보정1의 새 좁은 표 검사 PASS, 전폭3건 FAIL: 위 부동소수점 경계 판정 오류를 발견해 HWPUNIT 비교로 바로잡았다. 이 실패 후보는 제출하지 않는다.

## 보정 후 시각 검증

생산 소스 `5d2035bb0bb130cdae790b4f9c4f8b275cf92deb`를 Native/fresh WASM 각각 빌드해 보존했다. 최종 글꼴 목록에 실제 Human 글꼴 설치 경로 Hwp도 포함하여 base/head를 모두 다시 실행했다. 초기 누락 상태의 WASM 점수는 최종 판정에서 제외했다.

4개 대상 각3쪽 및 기존 정상 대조군5개 각1쪽, 양쪽 각17쪽을 같은 Print·compat2022·96dpi로 비교했다. 최저99.69093%, 90% 미만·쪽수 차이·누락·중복0. Decimal로 비교한34개 페이지/backend 쌍에 devel 대비0.01%p 이상 하락0이다. 원본·정상 HWPX·allowOverlap의 전3쪽 실제 RGB는 backend별 동일하며, 좁은 표 p3도 동일하다.

[입력 SHA·전쪽 수치·대표 PNG·독립 px 기준](../assets/pr7699/README.md)을 같은 PR에 보존한다. 원본 p1 표 아래 본문·두 단·원래 겹친 안내 글상자, 좁은 표의 옆 차선·빈 줄, 원본/좁은 표 p2의 표·그림·캡션·수식·번호·참고문헌과 p3 안내 표를 직접 읽었다. 수식 Σ/φ·괄호·첨자·이탤릭·식 번호 및 주변 간격은 같은 배율의 Native/Print/fresh WASM 확대 PNG에서 확인했다. 기존 래스터 획 차이를 전체 정확도100%로 표현하지 않는다.

## 정식 회귀

`tests/cases/issue_7699_stored_square_top_band.rs`의5함수는 다음 의미를 보호한다.

- 원본 HWP와 정상 HWPX 각각: 전3쪽·pi0/pi2의 전내용 순서/누락/중복·양단 본문 소유·표/바깥여백 아래·제목 뒤 본문 순서.
- allowOverlap: 같은 본문 배제 관계 유지. 원본 속성 하나만 바꾼 독립 Print는 전쪽 동일하다.
- 좁은 표: 실제 옆 차선, 표 내부 비겹침 및 Print 첫 본문 기준선.
- 빈 글줄: 제목과 본문 사이의 소유·한 번만 존재·점유 보존. 가시 글자 유무를 흐름 유무로 대용하지 않는다.
- 모든 입력: Print에 의도적으로 겹친 안내 글상자는 숨기거나 중복하지 않는다.

완료 p1의 양쪽99%대 일치율과 직접 판독을 근거로 Print 문자 origin 기준선을 ±2px로 추가했으며 관계 검사도 유지한다. 원본3종의 첫 단/둘째 단/제목과 좁은 표의 첫 본문을 각각 보호한다. 무관한 높은 p3 대신 핵심 동작의 높은 완료 페이지를 선택했다.

보존한 원 PR 라이브러리와 동일한 정식 테스트 소스의2 PASS/3 FAIL은 `output/pr-review/pr7699-20261016/logs/pre-fix-formal-regression.log`다. 과도한 밀어내기·allowOverlap 겹침·빈 줄 누락이 각기 실패한다. 후보 standalone 검사는5 PASS이며 공식 suite·전체 검증의 실제 결과는 최종 개별 review에 연결한다. 빌드/환경 오류를 결함 재현으로 세지 않았다.

## 적용 범위와 남은 한계

- 생산→페이지네이션 예약→paint 시작 원점의 동일 결과 공유는 위 세 소비 지점과 본문/다음 단의 실제 render tree로 확인했다. helper 호출 뒤 legacy 재계산으로 값을 바꾸는 신규 경로는 두지 않았다.
- 셀 내용 컷·rowspan·continuation의 규칙은 변경하지 않는다. 본문의 PartialParagraph 단·쪽 이어받기는 전내용 보존과 전체 쪽수로 함께 검사한다.
- 별도 캡션 상자, 편집 후 저장 줄의 재사용, 다른 anchor 정렬과 일반적인 선언/실측 높이 차이까지 해결했다고 주장하지 않는다. 해당 조건은 기존 경로에 남기고 #7548 전체를 종료하지 않는다.
- 기존 정상 대조군의 관계 검사와 golden/baseline은 완화·제거하지 않았다. 새 source·fixture·대표 PNG만 추가한다.


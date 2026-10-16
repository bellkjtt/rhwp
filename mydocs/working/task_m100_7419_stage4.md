# #7419 Stage4 — 저장 줄이 다른 TAC 형제의 프레임 소유

## 독립 입력과 수정 전 관측

Stage3의 `59d87766f`에서 새 적용 경계를 확인했다. 기존 수동 XML에 저장 줄을 임의로
덧붙이지 않고, `partial-nested-width-wrap.hwpx`를 한컴2024에서 HWP로 저장한 뒤 HWPX로
다시 저장했다. 실제 저장한 둘째 자식의 LineSeg는 유지하고 부모 host와 첫 자식의 줄만
제거한 혼재 입력을 만든다. 부모 host와 두 자식의 줄을 모두 제거한 대조군도 만든다.
LineSeg 제거 외의 표 폭·높이·문단/글자 속성은 두 변형에서 동일하다.

| 변형 | 독립 Print | 59d Native | 직접 관측 |
| --- | --- | --- | --- |
| mixed-child-storage | 2쪽 | 55.80081% / 68.31124% | 첫 자식이1쪽에서 빠지고2쪽에서 두 자식이 겹침 |
| no-child-storage | 2쪽 | 100% / 100% | 자식1은1쪽, 자식2와 뒤 본문은2쪽 |

두 Print의 페이지별 PNG가 바이트 동일하다. 저장 줄 혼재를 화면 차이의 정답으로
간주하지 않는다. 원본 저장 job `94075ab2-d378-4fd3-9eee-0050640e0f9b`, HWPX 저장 job
`7a141687-fbad-416c-8ae5-a5cee8d63e2d`, Print job은 각각
`d98accb7-7e02-4387-90c7-06ff11afa887` / `ba87fd38-33b3-4a3d-9c95-c447e1bb74da`다.
파일·SHA·제거한 문단/줄 수와 실제 한컴 버전/Print 경로는
`output/pr-review/pr7433-current-20261016/regression-fix/mixed-storage/{jobs,input-generation}.json`에 있다.

## 생산과 소비의 불일치

`composer/nested_table_frames.rs::recompose_cell_lines_with_nested_tables`는 현대 NO_LS
host를 재조판할 때 NO_LS 자식만 `FrameGeometry.inline_table_heights`에 넣었다.
실제 줄 구성은 저장 줄 자식을 포함한 전체 controls로 만든다. 그런데 셀 유닛의
`frame_owns_multiple_inline_tables` / `frame_owns_table_bands`와 partial paint의
`frame_owns_inline_table`은 일부 자식만 소유한다고 읽어 서로 다른 예약/배치를 한다.

`실측 형제 목록 → 재조판 줄/높이 → HeightMeasurer 상세 줄 → CellUnit 물리 줄 컷 →
table_partial의 같은 줄 소유·앵커`가 같은 전체 TAC 형제 목록을 소비해야 한다.
저장 줄이 있는 자식의 내부 줄을 다시 나누는 수정은 필요하지 않다.

## 보정과 비적용 계약

- 새 프레임의 시작 조건은 유지한다: 현대 비Native/비legacy NO_LS host에 NO_LS TAC 자식이
  하나 이상 있어야 한다. 모두 저장 줄인 일반 host를 새 경로로 넓히지 않는다.
- 그 시작 조건이 성립하면 같은 host의 **모든 TAC 표** 실측 높이를 기존 프레임에 전달한다.
  원본 자식의 LineSeg·controls 식별자는 보존하고 파생 프레임의 소유/본체 높이만 공유한다.
- Native HWP5·HWP3와 유효 저장 host의 기존 수용 조건은 유지한다. 문서 ID·수치 예외,
  clamp·겹침 숨김·허용치 변경을 추가하지 않는다.
- 새 입력은 먼저 Native/fresh WASM 전쪽90% 이상과 독립 Print 직접 판독을 확보한다.
  그 뒤 관계 검사와 자격을 충족한 완료 쪽의 독립±2px 보호를 보완한다.
  기존11문서와 Native 정상 대조군도 보정 source에서 다시 확인한다.

59d의 전체 검증은 새 실패 확인 후 SIGINT로 중단했다(pipeline exit130, test summary 없음).
이 실행을 제품 test FAIL이나 전체 PASS로 표시하지 않는다. 코드 보정 후 선행 경계/시각 판독을
마친 뒤 최종 전체·lint·Native Skia 묶음을 다시 실행한다.

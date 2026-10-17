# #7751 289쪽 셀 안 묶음 화살표 원점 — 15단계

시작 head는 `5060c4bb2`이며14단계의 실제 제품 source는`465fd10fd`다.
14단계 전체 Rust10,594건·Skia4,043건·모든 lint/정책과 전383쪽 점수 검증은
종료코드0으로 끝났지만 추가 확대 판독에서 아래 배치 차이를 발견했다.
해소 전에는 PR을 생성하지 않는다. 같은 작업의 보정·재검증 승인은 유지한다.

## 독립 근거와 대상

같은 원본 HWP·기존383쪽 KoPub 한컴2024 Print·96dpi를 유지한다.
대상은10구역28문단의 표,4행2열 셀 안 두 묶음 개체와 일반 선이다.
독립 PDF의B/f* 경로를 CTM 적용 후 좌상단CSS px로 변환해 대조했다.

| 개체 | 현재 Native/Canvas | 독립 Print | 관측 |
| --- | ---: | ---: | --- |
| 첫 묶음 첫 가로 화살표 x | 232.5 | 258.76 | 약26.26px 왼쪽 |
| 두 번째 묶음 첫 가로 화살표 x | 230.2 | 256.5227 | 약26.32px 왼쪽 |
| 묶이지 않은 대각선 x | 365.3 | 364.8853 | 약0.4px 이내 |

원 devel baseline도 묶음의 같은 x를 내므로 이번 색조 변경의 신규 회귀는 아니다.
세로 원점은 앞 단계 표 흐름 수정으로 개선됐다. 현재 보정 문서의 미해결 배치이므로
이관하거나raw99.6%·테스트 통과로 면제하지 않는다. Direct PDF의Path 연결선 미지원
#7501과 이Line/Group의실제 원점 문제도 구분한다.

## 원인 추적·보정 계획

1. 원본 셀 문단의 저장 LineSeg·문단 여백·group common/행렬을 읽고 독립 기대 원점을
   연결한다. 합성한 메타데이터로 수용 조건을 완화하지 않는다.
2. 셀 안 그룹의 원점을 생산하는 호출부에서 측정·실제 배치·하위 행렬 소비까지 추적한다.
   그룹 자체 bbox나 자식 render_tx를 임의로 더하는 보정 대신 잘못 선택한 원점 가정을 제거한다.
   단일 선·일반 도형·중첩 그룹·회전 경로의 적용 여부를 확인한다.
3. 실제 입력의 독립 좌표/포함·원본 변환 관계로 before FAIL/after PASS를 확인한다.
   관련 formal 회귀는 `tests/cases/`에 두고 source-side 새 단위 함수/모듈은 추가하지 않는다.
4. 영향 쪽 Native/fresh WASM·정상 대조를 먼저 확인하고 최종 source의 전383쪽을 갱신한다.
   90% 최소·devel 대비 하락·완료 쪽98%·직접 판독 조건을 그대로 적용한다.
5. source 변경 뒤 필수 회귀·Skia·lint·브라우저 검증을 완료하고 승인된 PR·같은 PR의
   trailing archive review/오늘할일·CI·merge·후속 처리로 이어간다.

독립 PDF path·baseline/현재 그룹 노드·확대 PNG·실행 상태는 ignored
`output/pr-review/pr7494-20261016/stage14-p289-*`·`stage15-*`에 보존한다.

## 실제 호출 경로와 첫 보정

원본 문단1206의 원시 왼쪽 여백4000(1/2 HWPUNIT)이 resolved26.666667px이며
저장 LineSeg.column_start=2000HU와 일치한다. 들여쓰기−2136은 첫 글줄 계약이며
문단 기준 떠 있는 개체 원점에 더하지 않는다. 그룹별 값·문서 식별자로 분기하지 않는다.

- 전체 셀 `table_layout.rs::layout_cell_content`의 도형 호출 두 곳,
  부분 셀 `table_partial.rs`의 도형 호출 두 곳,
  내장 표 `table_cell_content.rs::layout_embedded_table`의 도형 호출이 원본para_shape_id를 전달한다.
- 공통 `layout_cell_shape_with_parent_path`가 resolved 문단 좌우 여백으로 같은 가로 기준
  원점/폭을 만들고 Left/Center/Right 배치가 소비한다. 세로 원점·높이·흐름 예산은 바꾸지 않는다.
- `layout_shape_object`의 Group 분기는 그 최종 원점을 받아 자식의 누적 행렬 render_tx/ty에
  한 번 더한다. 중첩·회전 그룹의 행렬이나 bbox 원점을 다시 누적하지 않는다.
- TAC는 이미 예약한 글줄 좌표를 그대로 사용하고 문단이 아닌 가로 기준에는 여백을 더하지 않는다.
  문단 여백0도 종전과 같다. 그림·표의 별도 위치 계약은 변경하지 않는다.
- 본문 `shape_layout.rs`의 문단 기준 원점은 이미 같은 resolved 좌우 여백을 사용한다.
  가로 원점만 변경하므로 측정의 요구 높이·분할 컷은 동일하며 최종 도형 배치가 이 결과를 소비한다.

수정 전 실제465fd10fd 라이브러리로 독립 Print 좌표 계약을 컴파일(exit0)한 뒤 실행했다.
첫 그룹 x232.466667 대 Print258.76, 차이−26.293333px로 assertion이 실패(exit101)했다.
빌드 실패가 아닌 실제 배치 결함 재현이다. 명령·stdout·stderr는
`stage15-origin-before-command.json`, `logs/stage15-origin-before-{build,test}.log`에 보존한다.
새 정식 테스트 추가는 영향 페이지와 전쪽 Native/fresh WASM 확인 뒤 진행한다.

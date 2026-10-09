# 이슈 #7642 Stage 52 — 미주 안의 흐름 표와 행 이어받기

Issue: #7642

## 독립 근거와 원본 계약

현재 source는 `1af0ae1790578ad04a0530cbb1d398e58255ded2`이며 Stage51의
TAC 문단 점유 보정 검증은 진행 중이다. 이번 분석은 별도 Table 항목으로
배치하는 non-TAC 표의 경로를 확인한다. 같은 범위의 최종 판정을 대신하지 않는다.

11월 구분선 없음 HWPX의 원본 본문 문단 162, 미주 19 내부 문단 3은
non-TAC·TopAndBottom·flow_with_text 표다. RowBreak이고 3행·6열이며
rowspan은 없다. 표 선언 높이는 1182 HWPUNIT, host 저장 줄 높이는 900이다.
이 값 하나로 표 전체의 물리 높이를 정할 수 없다.

독립 한컴 2024 Print 13쪽에는 앞 단 하단에 첫 두 행, 다음 단 상단에
마지막 행이 있다. Native는 다음 단에 세 행 전체를 배치한다. 이전 source의
13쪽 수치는 63.65353%다. 출력 위치는 ignored
`stage52-print-split-table-rows.json`의 PDF 해시와 괘선 관측으로 고정했다.
관측한 px는 진단용이며 코드·회귀 기대값으로 쓰지 않는다.

## 실제 호출 경로의 차이

미주 경로는 `paragraph.rs → query_rewind_tail →
no_separator_tail_table_starts_next_column → 단 이월 → emit_endnote_split →
Table`이다. 본문 흐름 표는 `format_table → typeset_block_table →
prepare_block_table_entry → scan/continuation → PartialTable`을 사용한다.
미주는 이 행 분할 경로를 호출하지 않는다. body의 측정 원장도 가상 미주
문단의 표를 포함하지 않아, 단순히 본문 함수를 호출하면 측정 없음에 따른
통째 강제 배치로 다시 빠진다.

## 연결할 공통 결과

- 원본 소유를 유지한 가상 미주 표를 같은 profile·styles·normalization으로
  한 번 측정하고, 페이지네이션과 실제 배치에 그 결과를 전달한다.
- 현재 단의 실제 흐름 원점과 표의 행 측정을 일반 표 포맷·분할 경로에 넘긴다.
  저장 줄 높이나 legacy 누계로 행 예산을 대신하지 않는다.
- 조각의 시작/끝 행·컷과 소유 유닛, 요구 높이, 누적 예약, 예산 실패 시
  이월, 최종 PartialTable 배치를 실제 경로에서 대조한다.
- non-TAC 흐름 표와 배경·전경·Square 어울림 개체를 구별한다. 별도 float
  배치 계약을 강제로 block 표로 바꾸지 않는다.
- 반복 머리행은 원본 header cell 속성을 확인한다. repeat_header 비트만으로
  모든 조각에 첫 행을 추가하지 않는다.

정상 본문 표의 분할 알고리즘을 복제하지 않고 기존 경로와 결과를 사용한다.
새로운 문서 ID·고정 높이·행수별 예외를 넣지 않는다. 현재는 분석 단계이며
아직 이 경로를 구현·실행했다고 보고하지 않는다.

## 검증과 보존

11월 HWP/HWPX 변형의 최초 차이 페이지, 앞뒤 단, 마지막 조각의 뒤 문단을
Native/fresh WASM과 Print로 직접 비교한다. 행·유닛 누락/중복, 표 외곽과
다음 내용, 불필요한 빈 페이지도 확인한다. 행 분할에 영향이 있어 최종
쪽수·시각 판정에는 전체 페이지 TSV가 필요하다.

미달이나 측정 불가이면 회귀 기대값·baseline은 변경하지 않는다. 모든 관련
페이지 90% 이상과 수식·간격 확대 판독 뒤 기존 px 검사를 소유·행/유닛
보존·앞뒤 순서·포함·겹침 관계로 전환한다.


## 원본 표의 측정 경계 확인

2024년 11월 구분선 없음 HWP와 HWPX의 원본 본문 문단 162, 미주 19
내부 문단 3을 source `b616c877624d1740b79a75b40b2a9bfcadacf6fc`의
HeightMeasurer로 독립 진단했다. 양쪽 모두 총 높이 47.28, 행 높이
15.76 × 3, 누계 [0, 15.76, 31.52, 47.28]이다. repeat_header는 true지만
has_header_cells는 false다. 첫 행을 반복 머리행으로 복제할 근거가 없다.
표 개체 선언 높이 1182 HWPUNIT은 한 행과 같고 세 행 전체 높이가 아니다.
독립 Print의 첫 두 행/마지막 한 행 분할과 원본 셀 높이가 대응한다.

이 진단은 원본 표의 HeightMeasurer 결과이며 일반 표 fit·분할·renderer
normalization의 실행을 대신하지 않는다. 라이브러리 source, 진단 소스 및
실행 파일 해시와 두 원본 해시는 ignored
`stage52-owned-table-measure-probe.json`에 기록했다.

실제 전달 경계는 StateView의 미주 측정 원장 → PaginationResult →
DocumentCore 렌더 캐시의 section별 measured_tables → 최종 Table/PartialTable
배치다. 현재 캐시는 본문 MeasuredSection.tables만 받으므로 가상 미주
표를 조판 시 측정하기만 해서는 paint가 같은 결과를 받지 못한다.
전체·부분·pending 섹션 페이지네이션 각각의 캐시 저장 위치도 함께 수정해야 한다.

분할 함수 호출에는 본문과 현재 미주 문단을 같은 전역 인덱스로 연결한
읽기 문맥이 필요하다. 다음 원본 문단·표의 속성 조회도 이 문맥을 사용한다.
기존 본문 분할은 미주 소유를 자동으로 게시하지 않으므로 단 전환 뒤에도
미주 조각 소속과 후속 간격 계약이 보존되는 소비 위치를 추가로 확인한다.
현재 원장 연결이나 분할 방출 구현은 아직 시작하지 않았다.

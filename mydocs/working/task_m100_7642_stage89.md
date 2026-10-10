# Stage 89 — 미주 흐름 표의 행 프레임과 후속 줄 원점

## 범위와 독립 근거

변경 전 source는 `b4d47e2554f3de6d2c5f024e7b29fecc0e2f53e9`다.
Stage 88의 변수 대조군 개선과 별개로 네 원본의 단 이동 차이를 현재 브랜치에서
보정한다. 전체 원본·회귀 범위를 축소하거나 실패 검사를 제거하지 않는다.

첫 경계는 `samples/3-11월_실전_통합_2024-구분선위0미주사이20구분선아래2.hwp`
13쪽의 미주19 표와 문20 제목이다. 기준은 원본과 대응하는 독립 한컴 2024
Print PDF `pdf/issue7642-equation-print/equation-19-3-11월_실전_통합_2024-구분선위0미주사이20구분선아래2-hwp-print-2024.pdf`다.
PDF의 실제 drawing path를 읽으면 표 행 경계 간격이 원본 셀 선언과 일치한다.

| 독립 입력·관측 | 결과 |
| --- | --- |
| 원본 HWP와 자매 HWPX의 세 행 | 각 행 1182HU, 합 3546HU |
| 개체 크기의 저장 높이 | 3771HU |
| 독립 Print 표 전체 높이 | 35.48096pt, 약 3548.1HU |
| Native 높이 측정의 18개 셀 | 선언·요구 15.8, 내용 12.0, 상하 안 여백 3.8; 실제 성장 없음 |
| 표 format의 예약 행 높이 | 각 행 16.76, 합 50.28 |
| 실제 paint의 행 높이 | 15.76 / 15.76 / 18.76 |
| 현재 Native 일반 Sweep 13쪽 | 65.98698% |
| 현재 직접 Skia 13쪽 | 66.80147% |

좌표 관측은 원인 진단이며 정식 회귀의 고정 px 기대값이 아니다. HWP의 속성을
읽기 위한 `all20-rhwp-parsed-diagnostic.hwpx`는 RHWP의 임시 직렬화 산출물이다.
한컴 저장본·기준 출력으로 사용하지 않으며 원본을 대체하지 않는다.

## 생산 결과와 실제 소비 경로

1. `height_measurer.rs::measure_table_impl`은 실제 내용과 셀 행 선언에서 높이를
   구한다. 진단 `RHWP_DIAG_ROWH`에서 미주19의 모든 셀이 선언 안에 들어간다.
2. `notes/endnotes/flow_table.rs`가 이 측정을 미주 원장에 넣고 일반 표 format과
   block 경로로 방출한다.
3. `typeset/table.rs::fit_measured_for_host`는 빈 TopAndBottom 호스트도 개체 선언
   높이로 비례 조정하여 행별 저장 선언을 다시 키운다.
4. `layout/table_layout.rs::fit_row_heights_to_common_height`는 실제 paint에서
   개체 선언의 잔여를 마지막 행에 더한다. 같은 helper 결과를 소비한 것이 아니다.
5. `layout.rs`의 개체 원점 선택과 뒤 글줄의 `HeightCursor::vpos_adjust`가
   바깥 위·아래 여백, 저장 후속 줄 원점, 표의 실제 끝을 각각 소비한다.
   행 높이만 바꿔 뒤의 저장 원점 선택을 그대로 두어서는 해결을 입증할 수 없다.
6. `notes/endnotes/measure.rs`가 최종 layout 경로를 scratch로 재생하고,
   `paragraph/reflow_fit.rs::query_reflowed_endnote_new_head_fit`가 앞 미주의
   점유 끝과 미주 간격으로 문20 제목 수용 여부를 판정한다.

현재 Print는 문20 제목을 앞 단 하단에 남기며 본문은 다음 단에서 시작한다.
이전 Native는 제목까지 다음 단으로 넘긴다. 제목에 추가 허용치를 주거나
원점을 clamp하여 숨기지 않고 표·후속 줄이 소비하는 공통 프레임을 고친다.

## 구현·검증 계획

- 원본 저장 행이 실제 내용을 모두 담고 있고 개체 선언만 더 큰 경우의 행
  프레임을 공통 결과로 보존한다. 측정과 format·paint에서 다른 비율 조정을
  하지 않는다. 보호 크기·실제 행 성장·분할 프레임·저장 줄이 없는 재조판은
  자기 계약을 유지하며 판별 근거를 코드와 대조한다.
- 같은 프레임의 바깥 여백·실제 표 원점·점유 끝과 후속 줄의 시작을 fit과
  최종 배치가 함께 소비하게 한다. 다단 미주에서도 현재 단 원점으로 표현하며
  저장 좌표나 앞 단의 임시 원점을 다른 단에 이식하지 않는다.
- 네 실패 원본을 같은 Print PDF로 재검증하고 제목의 단 소속·후속 문단과
  표·그림의 누락·겹침·이월을 직접 판독한다. 90% 미만인 다른 선택 쪽의
  원인도 수용·예약·배치 경로에서 확인한다.
- 현재 변수·NOT·그리스·기존 미주 경계를 정상 대조군으로 유지한다.
  실제 행 성장 및 개체 높이가 필요한 표도 대조해 행 선언 보존을 일반화한다.
- 관련 모든 쪽의 Native/fresh WASM 90% 이상과 확대 판독을 확인한 뒤
  관계형 회귀를 추가·변경한다. 전체 원본 55개/853쪽과 기존 91파일,
  #1139 전체 함수의 범위를 유지한다. 전체 게이트·PR은 아직 완료하지 않았다.

## 증적 위치와 현재 상태

ignored `output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/stage88-original-skia/`에
원본·직렬화 진단·측정 로그·PDF 경계·현재 PNG를 보존한다.

- `all20-row-ledger-receipt.json`: 고정 source의 진단 명령·PID·최종 exit 0.
- `all20-row-ledger.log`: 실제 18개 셀의 선언/내용/여백/요구 분해.
- `all20-independent-table-boundaries.json`: 독립 PDF의 표 괘선 좌표·획.
- `all20-raster-table-boundaries.json`: 동일 영역 Native/Print의 긴 괘선 관측.
- `receipt.json`: 두 원본의 실제 Skia 명령·해시·점수.
- `all-20-p013-current-review.png`, `all-24-p013-current-review.png`: 변경 전 직접 판독.

현재는 원인 경로를 분석한 상태이며 구현·수정 후 검증은 미완료다.
Stage 88의 고정 source fresh WASM이 실행 중이므로 그 빌드의 파일 해시를
보존하며 Rust 수정은 해당 빌드의 종료를 확인한 뒤 적용한다.

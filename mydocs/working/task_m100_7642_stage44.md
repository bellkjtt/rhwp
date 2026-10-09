---
kind: task
status: active
canonical: mydocs/working/task_m100_7642_stage44.md
last_verified: 2026-10-09
---

# #7642 Stage 44 — 미주 경계의 실제 점유 측정과 이월 순서 보정

Issue: #7642

## 기준과 독립 관측

- 작업 head `4e932a86e7da5266440e141fdf9fe36f4e35a0fc`, base
  `79abd9e49675f4286eaed38197f7e837afb744e7`이다. rebase 전후 전체
  tracked tree는 동일하다. 이전 SHA의 실행 결과를 새 head 결과로 표기하지 않는다.
- 요청 52개와 #1139 추가 3개 입력을 유지한다. 이전 소스의 쪽수 대조는
  50개 일치 / 5개 불일치이며 최신 시각 완료를 뜻하지 않는다.
- 독립 한컴 Print에서 구분선 없는 2024년 11월 문서의 미주 8 제목은
  10쪽 오른쪽 단 하단에 있다. 기존 Native는 11쪽 왼쪽 단으로 넘긴다.
  앞 점유 끝과 미주 사이, 제목 높이의 합은 해당 단에 들어간다.
- 미주 사이 9/8/7 HWPX의 첫 차이는 미주 5 제목이다. 앞 단의 표·그림 때문에
  점유 query가 `None`이 되어 전체 묶음의 저장 높이로 이월한다.
  원본·독립 Print·행별 대응·실행 로그는 ignored
  `output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/`
  아래 `batch-endnote-boundary-analysis/`, `batch-flow-break-debug/`에 있다.

## 원인과 소비 경로

1. `measure.rs::measure_endnote_column_with_item`은 실제
   `LayoutEngine::measure_endnote_column_bottom`을 실행해 흐름 예약 끝과
   실제 점유 끝을 함께 반환한다. 현재 단과 후보는 실제 전역 소유자,
   float 배치, `measured_tables`를 사용한다.
2. `paragraph/reflow_fit.rs`의 일부 query는 실제 후보만 측정하면서도
   `future = Some(empty)`와 빈 표 측정값을 넘긴다. 이는 미래 문단의
   미지원 검사를 현재 단에도 적용하여 이미 배치한 표·그림을 거절한다.
   미래 입력 없는 query는 `None`과 실제 표 측정값을 전달한다.
3. `paragraph.rs`의 `advance_for_fit` 실행은 새 제목의 실제 fit보다 먼저다.
   실제 후보의 점유와 필요한 미주 간격을 확인한 뒤 이월한다. 명시적 단/쪽
   나눔과 `keep_with_next`는 면제하지 않는다. 실제 이동 여부를 후속 높이
   계산에도 전달한다.
4. 간격 하한은 실제 paint의 `layout.rs`와 같은
   `endnote_spacing::origin_after_gap` 결과를 소비한다. 흐름 예약에 이미
   포함된 간격을 다시 더하거나 좌표 clamp·마지막 빈 쪽 삭제로 숨기지 않는다.

## 묶음 수정과 검증

- 같은 잘못된 측정 문맥을 쓰는 전체 문단·단일 줄·새 제목·제목 뒤 첫 줄
  query를 함께 보정한다. 파일명·문항 번호·고정 좌표로 분기하지 않는다.
- Native를 한 번 빌드하여 관련 다섯 불일치 입력과 정상 대조군을 먼저
  확인한다. 필요하면 새 관측에서 남은 원인을 다시 분석한다.
- 새 head에서 요청 전체 Native/fresh WASM TSV, 전체 쪽수와 대표 review를
  생성하고 수식·간격을 확대 판독한다. 한 쪽이라도 90% 미만이면 실제
  출력을 더 보정한다. 관계형 회귀 전환은 해당 원본의 선행 조건 이후다.
- 전체 회귀·필수 lint·manifest/unit policy는 고정 base와 공용
  `target/pr-review`에서 순차 실행한다. 기존 실패를 완화하거나 제거하여
  통과시키지 않는다. 13개 기존 whitespace 오류는 내용 변경 없이 함께 정리한다.
- 계획 고정 뒤 구현한다. 로그·TSV·JSON·nextrun은 ignored output에 둔다.
  원본 증거와 새 실행 결과, 미검증 범위를 구분하여 결과를 기록한다.

현재는 분석과 보정 계획 단계이며 구현·시각 게이트·전체 회귀·PR 통합은 미완료다.

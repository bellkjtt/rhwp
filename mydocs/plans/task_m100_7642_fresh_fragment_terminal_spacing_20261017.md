# #7642 재조판 표 조각의 마지막 줄간격 예약 보정

Issue: #7642. 기준 source는 `21cdd179bfe8dc842af4022791c7bc5b2e7c2e4a`다.
실행 중인 고정 source 빌드와 전쪽 검증은 보존하고 다음 배치 후보를 준비한다.

## 독립 근거와 범위

원본 `samples/issue6776/78494-virtual-convergence-industry-decree.hwpx`와
독립 한컴 Print `pdf/78494-virtual-convergence-industry-decree-2020.pdf`를
동일한 실제 TrueType 프로필로 비교했다. 원본은 실제 HWP5이며, 본문 문단229의
비문자취급 2행2열 RowBreak 표는 저장 LineSeg가 없는 재조판 입력이다.

24쪽의 줄 내용과 원점은 PDF와 일치하지만, 마지막 `하는 신청서에 다음 각 호의
서류를` 줄이 다음 쪽으로 넘어간다. 독립 PDF의 줄 상자는 약1002.3~1021.0px로
본문 끝1046.9px 안에 있다. 다음 줄 원점까지의 전진은 약42.93px이며 줄 상자는
약18.67px다. 전체 전진과 셀 여백을 예약해 마지막 줄을 거부하면25쪽은 한 줄,
26쪽은 두 줄,27쪽은 세 줄씩 내용 소속이 밀린다. 이 수치는 진단 관측이며 구현
분기나 기대 좌표 상수로 사용하지 않는다.

## 원인·소비 경로

생산: `cell_units_uncached`가 실제 합성 줄의 높이와 뒤 간격을 CellUnit 전진으로
보존한다. 이 값은 다음 줄 위치 계산에 필요하므로 유닛 자체를 줄이지 않는다.
컷 선택: `advance_row_cut_inner`는 용량 초과 시 공통 마지막 간격 helper로 현재
조각의 물리 점유를 확인한다. 기존 `reflow_cut_trailing_spacing`은 같은 문단의
중간 컷에서는 간격을 빼지 않아, 문단 끝 컷과 서로 다른 예약 계약이었다.
측정: `row_cut_content_height`와 `row_block_content_height`도 같은 helper를
소비한다. 실제 부분 표는 같은 RowCut의 줄 소속과 물리 행 상자를 사용한다.

표 조각에 없는 다음 줄까지의 간격은 현재 조각의 마지막 줄 상자 점유가 아니다.
문단 중간 컷에서도 합성 결과의 마지막 줄간격만 제외한다. 문단 뒤 여백은 실제
문단이 끝난 컷에서만 제외한다. 중첩 투영, 개체, 문자취급 표, 유효 저장 LineSeg는
기존 범위대로 배제한다. 빈 행의 선언 최소 높이와 다음 내용은 보존한다.

## 검증

현재 고정 source의 로그·RenderTree·독립 PDF로 수정 전 조기 이월을 확인했다.
다음 source 배치에서24~27쪽의 줄 소속, 원점, 외곽·본문 점유와 전체74쪽을
Native/fresh WASM으로 검증한다. 저장 줄, 문단 끝 컷, 중첩 표, 문자취급 표 및
내용보다 큰 선언 최소 높이를 대조한다. 다른102문서 검증과 devel 대비 같은
조건의 페이지별 하락 판정도 유지한다. 전체90%와 직접 판독 전에는 rendering
회귀·baseline·쪽수 기대값을 추가하거나 변경하지 않는다.14쪽의 첫 행 분할은
고아 줄·예산을 별도로 추적하며 이번 원인으로 해결됐다고 가정하지 않는다.

## 같은 배치에 포함하는 첫 행 측정 원장 일치

수정 전 진단 실행2458에서 문단134는 첫 행의 실제 높이44.1px·줄 수2인데,
`FIRSTROW_DEFER`의 `splittable=false, clean_defer=true`로 판정됐다. 뒤에서 실제
컷에 사용한 `cell_units`가 아니라 MeasuredCell의 줄 수가 첫 이월을 결정한다.
원인은 상세 원장 생성 경로만 NO_LS Native HWP5의 셀 너비 재합성을 건너뛰는
조건이다. 행 높이 경로와 실제 `compose_cell_unit_paragraph`는 같은 입력을
이미 공통 `recompose_cell_lines_with_nested_tables`로 합성한다.

상세 원장도 저장 줄이 없고 가로쓰기인 HWP5에 같은 합성을 적용한다. 유효 저장
줄과 HWP3 보존 정책은 기존 배제 조건을 유지한다. `is_row_splittable`나 고아
줄 최소치에 문서별 예외를 붙이지 않는다.14쪽으로 온전히 이월됐던 표가13쪽에서
실제 첫 글줄 컷을 수용하는지는 다음 Native/fresh WASM 결과로 검증한다.

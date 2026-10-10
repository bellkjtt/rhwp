# Stage 94 — 수식의 명시적 빈 행과 첫 행 기준선 보존

Issue: #7642. 변경 전 source는 `0e178d78ad6b3cd861a699272cd96fe3b5419ff8`이다.
55개 원본 853쪽, 복원한 91파일 292검사와 #1139 전체 관계형 전환 범위를 유지한다.
Stage93의 집중 Skia 14개 242쪽은 90% 이상이지만 수학 1쪽 4번의 확대 판독에서
구간별 함수의 행 간격·중괄호 높이 차이가 남았다. 점수로 이 차이를 면제하지 않는다.

## 독립 근거

원본 `samples/exam_math.hwp`의 수식에 있는 중첩 `EQALIGN{...#}`를 그대로 추출했다.
그 뒤 마지막 `#`만 제거한 대조군, 선두·중간·끝 빈 행, 일반 두 행, 빈 묶음,
CASES의 명시적 빈 행을 11pt와 22pt로 각각 작성했다. 크기·기준선·저장 줄을
지우고 한컴 저장 및 Print로 정상화했다. 인증 정보·글꼴 바이트는 기록하지 않는다.

ignored 증적의 공통 경로는
`output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/`이다.
v2는 `stage93-eqalign-empty-row-probe-v2/`, v3는 같은 이름의 `-v3/`에 있다.
변환 전후 각각 12개·16개 수식 보존을 확인했고 한컴 저장본을 다시 읽어 개수·스크립트·
BaseUnit·크기·기준선을 기록했다. MCP 실제 result는 engine 2020,
Hancom 11.0.0.9136, method 0, `hancom2020_pdf_driver_one_up`이다.

| 대조 | 11pt 관측 | 22pt 관측 |
| --- | --- | --- |
| 원본 중첩 수식의 끝 빈 행 유무에 따른 높이 증가 | 1100HU | 2200HU |
| CASES 안 단일 EQALIGN의 끝 빈 행에 따른 높이 증가 | 1100HU | 2200HU |
| 독립 EQALIGN의 한 빈 행 추가 | 1210HU | 2420HU |
| EQALIGN 한 행/끝 빈 행의 기준선 비율 | 85% / 41% | 86% / 41% |

독립 EQALIGN은 첫 행 기준선을 유지한다. 전체 높이의 절반을 기준선으로 쓰는
현재 구현은 빈 행을 추가하면 첫 글자 원점까지 움직인다. 독립 묶음의 행 피치는
글자 크기의 1.1배이고 CASES 안에서는 바깥 CASES가 줄 간격을 소유하므로
중첩 빈 행이 한 글자 크기만큼 공간을 늘린다. 두 적용 경로를 구분한다.
새로운 문서별 좌표 예외를 넣지 않는다. 원본 시험지의 Print 기준은 바꾸지 않는다.

첫 진단 v1은 빈 자기닫힘 설명 태그를 RHWP 파서가 하위 텍스트 읽기에 보내
수식 12개 중 11개를 소비했다. 그 결과는 진단 근거에서 제외했다.
동일 문제를 정상 입력에서도 예방하도록 자기닫힘 설명·스크립트는 종료 태그를
기다리지 않게 처리한다. 빈 설명과 내용 있는 설명을 구분하며 다음 수식을 보존한다.

## 생산 결과와 실제 소비 경로

`parse_cases/parse_eqalign`은 마지막 `#` 뒤 빈 행을 AST의 실제 행으로 보존한다.
CASES는 행 점유와 행 사이 leading을 소유하고 중첩 EQALIGN은 그 내부의 빈 행을
소유한다. EQALIGN은 첫 행 기준선·각 행 점유·좌우 정렬 원점을 함께 계산한다.
빈 행은 글리프를 표시하지 않아도 물리 공간을 점유한다. 오른쪽 칸이 없을 때
정렬 gap을 전체 폭에 추가하지 않는다.

`layout_node → layout_cases_rows / layout_eqalign → LayoutBox`의 같은 결과를
SVG `render_box`, Rust Canvas `render_box`, Skia `equation_conv::render_box`가 소비한다.
각 backend가 행 소속이나 행 높이를 다시 추측하지 않는다. 원본 저장 객체 높이는
그대로 유지하고 내부 수식 행 원점·중괄호 점유를 직접 비교한다.

## 검증과 남은 범위

한 번의 보정 뒤 포맷·Native/WASM Clippy·Skia lib check·Native 빌드를 순차 실행한다.
수학 1쪽·17쪽과 두 수학 원본 전체, 독립 28개 대조 수식을 우선 직접 비교한다.
끝 빈 행 없는 원본과 첫·중간 빈 행의 대조군도 확인해 일반 행의 회귀를 찾는다.
전체 55개 Native/fresh WASM 검증과 확대 판독의 선행 조건 충족 뒤 정식 관계형 회귀를
작성한다. 현재 정식 회귀 추가·변경은 없으며 이 문서는 구현 전 계획이다.


## 구현 후보

조건식·정렬 파싱을 137줄의 `parser_alignment.rs`로 분리했다. CASES의 마지막
명시적 빈 행과 EQALIGN의 마지막 빈 행·빈 묶음을 보존한다. `#`가 없는 정상 식의
행 소속과 명령 구분은 유지한다. 기존 parser 본체는 원래 1000줄을 넘는 파일이며
이번에는 관련 행 파서만 분리하고 무관한 파서 전체 재구성은 하지 않았다.

`EqLayout`은 CASES의 간격 소유 여부를 자식 배치로 전달한다. HYhwpEQ 수학 face의
독립 EQALIGN은 0.1em leading, CASES 안은 바깥 leading을 소비한다.
행 높이는 기준선 위·아래 점유를 각각 합치며 빈 행도 최소 한 글자 크기를 점유한다.
HYhwpEQ EQALIGN의 묶음 기준선은 첫 행에 연결한다. 오른쪽 칸이 없으면 phantom
정렬 gap을 폭에 넣지 않으며, 완전히 빈 수식은 독립 Print의 반각 자리 폭을 예약한다.
일반 face의 정렬 묶음 기준선·leading 정책은 기존 값을 사용한다.

자기닫힘 `script`는 뒤 텍스트를 스크립트로 삼지 않고, 자기닫힘 `shapeComment`는
하위 텍스트 읽기에 진입하지 않는다. 내용 있는 설명은 기존대로 읽는다.
아직 이 후보의 lint·빌드·시각 결과는 미검증이다. 정식 회귀 추가·변경은 없다.


## 독립 대조와 lint 완료

v2의 HWP 저장 job은 `3c8d98a1-2eb0-4270-8456-887b875d1468`, Print job은
`c9b16008-e3eb-4a3c-8341-f91032822458`이다. v3의 저장 job은
`c306701e-571f-4d7e-9701-26d5a8f84462`, Print job은
`d72b6f6e-8757-4a82-afb0-8debb30fccf3`이다. 28개 모두 입력의 스크립트·BaseUnit과
한컴 저장본의 값이 대응한다. 독립 관측과 행 공간의 관계를
`stage94-independent-empty-row-contracts.json`에 연결했다.

검증 source는 `d666c0a2e7298f5d62f6ca5e33d7623937ed1c2e`이다.
`stage94-empty-row-type-checks.json`의 포맷 22.52초, Native Clippy 94.65초,
WASM Clippy 68.38초, Skia lib check 42.36초는 각각 exit 0이다.
Native 빌드는 `empty-row-native-provenance.json`의 고정 source로 실행 중이다.
완료 후 독립 28개 진단과 수학 두 원본의 전체 쪽 Skia 직접 비교를 순서대로 실행한다.
`stage94-validation-controller.json`에 실제 단계·PID·exit code를 기록한다.
현재 빌드 성공·시각 통과·정식 관계형 회귀 완료를 선언하지 않는다.

Stage93 전체 Native/fresh WASM 작업은 고정 이전 바이너리·패키지로 계속 실행한다.
새 수식 source의 검증으로 재사용하지 않으며 변경 전 대조 자료로 구분한다.
최종 source의 필수 전체 TSV·확대 판독·정식 회귀·전체 lint와 nextest는 남아 있다.


## 분수 반례로 간격 조건 재검토

추가한 11pt/22pt 분수 대조 8개도 변환 전후 수식 개수·스크립트·BaseUnit을 확인했다.
`stage94-eqalign-owner-counterexamples/hancom-normalized-equation-properties.json`에서
분자의 끝 빈 행이 있으면 분수 높이가 각각 1100HU/2200HU 증가한다.
같은 분수를 CASES에 넣어도 같은 증가량이다. 그러므로 위 첫 후보의 CASES 전용
소유 조건은 너무 좁다. 부모가 없는 독립 정렬 묶음에만 0.1em leading을 예약하고
분수·조건식 등 실제 부모가 있으면 그 부모가 바깥 간격을 소유하게 바로잡는다.
글꼴·색상·단일 행 포장은 내용 소속을 바꾸지 않는다. 독립 EQALIGN의 자식으로
다시 중첩한 정렬 묶음에도 자체 leading을 중복 전달하지 않는다.

첫 후보 Native 빌드는 소유 Cargo/rustc의 PID와 start ticks를 확인해 종료했다.
193.24초, exit -15이며 성공 binary나 시각 결과가 없다.
`stage94-invalid-owner-candidate-build-stop.json`에 종료 이유와 소유 근거를 보존했다.
관측 시간 만료로 재시작한 것이 아니며 새 독립 반례가 잘못된 조건을 드러냈다.
첫 후보의 lint 통과는 교정한 source의 lint 근거가 아니므로 새 경로에 재실행한다.


분수 반례의 HWP 저장 job은 `becb1177-0156-4f1f-85a1-8a5e66d64210`, Print job은
`0c2ca41e-bb7c-49ef-981d-06a7b67284b0`이다. 실제 Print method 0과 제품 빌드를
확인했으며 PDF SHA-256은
`e4de9cc3e3372c85cc47ab5aee91e006e0089218643f3da2b73e594e59ed1a02`다.

교정한 code source는 `5654d84514780e7b07f15b9d7c1bb3bb099c03d0`다.
`layout → is_standalone_eqalign → layout_eqalign`에서 실제 부모 소속을 판단한다.
글꼴·색상과 단일 행 포장만 통과시키고 분수·조건식 등 소유 구조는 독립 묶음으로
취급하지 않는다. 각 정렬 행의 자식 배치에는 독립 상태를 다시 전달하지 않는다.
정식 회귀 기대값은 여전히 바꾸지 않았다. 새 source 검증은
`stage94-empty-row-v2-type-checks.json`, `empty-row-v2-native-provenance.json`,
`stage94-v2-validation-controller.json`에 별도로 보존한다.

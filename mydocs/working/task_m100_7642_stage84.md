# #7642 Stage 84 — CanvasKit 수식의 표시 face와 자연 전진

Issue: #7642

## 독립 기준과 원인

Stage 83의 실제 CanvasKit software surface는 NOT 사선 20개와
공개 중괄호 윤곽을 정상 replay했지만 300dpi Print 대조는
88.24930%다. 같은 소스의 Native SVG는 94.57865%다. 화면의
렌더 오류 0건을 시각 일치의 증거로 사용하지 않는다.

Native SVG는 수학 알파벳을 Latin Modern Math에서 표시하고,
문자·숫자는 그 글꼴의 자연 전진으로 그리며 논리 상자의 폭은
다음 개체의 원점 예약에 사용한다. CanvasKit은 CJK 기본 face를
먼저 선택하고 수학 fallback은 Noto Sans Math이며, 논리 예약 폭에
맞춰 글리프를 다시 수평 확대한다. 실제 소비 계약이 서로 다르다.

`renderEquationBox → drawEquationText → face 선택 → setScaleX →
drawText`를 SVG Text/Number/Symbol의 실제 호출과 대조했다.
두 경로의 표시 face와 자연 글리프 전진을 일치시킨다. 가운데
기호는 논리 상자의 가운데와 자연 표시 폭으로 정렬한다.
논리 상자의 측정·다음 원점·NOT 끝점은 변경하지 않는다.

## 구현 범위

CanvasKit의 기본 수학 fallback 공급을 동봉 Latin Modern Math로
정하고 수학 glyph replay에서 먼저 선택한다. 사용자가 지정한
math fallback URL은 그대로 소비한다. 수학 face에 없는 CJK 등은
기존 본문·기호 face 순서로 공급한다. 표시 함수는 별도 1000줄
미만 모듈로 옮긴다. 기존 대형 renderer는 공급 경로와 호출부만
바꾼다. 공개 글꼴 자산은 기존 것을 재사용한다.

## 검증과 남은 범위

같은 fresh WASM·입력·Print와 실제 CanvasKit software surface의
수정 전후를 대조하고 수식 영역을 확대 판독한다. tsc와 관련
기존 Studio 계약·글꼴 검사를 실행한다. 새 정식 회귀는 시각
선행 조건을 충족한 뒤 추가한다. Rust source tree가 같으면
Stage 83의 Native/fresh WASM 빌드 해시와 현재 frontend SHA를
구분해 기록한다. 전체 원본 55개·853쪽 TSV 및 기존 px 검사의
관계형 전환 범위는 유지한다.

`W`의 Print는 같은 HYhwpEQ 글리프에도 text matrix의 shear
0.333을 사용한다. 네 크기의 Print 연산에서 확인했으며 Linux
설치본과 PDF subset의 윤곽 해시도 같다. 이 독립 변환과 글꼴
공급 문제는 아직 해결하지 않았다. 해당 차이는 별도 잔여로
기록하고 이번 CanvasKit 소비 계약 보정으로 완료 판정하지 않는다.

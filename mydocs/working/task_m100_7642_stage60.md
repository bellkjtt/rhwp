# 이슈 #7642 Stage 60 — 인용 기호의 공통 글리프와 자동 간격

Issue: #7642

## 독립 Print에서 남은 차이

Stage 59 code head `992a275829741001740ef0deb2c6e192d7fbc555`는 기본 인용
변수의 글꼴 전환을 보정했으나 혼합 문자열의 수학 기호·자동 간격은 빠뜨렸다.
99.13060%의 높은 점수에도 직접 확대 판독에서 x+y·x-y·x=y가 Print보다 좁다.
한컴 Print의 텍스트 원점과 실제 Native RenderTree/LayoutBox를 같이 확인했다.

- 독립 PDF의 기호는 HYhwpEQ의 수학 PUA다. +는 U+E048, −는 U+E046,
  =는 U+E047이고 괄호도 수학 PUA다. 인용은 명령·첨자 해석을 막지만
  수학 글리프·자동 여백까지 정체 ASCII 글자 폭으로 바꾸지 않는다.
- 실제 layout_literal_text는 기호를 layout_number로 보내며 -를 하이픈으로
  표시하고 기호 양쪽의 자동 간격을 생산하지 않는다. 빈칸도 숫자 face의
  일반 space 전진폭으로 게시한다.
- 실제 Native의 9pt 입력 x+y·x-y·x=y 선언 폭은 각각
  25.36/25.36/27.04이고 자연 폭은 22.336/16.996/22.336이다.
  원본 저장 폭과 독립 PDF 위치가 공통 symbol_spacing의 글리프·양쪽 여백과
  일치한다. 이는 관측값이며 절대 px 회귀 기대값으로 추가하지 않는다.
- 인용 x y 선언 폭은 14.493333이고 자연 폭은 16.984다. 문자 x/y의 공통
  전진과 한컴 BaseUnit/8 Thin 공간을 합하면 선언·Print와 대응한다.

원본 PDF SHA는 Stage 59의 인쇄 증거와 같다. PDF word 원점은
`stage59-quoted-mode-probe/print-text-bbox.html`, 실제 완성 Native library의
모델·RenderTree·LayoutBox는 `stage60-current-quoted-layouts.log`에서 읽었다.
사용한 helper 원문과 컴파일 로그도 같은 ignored output에 보존했다.

## 원인 소비 경로와 수정 준비

Quoted AST 원문 → literal_text의 문자별 glyph 종류 → 공통 layout_row/
symbol_spacing → LayoutKind·전진폭·기준선 → SVG·Canvas의 같은 children
소비 경로로 연결한다. 문자열 자체를 parser에 다시 넣지 않는다.
예약 명령 sin/alpha·밑줄/지수 문자를 함수·명령·첨자로 바꾸지 않으며,
표시할 수학 연산자·구분자와 공백만 기존 측정·자동 간격 결과를 소비한다.
절대 문단 위치·원본 이름·고정 폭 보정은 넣지 않는다.

수정안은 `stage60-literal-symbol-spacing.patch`와 source hash JSON에 준비했고
`git apply --check`를 통과했다. 아직 source에는 적용하지 않았다.
Stage 59 fresh WASM이 실제로 종료되고 해시를 확인할 때까지 source를 유지한다.

인용/비인용 +/−/=, 빈칸/명시적 Thin, 괄호, 점, 시작·끝 연산자,
연속 연산자, 관계 기호와 나눗셈 22개를 같은 정상 section·font·BaseUnit으로
추가 생성했다. `stage60-quoted-spacing-probe/input-plan.json`에 생성·원문
해시를 남겼고 Save job `adfe55c8-605c-4dee-89d7-1e6d6c09ae19`를 모니터링한다.
이 원격 요청은 진행 중이며 성공·Print 일치 증거로 미리 사용하지 않는다.

Native/fresh WASM·직접 확대 판독과 전체 55개·853쪽의 원래 검증 범위,
관련 회귀의 관계형 전환·전체 Rust 및 필수 lint 범위는 계속 유지한다.
렌더 회귀·baseline 기대값은 아직 변경하지 않는다.

# #7751 KoPub 주의표 폭의 누락 — 4단계

2026-10-16, 기준 HEAD `121c95daaebdc1955ab4be547f937b566c5c2e50`.
전383쪽 Native 재산출은 최저92.24708%, 누락0, 이전 보정 출력과 전쪽 값이 같다.
다만 devel 대비0.01pp 이상 하락은74·206·209·297쪽에 남아 있으므로 최종 제출을
완료로 판정하지 않는다. 이전과 같은90% 통과만으로 하락 재검토를 면제하지 않는다.

## 독립 근거와 원인

- 독립 한컴2024 KoPub Print74쪽의 `※` 원점은146.24px, 다음 공백은157.60px다.
  실제 전진폭은11.36px다. 뒤 `｢`·`｣`는 각각6.24px로 반각이 맞다.
- 같은 쪽 read-only font trace는 `※`를 `heuristicHalfwidth`로 고르고, 원점 이후
  전진폭472HU=6.2933px를 쓴다. 약5.07px 차이는 이 한 글자 직후부터 이어진다.
  반각 따옴표 뒤의 다른 한글 전진은 독립 Print와 같은11.04px다.
- 설치된 KoPub Dotum/Batang Light의 cmap+hmtx를 직접 읽으면 U+203B의 전진폭은
  모두900/1000em이다. 두 face에 U+FF62/FF63/FF65는 없으므로 이 글리프의 반각
  fallback 수정은 유지한다. 글꼴 파일을 복사·재배포하지 않는다.
- 원인 계층은 KoPub의 내장 글리프 폭 선택이다. 표 배치·좌표 clamp나 특정 문서
  분기로 처리하지 않는다. `cmap/hmtx → KoPub 폭 선택 → char_width_decision의
  장평·자간 → 줄 측정과 run 배치 → 최종 원점`을 같은 결과로 연결한다.

## 구현·검증 계획

1. 다른 굵기의 같은 glyph 폭도 확인한 뒤 KoPub의 주의표 폭을 원본 메트릭으로
   공급한다. 폰트별 원본 폭의 자료이며 문서 ID나 속성 조합의 시각 예외가 아니다.
2. 영향74·206·209·297쪽 및 정상3·241·289·290쪽을 Native로 먼저 비교한다.
   하락쪽을 포함해 확대 판독하고, 역효과가 있으면 전체 회귀 전에 원인을 재검토한다.
3. 최종 소스에서 fresh WASM과 전쪽 양쪽 TSV·기준 비교를 갱신하고, 중요한 경계의
   회귀·Native Skia·전체 Rust·필수 lint를 완료한다. 이전 head의 결과는 구분한다.

원시 증적은 `output/pr-review/pr7494-20261016/p74-font-trace.json`,
`p74-character-origin-comparison.json`에 있다.3단계 커밋의 집중33건은 통과했으며
필수 fmt·Native/WASM Clippy·workspace build도 완료했다.4단계 코드 변경 뒤에는
그 결과로 새 head의 완료를 주장하지 않는다.

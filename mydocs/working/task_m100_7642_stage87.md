# #7642 Stage 87 — 대문자 변수의 표시 face와 기울임 계약

Issue: #7642

## 독립 근거

크기별 NOT의 W와 9월 2024 HWPX Print 스트림을 직접 읽었다.
기본 이탤릭 대문자 A/B/C/D/E/F/R/S/W/X/Y/Z의 수학 PUA는 모두
`Tm=[1,0,0.333,-1,...]`로 표시된다. 소문자 수학 PUA는 같은 스트림에서
shear가 0이다. bfchar와 bfrange를 모두 해석했으며 미해석 CID는 없다.
결과는 ignored output의 `stage86-print-variable-transform-audit.json`에
PDF 해시·페이지·문자·실제 변환과 함께 보존했다.

현재 표시 투영은 대문자도 Latin Modern Math의 Unicode 수학 이탤릭
글리프를 직접 선택한다. 이것은 원본의 정체 글리프에 적용한 기울임과
글리프 설계·잉크 범위가 다르다. 공개 정체 W의 윤곽을 원본의 em 잉크
프레임과 독립 Print shear에 배치한 SVG 진단은 모양이 Print에 가까워졌다.
진단은 production 출력의 통과가 아니며, W 하나의 코드 예외로 구현하지 않는다.

## 후보와 검증 범위

- 한컴 수학 face의 보통 굵기 이탤릭 ASCII 대문자 전체에 동일한 표시 계약을
  적용하는 후보를 검토한다. 정체·굵은 변수·소문자·다른 face는 같은 근거가
  없는 방식으로 바꾸지 않는다.
- 공개 Latin Modern Math의 A~Z 정체 윤곽만 라이선스와 출처를 보존해 사용한다.
  설치 HYhwpEQ에서는 em 잉크 프레임의 메트릭만 읽고 사설 바이트·윤곽을
  코드·자산에 복제하지 않는다. 공개 파생 asset의 이름과 manifest를 구분한다.
- 논리 전진·기준선·AST와 줄 소속을 유지한다. 실제 표시 윤곽은 공통 FontGlyph
  결과로 발행해 SVG·Canvas·Skia·paint JSON·CanvasKit이 소비하게 한다.
  NOT는 변수 역할을 보존하며 기울임을 두 번 적용하지 않는다.
- 6/9/12/18pt의 A~Z와 소문자·정체·굵은 변수 대조군을 한컴 2024 MCP에서
  정상 저장하고 저장본을 Print 출력한다. 수동 변경 입력의 오래된 선언 상자와
  정상 저장본의 재생성 상자를 구분한다. 정상 저장본이 독립 기준의 입력이다.
- 대조군·기존 NOT·실물 영향 쪽의 Native/fresh WASM Visual Sweep과 확대 판독
  뒤 후보를 판정한다. 진단 PNG의 높은 점수를 실제 구현 통과로 승격하지 않는다.
  90% 미만인 입력의 기대값·baseline·정식 회귀는 변경하지 않는다.

## 현재 상태

공통 원인의 독립 스트림 근거와 공개 윤곽의 W 진단을 확보했다.
전체 대문자의 독립 대조군·후보 코드·Native/fresh WASM 실행은 아직 미완료다.
Stage 86의 미주 경계 source 검증도 고정 바이너리와 package에서 별도로 계속한다.

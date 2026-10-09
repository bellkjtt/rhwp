# 이슈 #7642 Stage 61 — 축약 이항 기호·Unicode·slash의 일관된 배치

Issue: #7642

## 한 묶음으로 검토할 남은 원인

앞 인용 간격 보정 f6f51dd5d의 22개 비교 입력은 98.63868%다.
직접 판독에서 비인용 x+-y가 두 기호로 남고 Print는 ±로 출력된다.
인용 x+-y는 Print와 같이 두 기호를 보존해야 하므로 문자열 전체를 다시
tokenizer/parser에 넣는 보정은 적용하지 않는다.

1. tokenizer::try_read_multi_char_symbol은 기존 <=/>=/!= 등의 경계만 처리하고
   +-와 -+의 수식 축약을 인식하지 않는다. 인용 read_quoted 이후 경로에서
   비인용 축약을 실제 명령 표와 같은 의미로 해석해야 한다.
2. 그 명령·실제 Unicode·인용 안의 표시 기호가 같은 glyph/전진/양쪽 간격을
   소비하는지 함께 확인한다. 현재 ±/∓는 operator 메트릭·row 분류에 빠져
   있으며 일반 비ASCII Text로 합쳐질 가능성도 있다. 관계 기호와 이항 기호의
   의미는 구분하되 실제 glyph는 같은 측정·배치를 소비하게 한다.
3. 인용/비인용 slash는 실제 Print의 같은 PUA E054이며 저장 폭·높이도 같다.
   비인용 기본 Symbol 여백이 붙어 자연 폭 20.2로 증가하는 경로를 보정한다.
   인용의 19.0은 저장/Print와 대응한다. 이것은 진단 관측이며 절대 px
   기대값이나 특정 문서 폭 조건으로 추가하지 않는다.

출처는 `stage60-current-quoted-paired-layouts.log`와
`stage60-quoted-spacing-probe/print-text-bbox.html`다. 설치 HYhwpEQ의 실제
cmap·hmtx도 확인했고 ± 831, ∓ 1024, 수학 slash E054 512(UPM 1024)다.
Print ±가 실제 U+00B1로 기록된 것도 확인했다.
`stage61-installed-operator-metrics.json`에 글꼴 SHA·codepoint·advance만 남겼고
글꼴 바이트·윤곽을 공개하지 않는다. 관측 9pt 한 쪽만으로 다른 크기의
일치를 미리 판정하지 않는다.

## 독립 한컴 대조와 적용 순서

정상 section·문단·font를 보존한 48개 입력으로 6/9/12/18pt의 축약 +-/−+,
명령 plusminus/minusplus, Unicode ±/∓, 인용 기호, slash를 함께 생성했다.
생성·입력 해시와 사례는 `stage61-binary-operator-probe/input-plan.json`에 있다.
한컴 2024 Save/Print의 같은 job을 모니터링하며 완료 뒤 실제 제품·인쇄
경로·PDF 해시·전체 쪽수와 각 크기의 원점을 확인한다. 아직 출력 완료
증거로 사용하지 않는다.

Source는 현재 f6f51dd5d로 유지한다. Native 빌드·fmt·Native Clippy는 통과했고
fresh WASM은 실제 빌드 중이다. 이 빌드 종료·패키지/Studio 해시 검증을
확인한 뒤 다음 수정 묶음을 적용한다. 다음 수정 전 3개 진단 입력의 선언/
자연 프레임 차이와 명령·glyph 분류를 함께 대조해 관련 누락을 모은다.

전체 55개·853쪽 Native/fresh WASM, 직접 확대 판독, 기존 관련 회귀의
관계형 전환과 전체 Rust/필수 lint는 원래 범위로 유지한다. 새 기대값과
렌더 baseline은 90% 및 직접 판독 선행 조건 전에는 변경하지 않는다.


## 네 크기의 실제 Print 완료

48개 입력의 Print가 2쪽으로 완료됐다. job은
`3b72b543-e788-476d-99cd-f4ffbf4883a8`, Hancom 13.0.0.3901,
PrintMethod 0, frame_print_to_pdf_ex_one_up이다.
PDF SHA-256은 `e9db1659fd6a3840381cd3af5d875cb3fc40739067fe8ef75505c3c227047da0`다.
`stage61-binary-operator-probe-mcp-print.json`의 실제 결과와
`stage61-binary-operator-probe/print-detail-provenance.json`에 페이지·원본
해시·배율·좌표를 연결했다. `print-binary-operator-four-size-detail.png`를
직접 열어 표시했다.

네 크기에서 비인용 축약 +-는 ±, -+는 ∓이며 해당 Unicode와 대응한다.
인용한 ASCII 두 기호는 각각 보존하고, 인용 Unicode ±/∓ 자체의 glyph와
자동 간격은 같은 표시 계약을 따른다. slash는 인용/비인용 모두 같은 한 줄이다.

다만 소문자 minusplus는 기호가 아니라 min 함수와 나머지 문자로 출력된다.
현재 symbol table의 MINUSPLUS와 대소문자 재조회가 이를 기호로 바꾸는 가정도
독립 Print와 대조해야 한다. 새 alias를 무작정 추가하지 않고 기존 등록의
적합성과 LaTeX 확장/한컴 원문 계약을 구분해 다음 묶음에서 확인한다.
plusminus는 이번 실제 Print에서 ±로 확인됐다. 미검증 별칭을 같은 것으로
추정해 코드 변경의 기대값에 넣지 않는다.

현재 source f6f51dd5d의 Native 빌드·fmt·Native Clippy는 통과했지만 전체
55개·853쪽과 fresh WASM은 아직 실행 중이다. 기존 관련 회귀의 관계형
전환도 선행 조건 뒤에 진행하며 최종 완료로 보고하지 않는다.

## 명령 대소문자와 축약 경계의 추가 Print

20개 입력의 독립 Print도 한 쪽으로 완료했다. job은
`31102d20-a7a2-453a-aed7-f2a4b52d760a`, Hancom 13.0.0.3901,
PrintMethod 0, frame_print_to_pdf_ex_one_up이며 PDF SHA-256은
`61225a30eb585990266b9cf64262df092882cf9c8de85e7585e383530b6e34fe`다.
`stage61-operator-alias-probe/print-alias-review.png`를 직접 열어 다음
구별을 확인했다.

| 입력 | 독립 Print의 해석 |
| --- | --- |
| plusminus / PLUSMINUS | ± |
| MINUSPLUS | ∓ |
| minusplus | 정체 min 함수와 이탤릭 usplus |
| pm / PM / mp / MP | 일반 문자 |
| 붙은 ASCII +- / -+ | ± / ∓ |
| ordinary space로 분리한 + - / - + | 두 기호 보존 |
| ASCII +와 Unicode −를 붙인 +− / −+ | 두 기호 보존 |
| 직접 Unicode ± / ∓ | 수학 기호 |
| 인용 ASCII +- / -+ | 두 기호 보존 |
| 인용 Unicode ± / ∓ | 수학 기호 표시 |

현재의 PM/MP alias와 MINUSPLUS의 무조건 대소문자 재조회는 이 계약과
다르다. LaTeX 확장인 백슬래시 pm/mp는 별도의 명령 읽기 경로에서 보존하고
한컴의 일반 문자열을 그 확장으로 해석하지 않는다. min 접두사는 실제
함수 의미로 분리하며 인용 문자열과 완전한 명령의 경계는 보존한다.
이 표는 실제 입력의 의미 근거이며 Native/fresh WASM 시각 통과를
선언하거나 절대 px 회귀 기대값을 추가하는 근거로 대신하지 않는다.

## 구현과 의미 진단

비인용 tokenizer는 서로 붙은 ASCII +-/-+만 ±/∓로 변환한다. 인용과
ordinary space 및 Unicode minus 혼용은 별도 경계로 유지한다. Unicode
산술 기호는 문자 덩어리에 흡수하지 않고 parser의 수학 기호로 전달한다.
일반 pm/PM/mp/MP는 별칭에서 제거했고 백슬래시 pm/mp는 LaTeX 읽기
경로에서 기호로 전달한다. MINUSPLUS의 exact case와 min 함수 접두사도
독립 Print 계약을 따른다. 기존 lookup_symbol(pm) 단위 기대값 한 건은
이 독립 의미 근거에 따라 None으로 정정했다. 렌더 baseline은 변경하지 않았다.

배치의 소비 경로는 tokenizer → parser Symbol/MathSymbol →
layout_node → layout_symbol/layout_math_symbol → source_font_symbol의
논리 전진 → row_children의 인접 여백 → layout_row의 최종 glyph 원점이다.
인용도 명령을 재해석하지 않고 같은 Symbol 행으로 전달한다. ±/∓에는
각각 UPM 1024 전진 831/1024와 이항 여백을 사용한다. slash는 같은
전진 512를 소비하고 이항 여백을 예약하지 않는다. 문서 ID나 고정 px
좌표로 분기하지 않는다.

원본 모듈을 직접 include한 의미 진단은 수정 전 8건 실패, 수정 후
0건 실패다. 축약·별칭·min 함수·LaTeX 확장·공백·Unicode minus·인용
보존을 한 묶음으로 확인했다. 로그는 stage61-source-ast-before.log 및
stage61-source-ast-after.log에 보존한다. 진단 결과는 실제 Native/fresh
WASM의 시각 통과나 정식 렌더 회귀 검사의 대체가 아니다.

이전 source f6f51dd5d의 fresh WASM은 exit 0, 1243.09초로 완료했다.
패키지와 Studio의 rhwp.js/rhwp_bg.wasm SHA-256은 각각 일치하며,
WASM은 9fad268accac7dd7570612cfc23cbee0db8a3134d9e5c2370634b1d1ba8be065다.
수정 후 코드는 별도로 Native 빌드·기호 진단 전체 페이지·대표 원본을
확인한 뒤 fresh WASM과 전체 55개·853쪽 검증으로 이어간다.

source-side 정책 검사는 고정 base
79abd9e49675f4286eaed38197f7e837afb744e7과 대조해 exit 0이며
4198 tests / 298 modules를 유지했다. LaTeX pm/mp는 정규 명령 토큰으로
전달해 기존 command의 첨자 처리도 유지한다. 아직 전체 Rust나 시각
검증을 통과했다고 판정하지 않는다.

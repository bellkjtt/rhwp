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

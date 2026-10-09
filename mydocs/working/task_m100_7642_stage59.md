# 이슈 #7642 Stage 59 — 붙은 그리스 기호 명령과 인용 글꼴 계약

Issue: #7642

## 독립 한컴 Print의 문법 대조

원래 11월 HWPX 18쪽에 `sin pix`가 남아 있다. RHWP는 pi를 문자 p·i로 표시하고
독립 Print는 π로 표시한다. 원문을 수정하지 않고 명령 경계를 확인하기 위해
16개 합성 진단 입력을 한컴 2024에서 HWP로 저장한 뒤 Print PDF로 출력했다.
이 입력은 원래 55개 검증 문서나 그 기준 PDF를 대체하지 않는다.

Hancom 13.0.0.3901, engine 2024, pdf_print_method=0,
pdf_output_mode=frame_print_to_pdf_ex_one_up을 실제 결과에서 확인했다.
HWP SHA-256은 `7e849f598f7f34f861800032fb36743ffb57ceb45eee78bb699a0add1a8bf720`,
PDF SHA-256은 `d518195f25f4567480b45493023dec1e7f788ba511131837d3ffc16d8392a21c`다.
Print는 1쪽이다. 생성 방식·template/입력 해시·사례 목록은 ignored
`stage58-greek-prefix-probe/input-plan.json`, MCP 제품·인쇄 모드·job·출력 해시는
`stage58-greek-prefix-probe-mcp-save.json`·`-print.json`에 남긴다.
처음 queued 상태가 이어졌지만 같은 job을 모니터링했고 새 job을 중복 제출하지 않았다.

| 입력 | 직접 확인한 한컴 Print |
| --- | --- |
| sin pix / sin pi x | 같은 sin πx |
| pix | πx |
| pixel | πxel |
| alphabet | αbet |
| alphaB | αB |
| sin alphax / sin alpha x | 같은 sin αx |
| rm alphabet / rm pix | 그리스 명령은 유지하며 뒤 라틴 글자는 정체 |
| sin "pix" / "alphabet" | 문자 그대로 보존하며 현재 기본 이탤릭 유지 |
| bar{RmOM_1} / bar{rmOM_1} | 같은 정체 OM₁와 윗줄 |

한컴 PDF의 같은 영역을 확대해 직접 표시했다.
`stage58-greek-prefix-probe/print-greek-command-contract-detail.png`와
`print-detail-provenance.json`에 원본 PDF·동일 좌표·배율을 남겼다.
저장 HWP의 script도 별도로 읽었으며 pix·alphabet·인용 문자열의 철자는
저장 뒤 그대로 남는다. Script 읽기에 사용한 기존 helper의 AST는 옛 파서 결과이므로
현재 source의 AST 증거로 사용하지 않고 `stage59-saved-probe-source-ast-old-parser.log`의
script 항목만 대조했다.

## 90% 통과 뒤에도 남은 의미·글꼴 차이

source de11af0c7의 Native Visual Sweep은 이 진단 입력 1쪽에서
92.56245%이고 자동 gate는 passed다. 엄격 내용 픽셀은 10.12773%다.
그러나 새 review를 직접 보면 붙은 pix·alphabet 등이 문자로 남으며
따옴표 문자열은 Print의 기본 이탤릭과 다르게 정체다. 자동 점수 통과를
이 문법·글꼴 결함의 완료 판정으로 사용하지 않는다. Native before 증거는
`stage59-greek-prefix-before-native/greek-prefix-before` 아래에 보존했다.

같은 입력의 quoted 구간은 토큰/AST에서 문자 해석을 막는 계약이다.
layout의 EqNode::Quoted가 layout_number로 전달돼 현재 글꼴 전환을 무시한다.
숫자·문자 여부와 글꼴 전환을 혼동하지 않고 literal 텍스트의 같은 측정·paint
결과를 소비해야 한다. 인용 구간의 명령은 계속 재해석하지 않는다.

## 수정 전 실제 파서와 잘못된 검사

현재 Native library의 tokenizer/AST 진단에서 9개 붙은 그리스 명령이
공백으로 구분한 Print 계약과 달라 exit 1이다. pi 전체 명령·argmax·rootn·
따옴표 문자열 대조군은 유지됐다. 로그·원문 해시는
`stage59-greek-prefix-contract-before.json`·log·probe 원문에 보존했다.

기존 tokenizer 검사의 두 `alphabet → [alphabet]` 기대값은 한컴 Print의
αbet와 다르다. 입력의 문자 손실을 막는다는 주장을 하면서 예약 명령의
의미를 지운 사례다. 이것은 절대 좌표나 렌더 baseline 갱신이 아닌 실제
명령 경계의 독립 계약 보정이다. pi·alpha 전체 명령, 전체 argmax/rootn과
인용 문자열의 정확한 기존 계약은 유지한다.

## 준비한 수정 묶음과 다음 검증

기호표가 그리스 문자 하나로 해석하는 실제 명령만 붙은 피연산자와 분리한다.
인용 문자열과 이미 인식한 전체 명령은 분리하지 않는다. 특정 문서 ID나
sin 뒤라는 우연한 맥락, 임의 단어 길이를 조건으로 사용하지 않는다.
그리스 이름의 대소문자는 현재 기호표의 계약을 유지하고 닫는 RIGHT 경계도
같은 기호표 판정을 사용한다.

Quoted는 명령 재해석을 막으면서 현재 정체·이탤릭·굵기 전환을 이어받도록
literal text 배치 경로로 연결한다. 숫자와 한글·기호·혼합 문자열도 같은
측정/출력 경로로 확인하며 새로운 고정 폭·좌표를 넣지 않는다.

수정안은 `stage59-greek-prefix-correction.patch`에 준비했고 현재 source 해시와
`git apply --check`를 확인했다. 아직 source에는 적용하지 않았다.
실행 중인 de11af0c7의 fresh WASM owner 종료·해시를 확인한 뒤 적용한다.
수정 전후 AST, 이 Print 입력의 Native/fresh WASM·확대 판독, 원래 sin pix가
있는 문서의 영향 쪽과 정상 대조군을 같은 수정 묶음에서 확인한다.
새 배치 회귀·fixture·baseline은 전체 관련 출력 90% 이상과 직접 판독을
충족한 뒤 관계형으로 추가·변경한다. 전체 55개·853쪽과 기존 검사 전환의
원래 범위는 계속 유지한다.


## 적용과 혼합 문자열의 독립 대조

앞 소스 de11af0c7의 fresh WASM은 exit 0, 1274.62초로 종료했다.
패키지·Studio public의 JS/WASM SHA-256 일치를 확인하고 수정안을 적용했다.
앞 소스의 집중 fresh WASM은 고정한 바이너리·패키지로 별도 실행한다.
새 소스와 혼동하지 않으며 빌드 중 source를 변경하지 않았다.

현재 tokenizer/parser/ast 원본 모듈을 직접 include한 의미 진단에서 앞서 실패한
9개 계약은 모두 통과하여 exit 0이다. `stage59-current-source-ast-after.json`·log에
실제 source hash를 남겼다. 이 의미 검사로 출력 검증을 대체하지 않는다.

추가 24개 혼합 문자열도 같은 정상 section과 수식 문단에서 생성한 뒤 한컴
2024 Save/Print로 확인했다. Print job은 `27bbf59c-d23f-49f6-880a-c26029fa5780`,
Hancom 13.0.0.3901, PrintMethod 0, frame_print_to_pdf_ex_one_up이다.
PDF SHA-256은 `4f159634c2e1a458ba16161b9633827ddd44953c121b91b4c43cb3c33e4143d3`다.
`stage59-quoted-mode-probe/input-plan.json`·`print-detail-provenance.json`과
`stage59-quoted-mode-probe-mcp-print.json`에 생성·출력 증거를 보존했다.
새 Print 확대 이미지를 직접 열어 표시했다.

- 인용 x2와 비인용 x 2는 같은 이탤릭 x·정체 숫자다. rm/it/bold 전환도
  문자와 숫자에 각각 적용되며, 인용을 이유로 전체 정체로 바뀌지 않는다.
- 가x2·A1가B2에서 한글은 정체 본문 face, 라틴 문자는 이탤릭, 숫자는 정체다.
- x+y·x-y·x=y·x_2·(x)는 문자 그대로 보존한다. 명령·첨자로 다시 해석하지 않는다.
- sin은 인용하면 일반 변수 글꼴을 유지하고 rm에서만 정체다.

`layout_literal_text`는 인용 문자열의 문자 순서를 유지한 채 공통 문자·숫자
측정 결과를 나열한다. 기존 layout_text 전체 문자열 판정의 혼합 CJK 억제를
인용 구간에 적용하지 않는다. 라틴·그리스와 CJK는 해당 기존 글리프 경로,
숫자·문장 부호는 정체 literal 경로로 전달한다. 측정된 전진폭·기준선과 선택한
LayoutKind를 SVG·Canvas가 그대로 소비하며 연산자 여백을 다시 넣지 않는다.
절대 문단 위치·문서 ID·글자 수 조건은 추가하지 않는다.

현재 출력 보정 묶음은 적용했고 Native 빌드·시각 검증은 다음 실행으로 확인한다.
렌더 회귀 기대값이나 baseline은 변경하지 않았다. 두 alphabet 검사는 독립
Print와 다른 순수 명령 해석 기대값만 보정했다. 전체 Native/fresh WASM·확대
판독과 전체 원본의 선행 조건을 충족하기 전 관계형 배치 회귀 추가는 보류한다.

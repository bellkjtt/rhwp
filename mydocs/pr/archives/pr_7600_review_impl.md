---
kind: plan
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-06
---

# PR #7600 메인터너 보정 실행 기록

## 범위와 독립 근거

사용자가 메인터너 보정을 승인했습니다. 별도 과거 79.8%/64.3% 자료는 없다고 확인했습니다. 공개 #7593 원본은 확보했고 MCP engine2020의 Print PDF(job `c0af74c9-1bcf-454d-b177-08ab91b24e59`, Hancom 11.0.0.9136, pdf_print_method 0, hancom2020_pdf_driver_one_up)와 새 Native 전체 1쪽에서 79.58463%를 재현했습니다. 이 원본을 삭제/축소하지 않고 같은 독립 Print 기준으로 보정합니다. 미제공 문서와 같은 파일이라고 추정하지 않습니다.

## 단계

1. 원 기여 체리픽 `5d7a2ad98`, 원시 TSV local output 분리 `08abab871`와 초기 리뷰를 보존합니다.
2. 공개 원본/Print PDF를 커밋하고 마스터 쪽 colPr 파싱 → MasterPage.paragraphs의 ColumnDef → build_master_page의 실제 paint 소비 경로를 고칩니다. 본문 단 구분선의 기존 column 영역 계산·선 스타일 결과를 재사용하고 바탕쪽 프레임 전체 높이를 적용합니다. 문서 전용 좌표·문서 ID·임계값 예외는 추가하지 않습니다.
3. 공개 원본을 먼저 재출력해 90%와 내용/경계 직접 판독을 확인합니다. 새 회귀는 독립 Print·관계 기반이고 그 선행 조건을 통과한 뒤에만 추가합니다. 기존 원 기여 테스트는 삭제하지 않습니다.
4. PR 최소 입력은 MCP가 기본 빈 OnDocument_New 스크립트 때문에 무인 변환을 거절했습니다. 매크로 실행·보안 설정 변경은 하지 않습니다. 기본 빈 템플릿을 정적으로 확인해 스크립트 없는 별도 검증 사본을 만들 경우 변경 entry를 공개하고 모든 조판/본문 XML의 동일성 및 Native 출력 동일성을 입증합니다. 원본은 보존합니다.
5. Native/fresh WASM과 바탕쪽 관련 대조군·정식 회귀 red/green, fmt/native/WASM32/workspace Clippy·base 정책·필요 전체 회귀를 순차 검증합니다. 로그/TSV/중간 JSON은 output에만 둡니다. source 변경 후 이전 캡처를 완료 근거로 쓰지 않습니다.
6. 결과를 개별 review와 오늘 할 일에 확정하고 원 PR 기여/보정 source와 실제 검증 head를 구분합니다. 원격 PR 생성·push·merge는 사용자의 해당 단계 지시에 따릅니다.

## 적용 계약과 반례

마스터 쪽 단 구분선은 배경이지만 보이는 개체이며 본문 흐름을 전진시키지 않습니다. masterPage의 텍스트 프레임과 그 안 colPr 정의에 따라 위치/높이를 계산하고 별도 본문 단 설정으로 바꾸지 않습니다. 1단/구분선 없음에는 선을 추가하지 않으며 가변 너비·여백·여러 단의 위치도 기존 column 계산 계약을 재사용합니다. 본문 쪽수·다음 내용·번호와 다른 바탕쪽 개체 보존을 확인합니다. 분할/이어받기 규칙 변경은 없습니다.

## 2026-10-06 추가 원인 추적

바탕쪽 선 복원 뒤 공개 원본 전체 1쪽은 86.42433%로 아직 미달입니다. 굵은 정식 face를 추가 공급해도 점수는 동일했습니다. 보기 ㄴ의 수식은 저장 높이 2580 HWPUNIT, baseLine 85이며 MCP Print에서 앞뒤 글자보다 위에 표시됩니다. `equation::flow_metrics_hwp → composer/line_breaking.rs의 flow_inline_controls 및 줄 기준선 → paragraph_layout.rs의 두 인라인 EquationNode 생성`을 추적하면 측정은 저장 기준선 거리를 쓰고 배치는 intrinsic 기준선으로 다시 계산한 뒤 줄 위를 clamp합니다. 같은 flow 메트릭을 실제 원점도 소비하도록 고치며 셀/본문, 텍스트 혼재/수식 전용 경로와 최소 입력을 함께 재확인합니다. SVG/Canvas2D/Skia는 생성된 EquationNode 원점을 그대로 소비하며 압축 프레임의 공통 세로 비율은 유지합니다. 독립 Print 확인 없는 합성 저장값을 근거로 새 예외를 만들지 않습니다.

두 파일 비교는 (1) 원 최소 입력과 스크립트 제거 사본의 최신 Native 전체 페이지 raster, (2) 기존 기여자 Hancom 2022 PDF와 새 MCP 2020 Print PDF, (3) 각 HWPX와 대응 MCP Print PDF를 구분해 같은 96 DPI로 전체 페이지를 비교합니다. 원본의 조판 entry는 보존하고 TSV/로그는 output에만 남깁니다.

선택지의 원본 HWPX 문단에는 저장 LineSeg가 없고 각 LEFT 탭 폭 4000은 고정 전진 폭으로 남아 있습니다. MCP Print의 ②/③은 문단 기본 탭 격자에 놓이지만 Native는 앞 수식 폭을 포함한 커서에 저장 전진 폭을 다시 더해 점점 오른쪽으로 밀렸습니다. `compose_paragraph_scoped → ComposedParagraph.tab_extended → paragraph_layout의 런/개체 커서 → text_measurement의 저장 전진/TabDef 분기`를 추적했습니다. 저장 줄 부재/합성 줄의 수용 여부는 기존 `para_has_no_stored_line_segs` 계약을 그대로 쓰고, 이 경우 저장 advance를 전달하지 않아 현재 문단 TabDef/기본 격자에서 계산하도록 보정합니다. 원본의 tab/type/leader 메타데이터는 model에 보존하며 실제 저장 LineSeg가 있는 경로는 기존 advance를 사용합니다. 선택지/수식의 순서·본문과 셀·저장 줄 대조군을 최종 검증 범위에 포함합니다.

실제 SVG data-URI를 fontTools로 읽어 파일 공급의 오판도 확인했습니다. 선언된 한컴산뜻돋움은 실제 Noto Sans KR ExtraLight, 함초롬바탕은 Haansoft Batang이었습니다. 단순 fc-match 결과를 실제 SVG 공급으로 간주했던 초기 기록을 정정합니다. 시스템은 정식 HanSantteutDotumRegular/HCRBatang을 보유하지만 SVG 파일 탐색이 시스템 하위 폴더와 영문/해시 접두 파일명을 해석하지 못했습니다. 명시적 공급 우선순위 뒤에 fontdb의 family/weight 조회를 추가해 실제 등록 face를 파일명 폴백 전에 찾습니다. 이름/굵기 일치 여부와 최종 data-URI face를 다시 확인하고, 일반 시스템 등록만으로 실행하여 별도 RHWP_FONT_PATH나 사후 SVG 교체를 쓰지 않습니다.

## Native 선행 확인 및 입력 보존

source 보정의 선행 Native 전체 1쪽은 공개 문항 92.28714%, 최소 입력 96.47259%로 통과했습니다. 시스템 등록 글꼴만 사용한 full embed의 실제 name table은 Han Santteut Dotum Regular/Bold와 HCR Batang으로 확인했습니다. 단 구분선·보기 사각 틀·그림 경계가 대응하고 보기 ㄴ의 수식이 앞뒤 본문보다 위에 놓이며 ②/③이 기본 탭 격자로 복귀했습니다. 수식 글리프의 장평/획과 발문 간격의 잔여 차이는 남아 있고, 보조값 통과를 전체 glyph fidelity로 확대하지 않습니다. 최종 source의 fresh WASM·회귀·lint·전체 검증은 이어서 실행합니다.

원 최소 입력/스크립트 제거 사본의 Native 전체 페이지 raster는 픽셀 동일(실루엣 100%), 기존 기여자 Hancom 2022 PDF/MCP 2020 Print PDF도 픽셀 동일(실루엣 100%)입니다. paired PNG를 직접 열어 4행·화살표·번호 보존을 확인했습니다. 이는 원본 매크로 실행 또는 기존 PDF의 Print 절차 확인을 뜻하지 않습니다. 새 Print의 실제 MCP 실행 근거와 스크립트 없는 사본의 동일성으로 기준 출력 대응을 입증합니다. 자료는 각각 output의 min-original-vs-safe, min-old-vs-print-pdf와 compare 패널에 남겼습니다.

MCP 산출 PDF는 원본 형식·엔진·상대 디렉터리의 canonical 짝짓기에 맞게 `pdf/issue7593/repro-equation-issue7593-hwpx-2020.pdf`, `pdf/issue7593/repro-dyad-min-no-script-hwpx-2020.pdf`로 보존하며 출력 바이트를 바꾸지 않았습니다. 원문은 samples/issue7593 아래 두 파일이며 원 기여자의 최소 입력/PDF도 기존 경로에 보존합니다. 최종 검증은 이 안정 경로에서 다시 실행합니다.

## 정식 회귀와 Studio의 직접 누락 확인

Native/fresh WASM에서 두 대응 Print의 최저 92.28714%를 먼저 확인한 뒤
`tests/cases/issue_7593_print_placement.rs`에 구분선의 바탕쪽 소유/프레임 관계,
보기 ㄴ 수식의 저장 기준선/셀 소유, 선택지의 기본 탭 격자/순서/단 내부 포함을 추가했습니다.
원래 source `a0db9c052`의 실제 실행은 3 FAIL이며 각각 선 0개, Print의 기준선 순서 위반,
탭 격자 비율 3.5375(독립 Print는 3)로 실패했습니다. 빌드 실패를 red로 세지 않았습니다.
초기 테스트 작성 중 TextRun의 셀 문단 인덱스와 Equation의 최외곽 표 문단 인덱스를
혼동한 assertion은 CellContext의 실제 소유 관계로 수정했습니다. 최초 harness 준비 뒤
format으로 suite 배정이 바뀌어 0 tests가 실행된 결과도 통과로 세지 않았습니다.

실제 Studio CanvasKit 캡처를 직접 열자 공개 원본의 ①/② 수식이 빠진 것을 확인했습니다.
WASM render tree의 7 Equation 존재만 확인한 초기 browser 결과는 표시 성공 근거가 될 수 없으므로 정정합니다.
기본 Noto Sans KR에 U+2AFD glyph가 없으면 drawEquationText가 false를 반환하고,
PictureRecorder가 정상 AB/CD·dyad 장식까지 포함한 수식 전체를 버렸습니다.
기본 CJK → 기존 D2Coding → 실제 glyph를 가진 Noto Sans Math 순서로 font를 고르도록
보정했고 문자를 다른 codepoint로 바꾸거나 실패한 수식을 숨기지 않았습니다.
시스템 설치본의 전체 font를 WOFF2로 변환하고 SIL OFL 1.1을 함께 보존했습니다.
원 TTF SHA-256 `8242bd1e55368b27e32455260754cf9aa58f3ad7ea80664b66c21f1b09910d6c`,
WOFF2 `6836027b99d29fd0e6f668cf0c2e89107de4131b91116cd96516ad4a7e19d66d`이며 subset/문자 매핑 변경은 없습니다.
정확한 glyph 의미/한컴의 평행 기호 모양은 #7594의 잔여 축이고 그 이슈를 종료하지 않습니다.

보정 전 실제 browser는 ①/②의 내용 0과 `equation:invalidLayout`을 검출했습니다.
보정 후 실제 제품 loadDocument/font 준비/paint 경로의 Canvas2D·CanvasKit × 두 파일은 4 PASS,
모든 수식에 내용이 있고 CanvasKit unexpectedUnsupportedOps가 0이며 숨은 Canvas2D overlay는 없습니다.
`rhwp-studio/e2e/issue-7593-dyad-display.test.mjs`가 이 실제 경로를 보존하고,
기존 font coverage 검사도 Noto Sans Math의 U+2AFD/2225/2194 공급을 확인했습니다.
실제 내려받은 WASM SHA는 `040ffe26e4de9a064e350c6264452e3b767c4808f798661907ec42b29ba22a34`입니다.
Rust/lib source는 1a7ad89e4와 같고 TS 보정은 이 WASM을 재사용하는 paint 경로의 변경입니다.

신규 두 sample은 전체 1쪽 Print/PDF/Native/fresh WASM 대조를 확인한 뒤 oracle 쪽수 원장에 1/1로
등록합니다. regenerate.py의 canonical 선택을 두 입력에 적용해 형식·상대 디렉터리·저장 제품별
engine2020 짝이 실제 위 PDF로 결정됨을 확인했습니다. 기존 원장의 다른 행·허용치는 바꾸지 않습니다.
모든 최종 필수 lint·전체 Rust/Skia 검증의 완료 여부는 다음 단계 결과에서 확정합니다.

보정 복원 후 동일 정식 배치 검사 3 PASS / 0 FAIL (exit 0)을 확인했습니다.
로그 `output/pr-review/pr7600/logs/placement-red.log`, `placement-green.log`를 보존했습니다.

## 발문 수식의 저장 프레임과 실제 paint 폭 분리

사용자가 발문 첫 수식이 Print와 너무 다르다고 지적했습니다. 원본 `rm dyadAB`의
common.width 3000 HWPUNIT은 흐름 예약 폭 40 (96 DPI), 동일 BaseUnit 1100의 intrinsic AB 장식 폭은
20.02이며 SVG의 최종 transform이 약 1.998배 확대했습니다. 독립 PDF의 실제 AB·화살표 glyph는
약 20의 폭이고 뒤 본문은 예약 폭을 지나 시작합니다. 같은 보기의 `rm dyad AB`는 저장 폭 1500이므로
기존 강제 확대가 거의 없어 같은 글자가 문맥에 따라 2배 다른 장평이 됐습니다.

생산 `Equation.font_size/BaseUnit → EqLayout::for_equation의 공통 glyph/장식 layout`과
흐름 측정 `flow_metrics_hwp → tac_w/줄 커서 → EquationNode.bbox`를 구분합니다.
SVG/web_canvas/Skia/Studio CanvasKit은 현재 bbox.width/layout.width로 paint만 다시 늘립니다.
최종 paint에서 이 가로 확대를 제거하고 BaseUnit과 공통 intrinsic layout을 그대로 소비합니다.
예약 bbox와 뒤 텍스트의 위치·내용 소유는 보존하고, 기존 압축 높이의 공통 세로 비율은 유지합니다.
문서 ID·dyad 전용 폭·특정 비율 조건·clamp는 추가하지 않습니다.

먼저 동일 두 문서를 Native로 다시 대조하고 발문의 실제 glyph/화살표·뒤 글자 및 선택지를 직접
확인합니다. Native/fresh WASM 최저 90% 이후 동일 글자의 paint 장평/본문 예약 관계를 정식
회귀에 추가합니다. 수정 전 red와 수정 후 green, normal 최소 입력·숫자 대조와 필수 lint·전체
회귀·Skia를 새 source에서 검증합니다. 이전 source의 시각/CI 결과로 완료를 선언하지 않습니다.

Native 선행 재출력은 공개 원본 92.29683%, 최소 입력 96.24413%, 모두 gate passed / 전체 1쪽입니다.
발문 glyph/화살표 폭의 이중 확대가 없어졌고 흐름 예약 폭과 뒤 글자 위치는 보존됐습니다.
전후·독립 Print의 확대 패널 `output/pr-review/pr7600/equation-first-line-before-after-print.png`를
직접 열어 확인했습니다. 전체 보조값의 변화와 별개로 표시 장평의 결함이 해결됐음을 구분합니다.
CanvasKit의 Text 기본 이탤릭 처리가 명시적 Roman을 다시 덮던 경로도, 기본 상태와 명시적 상태를
구분해 rm의 정체·it의 이탤릭을 상속하도록 보정합니다. 수식 글자 자체의 backend별 폰트와 작은
획/원점 차이는 여전히 남아 있으며 이 선행 결과는 전체 검증 완료 문구로 쓰지 않습니다.

## 선택지의 내부 관계 기호 간격

새 출력에서 ④/⑤의 문단 탭 원점을 독립 Print와 같은 좌표계로 확인했습니다.
사용자가 지적한 ①/②의 수식 안 간격은 별도로 남았습니다. 일반 공백은 tokenizer가 명령 경계로
제거하며, layout_row는 형제를 간격 0으로 붙입니다. BOT는 MathSymbol이지만 옆 여백이 없고
⊥의 advance를 0.5em으로 추정해 실제 수학 글꼴의 advance(배포 Latin Modern Math 0.778em,
시스템 DejaVu Math 0.788em)보다 작습니다. 다음 dyad가 관계 기호와 붙거나 겹칩니다.
직접 U+2AFD는 Text로 취급되어 관계 기호의 정체·간격 계약도 소비하지 못합니다.

공통 관계 기호 분류를 parser/layout에 적용하고 기존 연산자 여백 계약 0.15em을 관계 기호의
양쪽에도 소비하게 합니다. glyph/간격을 명시적인 공통 Row 결과로 만들어 SVG/Canvas2D/Skia/
CanvasKit이 같은 원점을 소비합니다. ⊥/⊤의 추정 advance는 해당 실제 수학 글꼴 폭에 맞는
0.8em 계열로 분리하고 °/‰ 등의 기존 좁은 폭은 보존합니다. 이 값은 원본 페이지 좌표에서
역산한 px 값이 아니라 독립 글꼴 메트릭의 차원 없는 비율입니다. 다섯 선택지의 문단 탭 위치·
수식/글자 순서·누락과 최소 정상 입력을 다시 확인합니다. width 수정 전 WASM build는
새 source 검증으로 오인하지 않도록 종료했고 원시 로그만 보존했습니다.

Native 간격 보정의 재출력은 공개 원본 92.98588%, 최소 입력 99.43478%로 모두 전체 1쪽 gate를
통과했습니다. ①/② 관계 기호와 양쪽 수식의 간격, 다섯 선택지/뒤 정답의 순서·누락,
④/⑤ 탭 원점을 전체 페이지와 확대 패널에서 직접 대조했습니다. 수식/글자 획·작은 font 원점
차이는 남아 있고 완전 동일 출력이라고 쓰지 않습니다. 새 source는 `9ee2878be`입니다.

사용자 요청에 따라 `THIRD_PARTY_LICENSES.md`에 폰트의 2.001 버전·저작권·OFL 전문·출처를
등록했습니다. 실제 배포에서 전문이 빠지지 않도록 Chrome/Firefox의 OFL text/집계 문서 복사와
VS Code의 math font/라이선스/집계 문서 및 실제 webview URI를 보완했습니다.
Studio·Chrome·Firefox build, VS Code typecheck/webpack build, font/extension 배포 계약 9 PASS입니다.
새 37번째 폰트의 226,456 bytes 추가만큼 inventory는 37개 / 22,877,752 bytes로 변경했습니다.
이는 알려진 신규 자산의 재고 등록이고 renderer golden·래칫 오차 허용치 변경이 아닙니다.
최신 Rust WASM의 렌더 검증과 font 배포 자산 검사를 구분하며 위 build만으로 새 WASM의
시각 확인을 완료했다고 주장하지 않습니다.

## 9ee2878be의 전체 페이지 재검증과 paint/간격 회귀

최적화 fresh WASM build exit 0 (13분 48초), pkg/public/실제 브라우저 download의
WASM SHA-256은 `cf850c529720d37cef7953f6ef03d857662110d408e55a2decb9bd5029a43c18`이다.
Rust 런타임 source 9ee2878be와 동일한 84fda6939에서 Native/fresh WASM 각각
공개 원본 92.98588% / 최소 입력 99.43478%, 대응 Print 전체 1쪽 gate passed를 확인했다.
전체 페이지 compare·review·standalone overlay와 실제 Studio Canvas2D/CanvasKit 4개 경로를
직접 확인했다. 실제 수식 내용·backend·download SHA 및 숨은 overlay 없음은 4 PASS다.
원 최소 입력↔스크립트 제거 사본의 최신 Native 전체 페이지 raster도 100% / 픽셀 동일하다.
원시 TSV 원본 두 개를 PR 첨부용 ZIP으로 묶었고 output 안에만 보존했다.

이 독립 Print 및 Native/fresh WASM 90% 선행 조건을 충족한 뒤
`tests/cases/issue_7593_print_placement.rs`에 실제 SVG paint 장평 동일성, BOT/직접 Unicode
관계 기호의 양쪽 간격·내용 순서를 추가했다. 절대 px나 출력 해시를 기대값으로 쓰지 않는다.
수정 소스 선행 GREEN 6 PASS (`logs/paint-green-initial.log`), 수정 전 7a61272f1의
실제 RED 3 PASS / 3 FAIL (exit 100, `logs/paint-red.log`)을 확인했다. 장식 실제 폭
39.99996 대 20.02/20.02, BOT의 왼쪽 간격 부재, 직접 Unicode의 MathSymbol 0개가
각각 의도한 결함으로 실패했다. 기존 3개 기하/소유 검사는 RED 대조에서도 통과했다.
최신 소스 복원 후 같은 6개를 다시 실행하며 필수 lint·전체 Rust/Skia 게이트를 이어서 수행한다.

최신 WASM으로 Studio/Chrome/Firefox/VS Code 배포본을 다시 빌드했다. typecheck/webpack과
폰트·확장 배포 계약 9 PASS를 확인했다. Studio dist WASM도 위 cf850c52 해시와 일치한다.
Noto Sans Math의 OFL 전문·원 저작권 표기는 모든 배포본에서 원문 바이트 그대로 보존됐다.
Vite ENOSPC는 저장소 루트에서 실행한 데 따른 것이며 사용자가 Studio 디렉터리에서 정상
동작을 확인해 Vite 설정 수정은 범위에서 제외했다.

최신 source 복원 후 같은 6개 회귀도 6 PASS / 0 FAIL (exit 0)로 종료했다.
`logs/paint-green.log`의 실제 실행 count와 summary를 확인했다. 검증 후보는
`31c1d4ecd6cfd0fce50658fccce158cf876b8077`이며 9ee2878be 이후의 Rust 런타임 diff는 없다.
필수 검증은 upstream/devel을 fetch해 base a048da32d로 고정하고 순차로 실행한다.

## 전체 회귀의 실패와 사용자 승인 범위 확장

후보 31c1d4ecd의 fmt/세 Clippy/workspace build/base 정책과 원 기여 focused 3개는 통과했다.
전체 nextest는 10,517개 실행, 10,514 PASS / 3 FAIL / 50 SKIP, exit 100, 730.063초로 실제 종료했다.
Native Skia는 이 실패 뒤 실행하지 않았고 PR 생성/승인/merge로 보고하지 않는다.

- #1139 문 28의 고정 x/y 영역은 `(ii)` 소유인 pi817의 두 인라인 수식까지 선택했다.
  실제 앞 수식은 pi816이며 점유 끝 779.853, 다음 pi817 시작 789.853으로 10의 분리 공간이 있다.
  오류의 801.853은 다음 문단 자체의 수식 끝이다. 원본 문단 소유·순서 관계로 바꿀 필요가 있다.
- #1308의 HWP/HWPX 두 검사는 SVG 수식을 245..260의 고정 y 안에서 찾지 못했다.
  문단/명시적 개행의 줄 소유·내어쓰기 관계로 선택하고 실제 Print에서 확인한다.
- 사용자 추가 승인: #1139 파일 전체(86개 검사 / 3,731줄)의 px 고정 검사를 검토·보정한다.
  새로운 문서/원본의 시각 미달을 기존 차이라는 이유로 면제하지 않는다.

기존 2020 suffix PDF는 실제 Creator가 2022이고 p15의 문 28은 왼쪽 단 끝으로 재배치돼
현재 저장 문서의 오른쪽 단 시작과 다르다. 새 Native/fresh WASM 대조에서 35.87554%를 확인했다.
기존 원 PDF 2024의 p15와 Native는 84.63888%이며 이것도 90% 충족으로 보고하지 않는다.
둘의 Print 실행 출처를 Creator만으로 추정하지 않고 같은 HWP를 metadata 2022에 맞는 MCP 2020
Print로 다시 출력한다. job 15b90e11-d4b7-45f6-847a-baf3d0a67891은 Hancom 11.0.0.9136 /
PrintMethod 0 / one-up / 18쪽으로 실제 성공했다. 원본과 실패 증거를 보존한다.

Native TSV ZIP의 GitHub 첨부 API는 ZIP MIME 형식을 HTTP 422로 거절했다.
이는 자동 승인 거부가 아닌 서버 첨부 형식 제한이며 원시 파일은 output에 그대로 남아 있다.
파일 형식을 위장하거나 Git에 커밋하지 않고 PR 게시 시 정상 첨부/동일 파일 링크 경로를 확정한다.

## #1139 전체 px 고정 검사 재검토 단계

사용자 답변으로 파일 전체를 포함했다. 원본 86개 검사 중 58개가 px 고정/영역 선택 검토 후보이며
실제 입력은 19개다. 진단 inventory는 output/issue1139-audit.json에만 보존한다.

사용자가 재확인한 순서는 **원 회귀 테스트의 원본 입력 출력 보정 → 독립 한컴 Print PDF와
Native/fresh WASM의 관련 모든 페이지에서 90% 이상 확인 → 필요한 회귀 검사 변경**이다.
#1139와 #1308의 기존 검사도 이 순서를 적용하며, 미달 상태에서 px 고정 검사를 먼저 관계 검사로
바꾸지 않는다. 90% 이상을 확인한 뒤 문단·줄·개체의 소유·순서·보존·포함·겹침 관계를 검사한다.
기존 검사가 독립 기대값에 맞으면 유지하고 구현을 고친다. 원본과 실패 증거는 보존한다.

1. 각 검사의 주장·원본 문단/컨트롤 소유·쪽 범위와 실제 px predicate를 대조한다.
2. 각 원본을 저장 제품에 맞는 MCP Print로 출력해 Native/fresh WASM의 90% 선행 조건을 확인한다.
   저장 제품 2022의 기본 engine2020 p15는 새 Print도 35.9635%로 미달이다. 이전 PDF의 상태로 면제하지 않는다.
   같은 원본의 2024 Print는 버전 차이 진단 대조로 별도 산출했고 자동으로 기준을 대체하지 않는다.
3. font/배치의 실제 원인을 공통 생산·소비 경로에서 고친 뒤, 기존 검사의 pixel 영역 선택·절대 좌표를
   원본 문단/줄/개체 소유·완전성·앞뒤 순서·단 내부 포함 관계로 바꾼다. 픽셀 허용 범위 확대는 하지 않는다.
4. 변경 파일은 1,000줄 이하의 모듈 구성으로 정리하고 정식 integration 검사 count/등록을 보존한다.
5. 최종 source의 시각·focused·fmt/세 Clippy/base 정책·전체 nextest·Native Skia·fresh WASM을
   순차 완료한 뒤 오늘 할 일/개별 리뷰와 같은 통합 PR을 등록하고 CI/merge/후속 처리를 진행한다.

## 회귀 보정 순서의 지침 반영

`AGENTS.md`·`CLAUDE.md`·`CONTRIBUTING.md`, 기여자 스킬과 PR 템플릿, 시각 검증 정본,
메인터너·콜라보레이터·첫 기여자 절차와 리뷰 공통 표에 위 90% 선행 조건과 px 고정 금지를 반영했다.
PR 템플릿에는 실패한 원 검사·원본 입력·출력 보정 SHA·90% 이상 확인 증거·검사 변경 이유를
연결하는 항목을 추가했다. 기록의 띄어쓰기 수정도 함께 유지한다.

문서 검증은 `git diff --check`와 변경 Markdown 20개 상대 링크 검사에서 오류가 없었다.
기여자 스킬 계약 검사는 26개 중 24개 통과, 2개 실패였다. 수정 전 HEAD의 동일 검사도 같은
2개가 실패했으며, 기존 `fixtures/catalog.json`·`분석` 문구 요구 실패로 확인했다.
이 문서 검증 결과를 #1139·#1308의 시각 미달 해소나 전체 회귀 완료로 보고하지 않는다.

## 90% 통과 뒤 수식·간격의 확대 판독 지침 보완

사용자가 지적한 “이 부분 수식 너무 다른데?”와 “여기도 간격이 다름”을 독립적인 검토 항목으로
명시했다. 90% 통과 후에도 변경 영역·알려진 차이·사용자 지적 영역을 독립 Print PDF와 변경 전·후
Native/fresh WASM의 같은 내용·배율로 확대해 수식 모양·장평·기준선·첨자·화살표·장식,
관계 기호 양쪽·앞뒤 본문·보기 번호·탭·줄 사이 간격을 직접 대조한다.

공통 지침·기여자 지침과 스킬·역할별 검토 절차·PR 템플릿·리뷰 공통 표에 상세 판독 절차를
연결했다. 페이지 점수와 판독 결과를 구분하고, 확인된 결함은 점수 통과로 면제하거나 정상
기대값으로 고정하지 않는다. 미해결 결함·미검증은 해당 범위의 완료·회귀 추가/변경·승인·통합
보류 사유로 기록하며, 출력 보정 뒤 새 source에서 영향 영역과 관련 페이지를 다시 검증한다.

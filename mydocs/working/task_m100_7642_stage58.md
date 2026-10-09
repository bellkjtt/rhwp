# 이슈 #7642 Stage 58 — 붙은 글꼴 명령의 대소문자 혼용

Issue: #7642

## 독립 출력에서 확인한 결함

Stage 57 source `8ddb3b16963815c71da5e049f6b4ab8c2a8d111a`의 11월
구분선 위 9·미주 사이 8·구분선 아래 7 HWPX 17쪽은 Native 실루엣 97.36303%다.
본문 298의 미주 내부 13을 4배 확대하니 RHWP 첫 수식에 `Rm`이 추가로 보이고,
뒤의 분수와 본문이 겹친다. 독립 한컴 Print PDF의 같은 수식에는 이 글자가 없다.
수식의 원문은 다음과 같다.

```text
bar{rmOQ_1 }= bar{RmOM_1 }- bar{rmQ_1 M_1} = root5 over 5
```

원본·기준·최신 이미지와 명령은 Stage 57 기록 및 ignored
`stage57-current-selected-direct/all-31/metrics.json`에 연결했다.
원문을 고치거나 선언 폭을 늘려 문서를 맞추지 않는다.

## 생산·소비 경로와 원인

`tokenizer.rs::read_command`의 붙은 글꼴 명령 조회는 rm/RM·it/IT·bold/BOLD의
두 철자만 matches_at으로 분리한다. 일반 fallback의 GLUE_SAFE에는 이 글꼴
접두사가 없어 RmOM을 하나의 Command로 남긴다. parser의
parse_command_fallback은 분리된 명령의 대소문자를 이미 무시하지만 RmOM 전체는
글꼴 명령이 아니어서 Text가 된다. 이 AST의 글자가 equation layout 폭·장식·
실제 SVG에 전달된다. 수식 개체의 저장 영역은 원래 글꼴 전환을 전제로 하므로
추가 글자는 문단의 뒤 본문과 겹칠 수 있다.

21개 원본 HWPX를 읽어 따옴표 문자열 밖의 붙은 글꼴 명령을 조사했다.
9개 문서·27개 수식에 대소문자 혼용 접두사가 있다. 11월 2022 HWPX 및
7개 다른 2024 변형에도 RmP/RmC/RmOM이 반복된다. XML 조사 자료는
`stage58-mixed-font-prefix-source-census.json`에 보존했다.
이 조사는 tokenizer 실행·HWP 전체 조사나 각 수식의 Print 판정을 대신하지 않는다.

## 다음 수정 묶음과 검증

붙은 글꼴 전환 bold/it/rm은 parser와 같은 ASCII 대소문자 규칙으로 분리하고,
토큰의 원래 철자와 피연산자·첨자·장식 순서를 보존한다. 기존 times/sim의
분리 조건, 따옴표 문자열과 비명령 식별자는 이 수정으로 확대하지 않는다.
특정 문서·문단 번호나 수식 폭을 조건으로 넣지 않는다.

일반 rm/RM·공백 있는 Rm·붙은 Rm/rM·it/bold의 혼용·첨자 및 따옴표 문자열을
수정 전후 토큰/AST로 대조한다. 해당 입력들의 기존 정확한 검사는 유지한다.
이미 1000줄을 넘는 tokenizer.rs를 수정할 때는 테스트 모듈을 동일 모듈의
하위 파일로 분리해 코드 파일 크기 원칙도 지킨다. 새 배치 회귀는 전체 관련
Native/fresh WASM 90% 이상 및 확대 판독 뒤에 관계형으로 추가·변경한다.

현재 같은 source의 fresh WASM 빌드가 실행 중이다. source 해시를 바꾸지 않고
ignored `stage58-mixed-font-prefix-correction-proposal.json`에 수정안을 준비했다.
실제 빌드 owner 종료를 확인한 뒤 원본 파서 증거와 수정 묶음을 적용하고,
Native/fresh WASM 영향 페이지·대조군을 새로 출력한다. 아직 구현·최종 통과가 아니다.

## 19쪽의 같은 호스트 점유 소유 조사

최신 19쪽은 81.85884%이며 새 review를 직접 표시했다. 오른쪽 아래 문27의
시작·그림·풀이가 앞당겨진다. 독립 PDF bbox의 문26·문27 시작은 각각
270.024·690.504이며 같은 Native TextLine은 270.1·672.5다. 앞 문항은 맞지만
다음 제목이 약 18만큼 일찍 시작한다. 이는 관측값이며 고정 기대값이 아니다.

직전 본문 379·미주 내부 5·가상 문단 917의 원본 text는 개행 한 개이며,
TAC 그림 줄과 빈 줄 두 개의 저장 lineSeg가 있다. 마지막 빈 줄 높이 900과
후행 간격 452가 원본에 남는다. Native TextLine도 그림 줄 473.2~642.3 뒤에
빈 줄 648.3~660.3을 표시한다. 소유가 같은 FullParagraph와 Shape가 차례로
방출되고 Shape의 흐름 전진은 0이다. 다음 제목은 본문 393·내부 0·가상 918이다.

paragraph_layout은 실제 소비한 빈 줄의 끝을 last_item_flow_line_bottom에
전달하지만 column item 루프는 다음 Shape 앞에서 그 값을 NaN으로 지운다.
뒤의 occupied_end와 origin_after_gap이 같은 호스트의 빈 줄 끝 대신 그림 끝을
소비할 수 있다. 원본의 빈 줄+후행 간격 전진이 관측 차이와 대응한다.
이것은 코드 검토의 원인 후보이며 항목별 실제 소비 값의 수정 전후 확인이 필요하다.
원본·다음 미주·정확한 코드 경로와 관측값은
`stage58-nov-tac-empty-tail-ownership-diagnostic.json` 및
`stage58-nov-source379.json`·`stage58-nov-source393.json`에 보존했다.

보정은 같은 문단에 이미 배치된 TAC 호스트 글줄의 점유 끝을 후속 제어 항목에서도
이어받게 해야 한다. 가시 콘텐츠와 빈 줄 점유를 구분하고 다른 문단·단의 결과를
누적하지 않는다. 고정 높이를 더하거나 제목을 clamp해 맞추지 않는다. 같은 column
scratch 측정과 실제 배치가 이 결과를 소비하는지 함께 확인한다. TAC 그림 뒤 빈 줄,
빈 줄 없는 그림, 같은 호스트의 복수 제어 항목과 새 소유자의 첫 문단을 대조한다.
대조군을 조사한 뒤 이 보정을 붙은 글꼴 명령과 같은 구현 묶음에서 확인한다.

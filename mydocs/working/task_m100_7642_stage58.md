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

# #7751 289쪽 셀 안 묶음 화살표 원점 — 15단계

시작 head는 `5060c4bb2`이며14단계의 실제 제품 source는`465fd10fd`다.
14단계 전체 Rust10,594건·Skia4,043건·모든 lint/정책과 전383쪽 점수 검증은
종료코드0으로 끝났지만 추가 확대 판독에서 아래 배치 차이를 발견했다.
해소 전에는 PR을 생성하지 않는다. 같은 작업의 보정·재검증 승인은 유지한다.

## 독립 근거와 대상

같은 원본 HWP·기존383쪽 KoPub 한컴2024 Print·96dpi를 유지한다.
대상은10구역28문단의 표,4행2열 셀 안 두 묶음 개체와 일반 선이다.
독립 PDF의B/f* 경로를 CTM 적용 후 좌상단CSS px로 변환해 대조했다.

| 개체 | 현재 Native/Canvas | 독립 Print | 관측 |
| --- | ---: | ---: | --- |
| 첫 묶음 첫 가로 화살표 x | 232.5 | 258.76 | 약26.26px 왼쪽 |
| 두 번째 묶음 첫 가로 화살표 x | 230.2 | 256.5227 | 약26.32px 왼쪽 |
| 묶이지 않은 대각선 x | 365.3 | 364.8853 | 약0.4px 이내 |

원 devel baseline도 묶음의 같은 x를 내므로 이번 색조 변경의 신규 회귀는 아니다.
세로 원점은 앞 단계 표 흐름 수정으로 개선됐다. 현재 보정 문서의 미해결 배치이므로
이관하거나raw99.6%·테스트 통과로 면제하지 않는다. Direct PDF의Path 연결선 미지원
#7501과 이Line/Group의실제 원점 문제도 구분한다.

## 원인 추적·보정 계획

1. 원본 셀 문단의 저장 LineSeg·문단 여백·group common/행렬을 읽고 독립 기대 원점을
   연결한다. 합성한 메타데이터로 수용 조건을 완화하지 않는다.
2. 셀 안 그룹의 원점을 생산하는 호출부에서 측정·실제 배치·하위 행렬 소비까지 추적한다.
   그룹 자체 bbox나 자식 render_tx를 임의로 더하는 보정 대신 잘못 선택한 원점 가정을 제거한다.
   단일 선·일반 도형·중첩 그룹·회전 경로의 적용 여부를 확인한다.
3. 실제 입력의 독립 좌표/포함·원본 변환 관계로 before FAIL/after PASS를 확인한다.
   관련 formal 회귀는 `tests/cases/`에 두고 source-side 새 단위 함수/모듈은 추가하지 않는다.
4. 영향 쪽 Native/fresh WASM·정상 대조를 먼저 확인하고 최종 source의 전383쪽을 갱신한다.
   90% 최소·devel 대비 하락·완료 쪽98%·직접 판독 조건을 그대로 적용한다.
5. source 변경 뒤 필수 회귀·Skia·lint·브라우저 검증을 완료하고 승인된 PR·같은 PR의
   trailing archive review/오늘할일·CI·merge·후속 처리로 이어간다.

독립 PDF path·baseline/현재 그룹 노드·확대 PNG·실행 상태는 ignored
`output/pr-review/pr7494-20261016/stage14-p289-*`·`stage15-*`에 보존한다.

## 실제 호출 경로와 첫 보정

원본 문단1206의 원시 왼쪽 여백4000(1/2 HWPUNIT)이 resolved26.666667px이며
저장 LineSeg.column_start=2000HU와 일치한다. 들여쓰기−2136은 첫 글줄 계약이며
문단 기준 떠 있는 개체 원점에 더하지 않는다. 그룹별 값·문서 식별자로 분기하지 않는다.

- 전체 셀 `table_layout.rs::layout_cell_content`의 도형 호출 두 곳,
  부분 셀 `table_partial.rs`의 도형 호출 두 곳,
  내장 표 `table_cell_content.rs::layout_embedded_table`의 도형 호출이 원본para_shape_id를 전달한다.
- 공통 `layout_cell_shape_with_parent_path`가 resolved 문단 좌우 여백으로 같은 가로 기준
  원점/폭을 만들고 Left/Center/Right 배치가 소비한다. 세로 원점·높이·흐름 예산은 바꾸지 않는다.
- `layout_shape_object`의 Group 분기는 그 최종 원점을 받아 자식의 누적 행렬 render_tx/ty에
  한 번 더한다. 중첩·회전 그룹의 행렬이나 bbox 원점을 다시 누적하지 않는다.
- TAC는 이미 예약한 글줄 좌표를 그대로 사용하고 문단이 아닌 가로 기준에는 여백을 더하지 않는다.
  문단 여백0도 종전과 같다. 그림·표의 별도 위치 계약은 변경하지 않는다.
- 본문 `shape_layout.rs`의 문단 기준 원점은 이미 같은 resolved 좌우 여백을 사용한다.
  가로 원점만 변경하므로 측정의 요구 높이·분할 컷은 동일하며 최종 도형 배치가 이 결과를 소비한다.

수정 전 실제465fd10fd 라이브러리로 독립 Print 좌표 계약을 컴파일(exit0)한 뒤 실행했다.
첫 그룹 x232.466667 대 Print258.76, 차이−26.293333px로 assertion이 실패(exit101)했다.
빌드 실패가 아닌 실제 배치 결함 재현이다. 명령·stdout·stderr는
`stage15-origin-before-command.json`, `logs/stage15-origin-before-{build,test}.log`에 보존한다.
새 정식 테스트 추가는 영향 페이지와 전쪽 Native/fresh WASM 확인 뒤 진행한다.

## Native 첫 결과

공용 target·jobs8의 release-test Native bin/lib 빌드가 모두exit0으로 끝났다.
같은 독립 Print 좌표 검사는 컴파일exit0/실행exit0이며 첫 그룹 x259.133333
(Print258.76, +0.373333px), 둘째x256.853333(Print256.5227, +0.330633px)다.
이 결과만으로 시각 완료나 PR 준비를 선언하지 않으며 현재 영향 페이지를 캡처 중이다.

첫 source commit 뒤 끝난 cargo fmt의 tuple 줄바꿈만 build 시작 전 적용돼 있었다.
실제 producer는cef058e45 + `stage15-native-producer-format.diff`이고 세 생산 파일의
실제SHA-256은 `stage15-native-producer-sources.json`으로 고정했다. 이 후속 커밋이
같은 byte를 포함하며 소스 의미 변경을 추가하지 않는다. Frozen bin/rlib·명령·오류코드는
`stage15-native-provenance.json`, `stage15-build-results.json`에 남겼다.

## 全383쪽 재검증과 정식 회귀

| 범위 | Native raw | fresh WASM raw |
| --- | ---: | ---: |
| 최저 | 93.07066 | 92.98362 |
| 135쪽 | 98.05498 | 98.03088 |
| 289쪽 | 99.74756 | 99.72209 |
| 90% 미만·누락·고정 devel 대비0.01%p 이상 하락 | 0 | 0 |

두 backend 모두14단계 대비135·289쪽의x만 바뀌었고 나머지381쪽은 render tree와
점수가 같다. 전체11개 노드(사각형1·그룹2와 자식선8)의 가로 좌표 외에 세로·크기·
문구·소유·노드 순서 차이는0건이다. 전383쪽 Native/WASM의 내용·소유 일치 감사도
내부usize sentinel 대응 외에는 차이0건이며11쪽 각주1개를 유지했다.

Native와fresh WASM의135·289쪽overlay를 직접 열어 강조 사각형과 그룹 화살표의
개선·일반 화살표 보존을 판독했다. 135쪽 독립 Print의빨간S경로 bbox는
(112.198667,358.365333,66.968,43.836)px, Native는
(112.32,358.506667,67.04,43.72)px다. 원본3구역47문단 셀0의문단1은 좌우여백각1000
(1/2 HWPUNIT), 저장column_start500HU이며 문단은Center지만 도형은Para/Left다.
이 일반 사각형도 그룹과 같은 문단 기준을 소비해 개선됐고 문서ID 분기가 없다.

같은 동작의 우선 검증은99%대289쪽 그룹으로 하며,135쪽은 다른도형종류·문단Center/
도형Left라는 원본 경계가 있는98%대 완료 페이지다. 두 쪽 모두양쪽98%·전쪽90%·
직접 판독을 충족한 뒤 `tests/cases/issue_7751_handbook_completed_boundaries.rs`에 추가했다.
기존QA검사는두그룹8화살표의독립좌표±2px·표/셀/문단소유·화살표종류·누락/중복을
확인하고, 정상일반화살표2개의위치를유지한다. 새135쪽검사는사각형의독립좌표/크기±2px,
원본화면그림안포함과문구보존을함께보호한다.

정식 파일을 frozen14 라이브러리로 컴파일(exit0)·실행하자 기존4PASS/새경계2FAIL
(exit101)이었고 원인은각각26.29px/6.55px의독립Print x차이였다. frozen15로 동일
파일을컴파일(exit0)·실행하자6PASS/0FAIL(exit0)였다. 기존중첩글상자·TAC6건,
그룹수준roundtrip2건, 그룹그림crop2건도모두PASS였다. 이10건은정상대조이며
합성자료의통과를독립Print의시각일치로확대해보고하지않는다.

기존Direct PDF의289쪽Line화살표는손실없는replay미지원이라는명시적오류(exit1)를
확인했다. 이 경로를부분출력성공으로세지않으며 이번셀앵커결함과구분한다.
63쪽Path연결선·130쪽SVGpayload의거부와함께지원한계를기록하고기존guard를완화하지않는다.

첫선택Native비교명령은9PNG대383PDF에--pages를빠뜨려누락guard가거부했다.
명령을고쳐같은새PNG9개로성공했고제품결함/회귀실패로세지않았다.
WASM빌드는542a8bcbf에서exit0, publicJS/WASM동기화해시일치를확인한뒤이번wrapper가
생성한trackedJS만복구했다. frozenpackage와실제출처는stage15-fresh-wasm-provenance에보존했다.

필수전체회귀·Skia·lint·Studio·브라우저대조군의최종검증은이회귀추가head에서이어간다.

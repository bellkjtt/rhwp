# 이슈 #7642 — 한컴 수식 글자와 구분자의 논리 전진폭

이전 단계: [수식 문단의 내어쓰기](task_m100_7642_stage10.md).
시작 head: `c70b06ffc`. 17쪽은 95.33926%지만 괄호/등호 및 수식 내부 간격 차이가 남는다.
전체 원본의 전수 비교와 관계형 회귀의 선행 조건을 유지한다.

## 독립 근거와 적용 범위

정상 `samples/hwpx/eq-002.hwpx`의 16개 수식만 바꾼 진단 대조군을 만들었다.
6/9/12/18pt에서 Roman E, Italic X, 명시적 작은 공백, 보통 LEFT/RIGHT 괄호를 비교한다.
오래된 크기/기준선/저장 LineSeg를 비우고 한컴 2020이 재생성한 저장본과 같은 저장본의
MCP Print를 읽는다. 실제 한컴은 `11.0.0.9136`, Print method 0 / one-up driver다.
job `c5dd6969-4a2b-4aeb-a23b-9eb6fcf0ad78`(저장)와
`fe4b18ec-9d06-44cd-ac27-92fa699e27ea`(Print)의 상태·해시는 ignored output에 있다.
수동 합성 대조군의 통과를 원본 통과로 기록하지 않는다.

재생성 논리 폭은 설치 HYhwpEQ의 hmtx(UPM 1024)를 75 HWPUNIT 격자로 반올림한
글자 전진폭과 대응한다. 작은 공백은 1/8 em이며 보통 괄호의 글리프 전진폭 밖에
추가 내부 여백을 예약하지 않는다. 9pt에서 E=600, X=675, X+작은 공백=787,
E LEFT(X` RIGHT)=2137 HWPUNIT이다. 괄호 두 개의 각 375와 문자/공백의 합과 일치한다.
여러 글자 크기에서 같은 계산을 확인한다. 이 값은 문서의 화면 px 위치가 아니다.

## 원인 경로와 보정 계획

`Equation.font_name/BaseUnit → EqLayout::for_equation → Text/Space/Paren의 논리 폭 →`
`Row의 자식 원점 → 공통 layout box → SVG/Canvas/Skia 표시`를 대조한다.
현재 HYhwpEQ 소문자 이탤릭에만 원본 전진폭을 쓰고 대문자/정체는 대체 글꼴 폭을 쓴다.
보통 괄호는 옛 Times 폭과 0.08em 내부 여백, 작은 공백은 0.17em을 추가해 저장 프레임과
표시 내용이 어긋난다. source font의 글자 메트릭과 명시 공백 계약을 공통 결과에 반영한다.
사설 글꼴 바이트/윤곽을 저장소에 추가하지 않는다. 다른 face와 큰 구분자의 별도 계약은
구분해 정상 대조군으로 확인한다. 위첨자의 추가 전진은 별도 크기·문자 대조군으로 확인하며
관측한 한 문자에만 조건을 더해 맞추지 않는다.

## 검증

먼저 같은 원본 17쪽과 같은 576dpi 영역을 다시 대조한다. 확인된 출력 보정 후에만 회귀를
변경한다. Native/fresh WASM·전체 페이지 최저 90%와 확대 판독을 모두 완료하기 전에는
PR 제출·전체 시각 개선 완료로 기록하지 않는다. TSV/JSON/로그는 ignored output에 보존한다.


## 공통 소비 지점 보정

괄호 폭을 바꾼 뒤 SVG/Canvas/Skia가 기존 `0.333em`을 다시 계산하면 오른쪽 원점이
다시 달라진다. 좌우 측정 폭을 `LayoutKind::Paren`에 보존해 이 backend들이 직접 소비하도록
했다. paint JSON에도 같은 폭을 전달하고 Studio CanvasKit의 일반 괄호 원점·기준선도 이
결과를 사용한다. 폭이 없는 옛 paint JSON은 SVG 대체 경로로 돌려 임의 재계산을 하지 않는다.
기존 golden dumper의 패턴은 새 필드를 수용하는 문법만 바꿨고 기대값은 갱신하지 않았다.

첫 stage11 진단 빌드는 남아 있는 괄호 폭 재계산을 발견해 exit 130으로 중단했다.
출력 공백 때문이 아니며 해당 source로 시각 검증을 수행하지 않았다. 소비 지점을 연결한
후 `--profile release-test`(같은 최적화, LTO 없음)의 Native 진단 빌드를 다시 시작했다.


Studio의 `./node_modules/.bin/tsc --noEmit`는 exit 0이다. 실행 로그는
`output/pr-review/issue7642/failed-inputs-20261008/stage11-studio-tsc.log`에 있다.
이는 최신 WASM이나 Studio 실제 화면의 시각 검증을 대체하지 않는다.

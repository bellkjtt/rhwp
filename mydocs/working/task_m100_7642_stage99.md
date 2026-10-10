# Stage 99 — 대문자 복합 첨자의 전진과 공통 여백

Issue: #7642. 이전 구현은 `31b0f37c32ee7846f66bb9d53ccb039482948b46`다.
전체 55원본/853쪽과 요청 회귀 292개, #1139 전체 관계형 전환 범위를 유지한다.
HML 수식의 명시적 제외를 유지하며 미달 원본·기존 검사를 제거하지 않는다.

## 독립 근거와 원인

Stage98의 12종 전체 점수는 모두 90% 이상이나 대문자 확대에서 차이를 확인했다.
독립 정상 저장/Print 16식에서 위첨자 여백은 Sup와 SubSup에 동일하게 나타난다.
SubSup는 이 여백을 소비하지 않아 위첨자가 본체에 붙는다. 아래첨자의 원점은 유지하고,
위첨자만 단일 Sup와 같은 대문자 여백 helper를 소비한다. 전체 폭은 두 자식의 실제
점유 끝으로 계산한다. 구분자·공백 뒤와 소문자는 여백을 추가하지 않는 대조군이다.

22pt 이탤릭 X의 본체 전진은 독립 Print에서 약 1725HU다. 현재 1650HU는
논리 크기 30에 기존 측정 표가 없어 nominal 2200HU로 fallback한 결과다.
이미 독립 Print로 확인한 2250HU logical em과 설치 face의 hmtx 격자를 일반
미측정 ASCII 전진 fallback에서 공유한다. 특정 문자나 문서의 좌표 예외를 만들지 않는다.
측정한 기존 전진 표는 그대로 우선하며 표시 outline의 사설 곡선을 추출하지 않는다.

## 소비 경로와 검증

논리 em → ASCII hmtx 전진 → 본체 폭 → Sup/SubSup의 위첨자 원점 → 실제 점유 끝 →
SVG·Canvas·Skia·Paint JSON이 같은 LayoutBox를 소비한다. 아래첨자 및 적분의 별도
원점 계약은 유지한다. 측정 이후의 별도 x 덮어쓰기 여부도 실제 자식 원점으로 확인한다.

두 수정은 함께 적용하고 fmt·Native/WASM Clippy·Skia check·Native 빌드를 순차 실행한다.
동일 12종/13쪽 Print를 재출력해 전체 점수와 대문자/기존 marker 확대를 확인한다.
새 Native/fresh WASM 전체 TSV·원본 확대 판독 뒤 관계형 회귀를 변경한다.
전체 필수 검증·새 PR·CI·머지 후속 처리까지의 목표는 미완료 상태로 유지한다.

## 구현 반영

Sup와 SubSup의 위첨자 x가 공통 `script_uppercase_gap`을 소비한다. SubSup의
아래첨자 x는 본체 전진 뒤에 두고 전체 폭은 두 첨자의 실제 점유 끝으로 계산한다.
측정 표에서 얻은 ASCII 전진은 우선하며 표에 없는 크기만 기존 `em_advance`의
logical em을 사용해 hmtx를 반올림한다. 독립 Print의 22pt X 전진과 연결된다.

fmt 실행은 종료 코드 0이다. 새 구현의 lint·빌드·12종 시각 대조와 전체 검증은
아직 미실행이다. 정식 회귀 추가·기대값 변경은 하지 않았다.

## 초기 정적 검사와 문자 전진의 독립 대조

구현 SHA는 `3d89e1f45`다. `stage99-brace-type-checks.json`의 fmt·Native Clippy·
WASM Clippy·Skia lib check는 모두 종료 코드 0이다. 전체 workspace/회귀 통과는 아니다.
Native 빌드와 실제 출력 대조는 실행 중이다.

ASCII 전진 변경의 일반성을 확인하기 위해 11/22pt의 대소문자 전체와 정체·이탤릭
8식을 추가 정상 저장/Print했다. 정상 저장 job은 `1ad5bf74-c099-4bec-af7d-a12066ab26c2`,
Print는 `4f94a68e-d3eb-4816-98bf-beebff2809c4`다. 실제 한컴 11.0.0.9136, engine 2020,
method 0·one-up이며 PDF는 전체 1쪽이다. `stage99-ascii-advance-counterexamples`의
8식 원문·BaseUnit 보존과 PDF 각 행의 문자 27개를 확인했다. 분석용 HWPX는 로컬 export다.

각 문자열 끝에 대조 문자를 두어 A..Z/a..z의 실제 다음 문자 원점으로 전진을 읽었다.
`print-advance-origins.json`의 208개 glyph 전진은 설치 face의 hmtx와 logical em의
75HU 반올림 격자에 모두 일치한다. nominal em만 사용하는 값은 11pt에서 29개,
22pt에서 38개가 다르다. 기존 측정 전진 표는 그대로 우선하며, 이 대조는 미측정
fallback의 logical em 공유를 입증한다. 사설 font 곡선은 추출하지 않았다.

이 추가 8식을 포함한 실제 Native 대조는 총 13문서/14쪽/208식으로 확장한다.
전체 55원본/853쪽 검증 범위를 대신하지 않는다. 새 확대·fresh WASM·전체 검증과
관계형 회귀 변경·PR 절차는 미완료다.

# 이슈 #7642 — 수식 관련 HML 회귀를 #7445로 이관

이전 단계: [원본 크기 0과 떠 있는 수식](task_m100_7642_stage8.md).

## 사용자 지시와 변경 범위

사용자가 수식 관련 HML 회귀 검사를 제거하고 나머지를 먼저 전수 검증하도록 지시했다.
새 이슈를 만들지 않고 열려 있는 #7445에 원본·독립 Print·현재 차이·복구 조건을 연결한다.
이관은 출력 개선 완료나 PASS가 아니다.

- HML parser의 수식 전용 검사 8개, serializer의 수식 전용 검사 4개,
  WASM의 수식 경고 보존 검사 1개를 제거한다.
- 복합 HML 저장 상태 검사의 수식 경로를 분리하고 일반 HML/비-HML DTO 검사는 유지한다.
- 원본 HML·한컴 HWP/HWPX 저장본·수동 2024 Print 및 2020 MCP Print는 그대로 보존한다.
- 누락 RATIO·RELSIZE·ShadeColor의 일반 저장 계약 검사와 다른 HML 검사는 유지한다.
- 제거 전 검사 본문·이름·source SHA는 이관 자산에 보존하며 자동 test 수집에서 제외한다.

## 검증 순서와 후속 범위

현재 Native 빌드가 끝난 뒤 테스트 소스를 변경한다. 테스트 목록을 새로 준비하고
HML의 남은 일반 계약 검사·lint·정책 비교를 실행한다. 전체 nextest를 이후 전수 실행하며
제거한 검사를 통과 개수로 세지 않는다. 입력 37개 중 HML 1개는 #7445로 이관하고
나머지 36개의 전체 페이지 Native/fresh WASM·직접 확대 판독을 계속한다.
기존 원본 manifest는 당시 범위로 보존하고 새 활성 manifest는 ignored output에 둔다.

원래의 다른 실패 입력·시험지·#1139 전체·회귀 관계형 전환·rebase·최종 lint·PR/CI/merge와
후속 처리 범위는 유지한다. 관련 원본의 최저 90%와 확대 판독 전에 회귀 기대값을 바꾸지 않는다.
오늘할일과 번호 기반 PR review는 code head CI 성공 뒤 trailing commit으로 작성한다.

## 이관 실제 적용과 정책 확인

수식 전용 parser 8개·serializer 4개·WASM 1개를 제거했고 복합 DTO의 일반 경로는
`test_hml_save_state_is_one_canonical_dto_for_hml_and_non_hml`로 유지했다.
제거 전 검사 본문은 `mydocs/pr/assets/issue7445/hml-equations-20261008/removed-tests.rs.txt`에
보존했다. 원본·PDF 및 일반 저장 계약 검사에는 변경이 없다.

[#7445 이관 댓글](https://github.com/edwardkim/rhwp/issues/7445#issuecomment-6056803377)을 게시하고
API로 본문 일치·작성자·BOM 없음을 확인했다. 이슈는 OPEN이다. 원본 HML의 Native/fresh WASM
현재 진단은 각각 1쪽·6.22797%이며 이관 자산 README의 binary/source 해시로 구분한다.

`--prepare` 후 manifest 및 source unit tier의 정책 검사는
base `f0e7228f6dd2ea1437724e53ad640c40c56d204b`를 고정해 모두 exit 0이다.
Native Clippy는 exit 0(1분 14초), WASM Clippy도 exit 0이다. 남은 HML 일반 계약 검사는 전체 nextest에서 확인한다.
전체 회귀·전쪽 시각·최종 PR 게이트를 완료했다고 표시하지 않는다.

직전 단계의 Native 위치 보정 빌드는 exit 0(14분 14초)이며 세 위치 대조군은 각각
7.09220→100%, 0→100%, 0→100%다. 수식 글리프의 확대 판독 문제와 HML 파생 진단은
#7445로 이관한다. 공통 개체 위치 보정의 fresh WASM 및 다른 입력 무회귀는 아직 확인해야 한다.

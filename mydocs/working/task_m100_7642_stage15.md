# 이슈 #7642 — 최신 devel 통합 후 현재 브랜치 전수 검증

이전 단계: [최신 devel 통합](task_m100_7642_stage14.md).
브랜치: `fix/equation-font-metrics-pagination-20261007`.
정책 비교 base: `74f9b71b15a2db18b17c37ceab6cf2b3c20c03c3`.

## 범위와 순서

1. 현재 source와 파생 integration suite로 전체 nextest를 실행한다. 공유 target은
   `/home/tsjang/rhwp/target/pr-review`, release-test, threads 12, no-fail-fast를 사용한다.
   실행 head·명령·최종 exit·실패 이름과 원본 매핑은 ignored output에 기록한다.
2. 같은 source의 Native/fresh WASM 시각 검증과 독립 Print PDF·확대 판독을 확인한다.
   이전 source로 실행 중인 36개 Native 비교는 이전 코드의 진단 자료로 보존한다.
   그 결과를 현재 source의 시각 통과로 대신하지 않는다.
3. 실패한 원본의 검증 범위를 확인한다. 정상 출력의 회귀 대상으로 부적합한 문서는
   [새 이슈 이관](../manual/pr_review/visual_fixture_evidence.md#90-미만-회귀-대상의-별도-이슈-이관)에
   따라 별도 등록·검사/원본/Print PDF 이관 후 해당 이슈에서 보정한다. 이번 수정으로 발생한
   실제 회귀는 이관으로 해결했다고 보고하지 않는다.
4. 시각 선행 조건과 확대 판독을 충족한 대상은 고정 px 대신 소유·순서·보존·포함·겹침 관계로
   검사한다. 관련 검사 변경 후 전체 회귀와 필수 Rust lint·정책 base 비교를 완료한다.
5. fresh WASM·Studio 산출물과 대표 시각 증적을 확보한 후보로 새 PR을 생성하고,
   code CI 성공 뒤 review/오늘할일 trailing commit과 trailing CI를 확인한다.
   검증된 정확한 head를 merge한 뒤 정식 후속 처리를 완료한다.

로그·TSV·중간 JSON·nextrun은 `output/pr-review/issue7642/`에만 보존한다.
현재 PR 준비 완료·전체 회귀 통과·최종 시각 수용을 선언하지 않는다.

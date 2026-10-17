# seo-rii 통합 검토 7단계 — 최신 devel 리베이스와 제출 형식

## 사용자 지시와 실제 base 변경

사용자가 TSV를 PR 본문에 직접 넣도록 지시했다. 원본 32개를 파일명·생산 SHA·SHA-256과 함께
`tsv` 코드 블록으로 그대로 표시하고 게시 뒤 API에서 원문 바이트를 재대조한다. 별도 Gist를
생성하거나 TSV를 Git에 커밋하지 않는다. 이 제출 방식은 이번 작업의 명시적 사용자 지시다.

PR 직전 확인에서 devel이 `fe5725dc384689b45d60ef8ea4c83138bc2701d8`에서
`b7a5388034ee66f58a7e1d5363974e83b6374f57`로 전진했다. 새 commit은 #7953이며 PDF 가져오기를 기본 feature로
활성화하고 parser·WASM·PDF backend와 일부 CI 경로를 변경했다. 원 PR 20개의 head는 그대로다.

- 리베이스 전 통합 head: `70ca10c208389dfa8f03c3a4f048c9551f8eaf51`.
- 리베이스 직후 통합 head: `a7aac1a9ce5b2b902471ecd8f234f7d72834812d`.
- 원 161개 commit은 순서·제목을 보존해 충돌 없이 재적용했다.
- 독립 `git merge-tree --write-tree <이전 head> <새 base>`의 tree와 리베이스 결과 tree가
  `f05c9543837ab60314293f0620bee1ee28d17548`로 같다. upstream PDF 변경과 기존 통합 결과를 모두 보존했다.
- `prepr-rebase-audit.json`에 161개 이전/현재 SHA mapping, `logs/prepr-rebase-range-diff.log`에
  범위 비교를 보존한다. 1–6단계의 옛 SHA는 당시 실제 빌드·실행의 역사적 증거로 유지한다.

## 새 head 검증 계획과 범위

기본 feature가 바뀌었으므로 이전 head의 10,643 PASS를 새 head의 결과로 재사용하지 않는다.
다음은 기본 작업공간의 공용 `target/pr-review`, Cargo 잡 8개를 사용해 순차 실행한다.

1. clean한 이 검토 worktree를 새 base에 임시 checkout하여 Native Skia CLI와 fresh host `--no-opt`
   WASM을 빌드하고 source·명령·해시를 별도 `rebased-base-*` 증거로 복사한다. 검토 branch로 돌아온다.
   사용자 dirty 상태인 기본 작업공간과 다른 worktree는 변경하지 않는다.
2. 통합 후보 Native CLI·fresh WASM을 같은 조건으로 빌드한다. HWP/HWPX 8개 전체 페이지를
   새 base와 대조하여 양쪽 90% 이상·0.01%p 이상 하락 없음·쪽수 일치와 직접 판독을 확인한다.
   이전 base의 TSV 32개는 역사적 원본으로 보존하며 새 base/후보 TSV와 구분한다.
3. suite/unit-tier 정책, 원 변경 Native 집중과 기존 HFT·새 PDF paint 경계, 전체 release-test
   nextest, 공식 Native Skia lib·missing-picture·Direct PDF를 확인한다. 전역 feature 변화로 영향을
   받는 현재 전체 목록을 실행하며 이전 실행 수를 새 기대값으로 고정하지 않는다.
4. format·Native/WASM/workspace all-target Clippy·workspace build를 새 head에서 통과시킨다.
   새 fresh WASM으로 Studio TypeScript·unit·build·renderer/실제 browser 계약을 확인한다.
   기존 소스의 release LTO 실행은 6단계 역사적 결과이며 새 head 실행으로 표시하지 않는다.
5. 필요한 실제 ABI 소비·대표 PNG를 갱신하고 현재 source/binary/PDF/TSV/PNG SHA를 기록한다.
   완료 후 개별 판정을 갱신하고 원격 branch는 이전 remote SHA를 lease로 확인해 갱신한다.
   TSV 원문을 포함한 PR 생성 → code head CI → 같은 PR review/오늘할일 trailing → trailing CI →
   정상 merge·post_merge 순서를 유지한다. 부모 #536 전체를 종료하지 않는다.

현재 상태는 **리베이스 완료 / 새 기본 feature의 검증 미완료**다. 6단계 통과와 구분한다.
검증 로그는 ignored `output/pr-review/seo-rii-20261017/logs/rebased-*.log`에 기록한다.

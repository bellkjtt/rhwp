# 이슈 #7642 — 미주 보정의 전쪽 Native·fresh WASM 검증

생산 소스 커밋은 `94ebd9c0a43c66b48a6ba1f94327e82b86731f65`다.
현재 작업의 회귀 실패는 모두 현재 브랜치에서 보정한다.

## 고정 근거와 순서

1. Stage36 frozen Native `rhwp-endnote-owned-span`의 24개 소스 해시와
   전체 Rust diff 해시를 생산 커밋과 대조해 일치함을 확인했다.
   바이너리 SHA-256은 `e89ec2b139a33921b0617b0d57c4cd43b0d683c2ca5217f8f63eb13ab7926a1f`다.
   진행 중인 `endnote-owned-span-native-all`은 2022 HWPX와 독립 2024 Print의
   전체 21쪽 비교다. source가 같은 문서 계획 커밋을 새로운 생산 소스로 혼동하지 않는다.
2. 공용 `/home/tsjang/rhwp/target/pr-review`에서 root WASM wrapper를 실행한다.
   로그·빌드 출처·package 해시를 ignored output에 고정하고 Studio public과
   root pkg의 JS/WASM SHA-256 일치를 확인한다. Native Clippy는 끝났으므로
   Cargo 동시 실행 없이 진행한다.
3. 이 fresh package로 같은 HWPX·2024 Print의 전쪽 TSV를 산출한다.
   HWP/2020 정상 대조도 전쪽으로 확인한다. 영향 쪽의 compare·overlay·review와
   수식 확대 이미지로 단 소속·누락·중복·간격·기호 모양을 직접 판독한다.
4. 간격이 없는 HWPX 15쪽 75.79976%, 수식 재현 입력의 미달과 확대 판독에서
   남은 수식 중괄호·첨자·기호 주변 차이는 현재 브랜치에서 계속 수정한다.
   부분 페이지의 점수 통과를 전체 완료로 바꾸지 않는다.
5. 요청된 52개 원본(독립 Print 총 783쪽)과 관련 실패 입력을 같은 조판 세대에서
   검증한다. 출력 보정·전체 범위 Native/fresh WASM 90% 이상·확대 판독을
   먼저 충족한 뒤 필요한 회귀 검사를 고정 px에서 소유·순서·보존·포함·겹침
   관계로 변경한다. 기존 110건 실패 목록은 새 전체 실행으로 갱신한다.

## 진행 기록

명령·결과·TSV·JSON·nextrun은 ignored
`output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/`에만
보존하며 Git에 stage하지 않는다. 기본 검토 target을 삭제하거나 새 target을 만들지 않는다.
현재 전체 Rust·WASM lint/정책·전체 회귀와 PR/CI/merge는 미완료다.
최신 code head CI 이후에 PR review·오늘할일 trailing 기록을 커밋하는 기존 순서를 유지한다.

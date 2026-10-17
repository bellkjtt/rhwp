# PR #7985 적용 기록 — 교차 구역 선택 편집 사전 거절

## 신원과 범위

- 원 PR: https://github.com/edwardkim/rhwp/pull/7985
- 원 author: donggyun112 <ssddgg99@daum.net>.
- 원 head `5dd0c86ac1a86c977e9d80a7fe263ac60358f720` → 적용 `3930b9cdbe8c8ec706e9dcb48564cfb17f43c63c` (`-x` 원 SHA 보존).
- 누적 순서: 계획 `a8503c75a` → #7978 `2d7c0bcf0` → #7985 `3930b9cdb` → 보정 `a7c9e30ffebc843d5f5763f349f79ad3ea93cfb1`.
- base `f9d2ab17ab438c6bed97d91c3c2a98a5be27e573`. 기본 작업공간에서 검토했으며 별도 worktree를 만들지 않았다.
- [최종 검토](pr_7985_review.md)에 원 기여와 보정 범위·실제 실행·미검증을 연결한다.

## 충돌과 메인터너 보정

`MANIFEST.md`의 undo 설명 충돌은 #7977의 HF/FN 추가 설명과 #7983의 신규 항목을 둘 다 유지했다.
제품 보정은 `onInput`의 선택 허용 검사 순서 한 곳이며, 실제 누름틀 helper보다 먼저 검사한다.
#7978의 여섯 scalar 수정은 원 기여 그대로다. #7985에는 앞선 검사와 거절 불변식의 테스트를
추가했다. 기존 base의 가운뎃점 E2E 등록 누락 한 행도 보완했다.

## 실제 실행 증적

로컬 원본 영수증은 `output/pr-review/donggyun112-20261017/`의
`candidate-state.json`, `gate-progress.json`, `browser-and-build-gates.json`,
`fresh-wasm-identity.json`, `negative-controls.json` 및 `logs/`에 있다.
[검토 결과](pr_7985_review.md#공통-로컬-검증)의 명령·수치와 대응하며 미실행을 PASS로 적지 않는다.

fresh WASM은 #7985 적용 직후 Rust 입력에서 빌드를 시작했다. 최종 메인터너 보정은 Studio
파일뿐이고 Rust/build 입력 차이 0을 확인했다. 검증 시 JS SHA-256은
`7dcd8775b5ff2e6d6d46f6811d03cd3e0f0db9d7d0f0d1a0606741e762bce06e`, WASM SHA-256은
`06a6d021009d04717adc1d72faf5b5d8e082f79e2bde7c4da428a4da48990b20`다. `pkg/`와 public의 동일 바이트를 실제 브라우저가 사용했다.

## 남은 원격 단계

현재는 로컬 검토·보정 완료 상태다. 원격 push·통합 PR 생성·code head CI는 미실행이다.
사용자 승인 뒤 최신 base/head를 다시 확인하고 통합 PR을 만든다. 정확한 code CI 성공 뒤에만
같은 PR의 개별 review·오늘할일 trailing을 확정하고 최신 trailing CI로 merge를 판단한다.
원 PR에 아직 결과 댓글이나 close를 게시하지 않았다. fork branch는 보존한다.

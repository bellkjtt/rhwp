# #7494·#7751 최신 devel 리베이스와 제출 검증 — 14단계

새 base는 `b7a538803`(#7953)이며 이전 base는 `fe5725dc3`이다.
리베이스 전 head `b8ce9d753`와 전체 작업을 backup branch로 보존했다.
30커밋은 충돌 없이 적용됐고 리베이스 직후 head는 `843164b5a`다.

## 변경 영향과 검증 순서

새 base는 PDF 가져오기 기본 feature와 parser/PDF·의존성·WASM 노출을 바꿨다.
기존 HWP 조판·paint·그림 처리의 소스 diff는 없다. 이전 출력은 실제 이전 SHA의
증거로 보존하며 새 head의 실행으로 바꾸어 기록하지 않는다.

1. 공용 `/home/tsjang/rhwp/target/pr-review`에서 jobs8로 Native와 fresh WASM을
   순차 빌드한다. WASM은 기존 host 대체 경로와 codegen-units16을 사용한다.
2. 같은 HWP·기존 독립383쪽 Print·글꼴을 사용해 새 제품 전쪽 Native/fresh WASM을
   다시 산출한다. raw90%·누락0·쪽수·고정 devel 대비 하락 기준을 유지한다.
   대표 PNG와 실제 Studio DOM·SVG·Direct PDF 소비도 새 제품에서 확인한다.
3. 정식 focused·전체 nextest·Native Skia3종·모든 Rust lint와 suite/unit 정책을
   같은 target에서 순차 검증한다. 중단된 이전 nextest를 전체 통과로 세지 않는다.
4. upstream HEAD를 다시 확인하고 승인된 통합 PR을 제출한다. code candidate의
   CI 성공 뒤 같은 PR에 archive review·구현 기록·오늘할일을 trailing commit한다.
   trailing CI·정상 merge·post_merge.md의 issue/comment/정리까지 완료한다.

TSV·명령·실행 결과·빌드 지문은 ignored
`output/pr-review/pr7494-20261016/stage14-*`에 보존한다.
GitHub 본문에는 전쪽 TSV와 정확한 head SHA의 실제 PNG를 표시한다.
아직 실행하지 않은 단계는 완료로 판정하지 않는다.

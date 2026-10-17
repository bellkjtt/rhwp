# seo-rii 통합 검토 8단계 — #7964 반영과 실제 소비 경로 재검증

## 기준과 완료된 선행 검증

Issue: #536. 원 contributor 비 draft PR20개 범위와 source head는 그대로다. draft Qt·packaging은 제외한다.
[7단계](task_m100_536_integration_review_stage7.md)의 source751/baseb7 전체 회귀10,770 PASS,
실제 FFI·Native/fresh WASM 증거는 당시 결과로 유지한다. 새 base의 완료 증거로 바꾸지 않는다.
TSV 본문 삽입·본문 크기 제한 시 SHA 고정 원본 링크 제출 지침은 `92e4f518a`에 기록했다.
사용자가 취소한 `post_merge.md` 수정은 포함하지 않는다.

최신 `upstream/devel`은 #7964 merge `9415b920c8dec299e8ada69b01dbd1c3c044f1f1`이다.
한컴 그림 색조·WMF crop·물리 인쇄 종이·저장 줄과 셀 도형 원점·Direct PDF가 바뀌었다.
사전 merge-tree는 `src/renderer/skia/image_conv.rs`의 콘텐츠 충돌1개를 검출했다.
완료된 이전 gate 결과를 commit한 뒤 실행 중인 Cargo/Rust가 없음을 확인했다.
사용자 dirty `rhwp-studio/public/rhwp.js`가 있는 기본 작업공간은 변경하지 않는다.

## 통합 근거와 계획

1. 같은 검토 branch를 새 devel 위로 rebase한다. 기존 source author와 원 SHA를 보존한다.
   충돌은 원 commit의 일반 동작과 새 upstream 동작을 실제 호출 경로로 대조해 해소한다.
2. 이미지 경로는 `image_resolver::adjusted_picture_bytes`의 한컴 색조 바이트 생산 →
   Skia `draw_image_bytes`의 정규화·decoded-image 캐시 → 색 필터·sampling → 실제 draw로 추적한다.
   이미 bake한 색조에 효과·밝기·대비를 다시 적용하지 않고, 캐시 key의 입력 바이트와 변환 조건,
   source-pixel 이산 효과·nearest sampling·실패 fallback을 함께 확인한다. gradient와 Direct PDF의
   자동 병합도 검토하여 색조 지원과 기존 gradient admission을 둘 다 보존한다.
3. 공용 `target/pr-review`, Cargo 잡8개로 새 base와 후보 Native Skia CLI·fresh host `--no-opt`
   WASM을 순차 빌드한다. Docker 최적화 배포 검증으로 표시하지 않는다. 이전 산출물은 해시 고정한
   역사적 결과로 보존하며 새 producer의 실행을 대신하지 않는다.
4. 같은 원본8개 전쪽 Native/fresh WASM을 비교한다. 최신 정책의 `silhouette_raw_match_percent`로
   양쪽90% 이상·쪽수1:1·누락0·devel 대비0.01%p 이상 하락 없음을 확인하고 대표 PNG를 직접 판독한다.
   분자·괄호/폼 bevel의 기존 차이와 미달 단독 극한 진단은 전체 해결로 표시하지 않는다.
5. 기존 Native 집중·HFT·새 그림 색조/WMF/용지 경계, 공식 Skia·Direct PDF, 실제 ABI feature-on/off,
   fresh Studio/browser, 변경 범위의 Rust lint·workspace build·전체 회귀를 새 source에서 확인한다.
   기능 코드가 실제로 바뀐 통합 head의 검증이며 지침 Markdown 수정 때문에 회귀를 반복하는 것이 아니다.
6. 완료한 실제 결과만 개별 review20개와 구현 기록에 연결한다. 최종 TSV32개·대표 PNG20개를 새
   source로 갱신하고, producer SHA와 asset head를 구분한다. 원격 base/source head 재확인 후 자신의
   통합 branch만 정확한 이전 remote SHA의 lease로 push한다. PR의 실제 CI → 같은 PR의
   review/오늘할일 trailing → trailing CI → 정상 merge·post_merge 순서를 유지한다.

현재 판정은 **새 base 반영·검증 진행 중**이며 PR은 아직 생성하지 않았다.
실제 명령·로그·해시는 ignored `output/pr-review/seo-rii-20261017/latest-*`에 기록한다.

## 실제 리베이스 결과

리베이스를 완료했다. 직후 head는 `ec742f167991fd8c5efcd01ab7e8fe752c0a070e`이며164개
commit의 수·순서·제목과 원 contributor author를 보존했다. 독립 merge-tree와 비교한 수동 해소
경로는 `src/renderer/skia/image_conv.rs` 하나다. cache 도입·이산 효과·cache-hit sampling·진단
commit의 같은 파일 충돌을 순서대로 해소했다. 나머지 자동 병합 결과는 독립 tree와 같다.

한컴 색조 보정은 SVG/JPEG 정규화 뒤, decode 및 이산 효과 전의 cache miss에서 수행한다.
보정 PNG는 같은 cache/배치 경로로 중립 효과·밝기0·대비0으로 재생한다. 원본 decode key에는
색조 조건도 포함해, 먼저 중립 그림을 캐시한 뒤 같은 원본을 다른 밝기/대비로 그릴 때 보정을
우회하지 않게 했다. 기존 이산 효과의 source pixel 처리·nearest sampling·실패 fallback과
bounded cache 관측을 유지했다. 지원하지 않는 색조의 필터 합성 순서·linearRGB fallback도
새 upstream과 대조해 보존했다. 실제 실행 결과는 다음 검증이 완료된 뒤 추가한다.

`latest-rebase-audit.json`에 이전/현재164개 SHA mapping과 tree 대조를 보존한다.
이전 전체 회귀는 source751/baseb7 결과이며 최신 리베이스 결과의 회귀 통과로 표시하지 않는다.

## 실제 Native lint 오류와 보정

producer `8af3e72bde9e62cfb4daa1e9f7663bb3a244c282`에서 Native 집중288PASS, Skia lib4043PASS/12ignored와 fresh Studio10종을 확인했다. 이후 feature-on FFI Clippy는 `renderer/skia/renderer.rs`의 `needless_option_as_deref` 2곳으로 exit101이었다. 리베이스 자동 병합 뒤 `resolved.as_deref().or(fallback.as_ref())`가 이미 `Option<&ResolvedImagePayload>`를 생산하므로 다음 `as_deref()`는 같은 타입을 다시 역참조한다. 두 호출만 제거하고 동일 `map`/`is_some_and` 소비와 fallback·효과 억제 동작을 유지한다. lint 경고를 허용하지 않는다.

이 변경은 같은 payload 참조를 소비하는 타입상 항등 정리지만 실제 source가 바뀌므로 이전288PASS·4043PASS를 새 commit 실행으로 표시하지 않는다. 완료한 base9415 산출물은 재사용하고, 후보 Native/fresh WASM·시각 증거 및 영향 gate를 새 producer에 연결한 뒤 남은 전체 회귀·lint를 진행한다. 전체 회귀는 아직 이번 base에서 실행 전이었다. 이전 source751/baseb7의10770PASS는 역사적 결과로 유지한다.

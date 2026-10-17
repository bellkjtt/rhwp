# PR #7892 — feat(renderer): add native replay caches and visual diagnostics (P47)

## 최종 판정

**머지 보류 — 최신 devel의 PDF 기본 feature 변경을 리베이스했고 새 head 검증 중.** 이전 head의 로컬 통과 결과는 역사적 증거이며 현재 head 완료로 표시하지 않는다.

## 접수 정보

- [원 PR](https://github.com/edwardkim/rhwp/pull/7892), 기존 contributor `seo-rii`, base `devel`, non-draft.
- 원 head `6abf22be0cb0fa10f0a5b9dfe57bdd2539e5df4f`, 검토 base `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- 접수 시 규모 +14587 / -1245 / 111 files는 누적 source diff다. 선행 PR을 중복 적용하지 않는다.
- reviewer `jangster77` 지정. 작성 시 mergeability는 참고값이며 실제 통합 후보와 최신 head를 다시 확인한다.
- [구현 순서](pr_7892_review_impl.md), [전체 계획](../../plans/pr_review_seo_rii_20261017.md).

## 변경·검증 범위

- 구현 주장: Native Picture·glyph·image cache에 profile·font/environment 상태를 반영하고, 공급 font의 family·style·coverage 선택과 변환·sampling·효과의 실제 소비를 유지한다.
- 실제 소비 경로: `renderer/skia/renderer.rs → cache·font_catalog·text_replay·image_conv·fill_shader; render_backend/skia_adapter.rs`. 아래 검사는 source 단독 CI가 아닌 통합 후보에서 연결한다.
- 관련 검사: `skia_native_glyph_replay, issue_536_native_font_selection, issue_536_raster_diagnostics_contract; Native Skia lib / 실제 PNG·FFI frame`. 최종 전체 10,643건·Native 집중 277건 통과를 아래 실행 기록에 연결한다.
- 독립 기대값과 범위: 첫 공급 경로의 family 소유권·SFNT alias·CSS style matching과 HFT flag를 6c4db5ee7에서 복구했다. shape-group-02는 Native 90.04403→100.00000%, crop·lseg·폼은 무하락이다. 극한 이름의 공통 결과 보정은 별도 d9a0b1c79 및 정식 반례로 기록한다.
- 직전 source `8273ece22860b5793767b9888dd320b3bdc414b8` 대비 원 head의 delta를 확인했다. 누적 PR 전체 diff를 이 단계의 고유 변경량으로 세지 않는다. 원 commit → 체리픽 SHA는 [구현 기록](pr_7892_review_impl.md)에 보존한다.

## 조판 원칙과 입력 확인

공통 구현·통합 해소는 [1단계](../../working/task_m100_536_integration_review_stage1.md),
구조 분리와 예제 설명은 [2단계](../../working/task_m100_536_integration_review_stage2.md),
실제 Print 출처·폼 대응은 [3단계](../../working/task_m100_536_integration_review_stage3.md),
극한 이름의 생산→각 backend 소비·비적용 경계는 [4단계](../../working/task_m100_536_integration_review_stage4.md),
실제 전쪽 결과·정식 수정 전 FAIL/수정 후 PASS는 [5단계](../../working/task_m100_536_integration_review_stage5.md)에 연결한다.

검증 입력 8개는 각각 전체 1쪽이며 Native/fresh WASM 모두 90% 이상, 고정 devel 대비 0.01%p 이상 하락이 없다.
원본·독립 Print PDF·직접 판독과 metadata/합성 계약을 구분한다. Native IME 저장본은 실제 host 조합·undo/redo·저장·재열기 후 한컴 Print와 양쪽 100%를 확인했다. [최종 실행 기록](../../working/task_m100_536_integration_review_stage6.md)에 연결한다. 별도 단독 극한 진단 입력은 90% 미만으로
정상 fixture로 등록하지 않았고 완료로 표시하지 않는다. 새로운 절대 px 핀·baseline 완화는 없다.
쪽 분할·이어받기·저장 LineSeg 수용 규칙을 이 통합의 새 기능으로 변경하지 않는다.
로컬 필수 검사는 완료했다. 전체 nextest 10,643 PASS / 0 FAIL / 45 skipped, Native 집중 277 PASS, Native Skia lib 4,043 PASS이며 format·Native/WASM/workspace lint·workspace build·doc test·실제 feature-off ABI·기존 HFT Native 검사도 통과했다. 원격 exact-head CI는 아직 미실행이다. source SHA·명령·한계는 위 최종 실행 기록에서 구분한다.

## Merge 후 contributor PR comment 계획

1. 통합 PR의 실제 MERGED·merge SHA와 최종 head CI를 API로 확인하고, 이 개별 review·대표 asset이 devel에 존재하는지 먼저 확인한다. 원 PR을 직접 merge한 것으로 표시하지 않고 체리픽 통합 링크와 원 head `6abf22be0cb0fa10f0a5b9dfe57bdd2539e5df4f`를 연결한다.
2. 원 author의 기여에 한국어 존댓말로 감사하고, 원 PR의 고유 주장·소비 경로·메인터너 보정 이유를 위 기록에서 가져온다. 로컬 검증은 code candidate `068769e9d`의 전체 10,643 PASS / 0 FAIL / 45 skipped, 실제 Native/브라우저 및 이후 완료한 필수 gate를 최종 기록에서 인용한다. 미완료 검사·미실행 플랫폼을 통과로 바꾸어 적지 않는다.
3. 직접 비교한 전체 입력은 8개·각 1쪽이며 Native/fresh WASM 모두 90% 이상, 고정 devel 대비 0.01%p 이상 하락·누락·쪽수 차이가 없음을 기록한다. 공통 출력 대조군의 Native/WASM 전체 1쪽 100%와 이 PR 고유 계약 검사 결과를 구분한다. 그림 점수만으로 glyph 계약 전체의 검출력을 주장하지 않는다.
4. 대표 stable 경로는 `mydocs/pr/assets/issue_536_seo_rii_20261017/{native,wasm}/shape-group-02_{review,overlay}.png`다. 실제 merge SHA로 `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/issue_536_seo_rii_20261017/<backend>/shape-group-02_<panel>.png`를 만들어 **Markdown 이미지**로 표시한다. [Visual Sweep 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)을 direct link로 함께 남긴다.
5. 알려진 원래 분수/폼 차이와 미해결 단독 극한 진단을 전체 해결로 표시하지 않는다. 부모 #536은 종료하지 않는다. 실제 결과 본문은 UTF-8 `--body-file`로 게시하고 API로 한글·선두 BOM·본문·merge SHA·asset URL을 재조회한다. 게시 성공 뒤 이 원 PR을 통합 링크와 함께 close하고 contributor fork branch는 보존한다.

## PR 직전 base 갱신

새 base `b7a5388034ee66f58a7e1d5363974e83b6374f57`와 통합 head `a7aac1a9ce5b2b902471ecd8f234f7d72834812d`의 [7단계 재검증](../../working/task_m100_536_integration_review_stage7.md)을 따른다. 원 head와 1–6단계 실행 기록은 그대로 보존한다. TSV 원본은 사용자 지시에 따라 PR 본문에 그대로 넣는다.

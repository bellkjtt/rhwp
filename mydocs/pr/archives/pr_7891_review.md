# PR #7891 — feat(canvaskit): prepare bounded resources and cache page replay (P46)

## 최종 판정

**메인터너 보정 후 수용 가능 — 통합 후보의 로컬 필수 회귀·lint·시각 검증 완료.** 원 contributor head 직접 merge 대상이 아니며, 실제 통합 PR의 exact-head CI와 문서 trailing head CI 확인 전 merge하지 않는다.

## 접수 정보

- [원 PR](https://github.com/edwardkim/rhwp/pull/7891), 기존 contributor `seo-rii`, base `devel`, non-draft.
- 원 head `8273ece22860b5793767b9888dd320b3bdc414b8`, 검토 base `fe5725dc384689b45d60ef8ea4c83138bc2701d8`.
- 접수 시 규모 +11743 / -865 / 90 files는 누적 source diff다. 선행 PR을 중복 적용하지 않는다.
- reviewer `jangster77` 지정. 작성 시 mergeability는 참고값이며 실제 통합 후보와 최신 head를 다시 확인한다.
- [구현 순서](pr_7891_review_impl.md), [전체 계획](../../plans/pr_review_seo_rii_20261017.md).

## 변경·검증 범위

- 구현 주장: producer의 이미지·폰트 증거를 제한된 불변 resource table로 hydration하고, profile·font·표시 옵션이 같은 결과만 캐시한다. 폼은 producer drawing을 재생한다.
- 실제 소비 경로: `core/layer-resource-store·layer-tree-transport → page-renderer → CanvasKit caches·form-replay·font selection`. 아래 검사는 source 단독 CI가 아닌 통합 후보에서 연결한다.
- 관련 검사: `Studio unit / renderer contract / 실제 CanvasKit font coverage·portable geometry·font preparation; form HWP·HWPX 전쪽 Print 대조`. 최종 전체 10,643건·Native 집중 277건 통과를 아래 실행 기록에 연결한다.
- 독립 기대값과 범위: 폼 기준은 기존 사용자 HWPX 수동 Print다. HWP와의 속성·실제 렌더 대응을 별도로 입증했으며 HWP 자체의 새 Print라고 표시하지 않는다. #7688 공급 글꼴 준비 동작을 원 devel 그대로 보존했다.
- 직전 source `79942a3807d4066dd7cb44abda7697c19ffb614e` 대비 원 head의 delta를 확인했다. 누적 PR 전체 diff를 이 단계의 고유 변경량으로 세지 않는다. 원 commit → 체리픽 SHA는 [구현 기록](pr_7891_review_impl.md)에 보존한다.

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

1. 통합 PR의 실제 MERGED·merge SHA와 최종 head CI를 API로 확인하고, 이 개별 review·대표 asset이 devel에 존재하는지 먼저 확인한다. 원 PR을 직접 merge한 것으로 표시하지 않고 체리픽 통합 링크와 원 head `8273ece22860b5793767b9888dd320b3bdc414b8`를 연결한다.
2. 원 author의 기여에 한국어 존댓말로 감사하고, 원 PR의 고유 주장·소비 경로·메인터너 보정 이유를 위 기록에서 가져온다. 로컬 검증은 code candidate `068769e9d`의 전체 10,643 PASS / 0 FAIL / 45 skipped, 실제 Native/브라우저 및 이후 완료한 필수 gate를 최종 기록에서 인용한다. 미완료 검사·미실행 플랫폼을 통과로 바꾸어 적지 않는다.
3. 직접 비교한 전체 입력은 8개·각 1쪽이며 Native/fresh WASM 모두 90% 이상, 고정 devel 대비 0.01%p 이상 하락·누락·쪽수 차이가 없음을 기록한다. 기준 PDF가 HWPX 사용자 수동 Print이며 HWP 대응은 별도 속성·렌더 해시로 확인한 것이라고 명시한다. 기존 Edit bevel 차이는 새 해결 성과로 기록하지 않는다.
4. 대표 stable 경로는 `mydocs/pr/assets/issue_536_seo_rii_20261017/{native,wasm}/form-hwp-manual-crossformat_{review,overlay}.png`다. 실제 merge SHA로 `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/issue_536_seo_rii_20261017/<backend>/form-hwp-manual-crossformat_<panel>.png`를 만들어 **Markdown 이미지**로 표시한다. [Visual Sweep 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment)을 direct link로 함께 남긴다.
5. 알려진 원래 분수/폼 차이와 미해결 단독 극한 진단을 전체 해결로 표시하지 않는다. 부모 #536은 종료하지 않는다. 실제 결과 본문은 UTF-8 `--body-file`로 게시하고 API로 한글·선두 BOM·본문·merge SHA·asset URL을 재조회한다. 게시 성공 뒤 이 원 PR을 통합 링크와 함께 close하고 contributor fork branch는 보존한다.

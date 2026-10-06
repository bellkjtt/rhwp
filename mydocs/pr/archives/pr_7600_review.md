---
kind: review
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-06
---

# PR #7600 리뷰 — dyad 붙여쓰기와 양쪽 화살표

## 최종 판정

**머지 보류 — 원래 실패 입력의 시각 증거와 기준 PDF의 Print 출력 근거가 부족합니다.**

새로 실행한 최소 입력 Native 1쪽은 96.46055%, 집중 회귀 3개는 PASS입니다. 이 결과를 원래 실패 입력의 개선이나 전체 수용으로 확대하지 않습니다. 원 PR head의 CI 성공과 최신 devel 체리픽의 로컬 결과를 구분합니다.

보류 해제 조건: 79.8%·64.3%로 보고한 원본/실패 입력·대응 Print PDF·실패 결과를 보존하고, 문서/영역 제거로 대체하지 않은 해당 범위의 출력을 개선해 새 head에서 90% gate와 직접 판독을 충족해야 합니다. 기존 기준 PDF의 실제 Print 출력 방법/설정도 확인해야 합니다. 이후 통합 후보의 fresh WASM·영향 경계·최신 required CI를 확인하며 실제 원격 push·PR 생성·review 게시·merge는 승인 범위에 따라 수행합니다.

## 접수 정보

| 항목 | 값 |
| --- | --- |
| 원 PR·작성자 | [#7600](https://github.com/edwardkim/rhwp/pull/7600) / Phantomn / 첫 기여자 |
| 관련 이슈 | [#7593](https://github.com/edwardkim/rhwp/issues/7593), 원 PR은 closes; [#7594](https://github.com/edwardkim/rhwp/issues/7594)는 별도 글리프 문제 |
| 원 head·fork | `dbb87b98047ffb73ff038374a59501495a47bdf2` / Phantomn/rhwp / fix/issue-7593 |
| 검토 base | `a048da32d2b76867283aa6dbdf2578816a0e6238` / upstream/devel |
| 검토 branch·체리픽 | `review/phantomn-pr7600-20261006` / `5d7a2ad98` (원 저자 보존, 충돌 없음) |
| 로컬 기록 보정 | `08abab8717e228b5c593aa24e94b03a9c35b1424`: 원시 TSV 3개 Git 제외; source/test 변경 없음 |
| reviewer·라우팅 | jangster77 지정 확인 / collaborator_external_pr + first_time_contributor + intake_and_review + local_validation + visual_fixture_evidence |
| 작성 시점 상태 | 원 PR Open, MERGEABLE/CLEAN, exact head required CI 성공; merge 전 재확인 필요 |

PR 규모는 원 head 18개 파일·121줄 추가/6줄 삭제입니다. 부모 workflow·선택표·위 자식 절차와 review_template을 읽었습니다. 주 작업공간의 clean devel에서 최신 base 위에 단일 기능 commit을 누적 적용했습니다. 별도 worktree와 원격 통합 PR은 만들지 않았습니다.

## 발견 사항

### P1 — 실패 입력을 제거한 대조군만으로 원본의 시각 조건을 충족할 수 없음

[원 PR 본문](https://github.com/edwardkim/rhwp/pull/7600)의 「남은 차이 / 최소 재현을 쓴 이유」는 전체 문항 79.8%, 첫 최소 입력 64.3%, 마스터쪽 단 구분선을 끈 재생성 입력 96.21277%를 기록합니다. 현재 PR에는 마지막 입력과 그 PDF만 있고 두 실패 입력·기준 PDF·실패 TSV/PNG가 없습니다. 이 값들은 작성자의 보고이며 이번 검토에서 재현한 실패로 기록하지 않습니다.

실패 범위를 기존 차이 또는 별개 결함으로 분류했다는 이유만으로 gate를 면제할 수 없습니다. `dyad` 자체의 독립 최소 재현은 유용하지만, 삭제한 구분선과 실제 문항의 미달 원인이 해결됐다는 증거는 아닙니다. 실패 입력의 보존·전후 대조와 실제 출력 개선이 필요합니다. 자료가 없으므로 본 검토도 그 원인이나 최종 일치율을 추정하지 않았습니다.

### P2 — 한컴 PDF의 Print 인쇄 계약 미확인

기준 PDF의 Creator는 Hwp 2022 12.0.0.535, Producer는 Hancom PDF 1.3.0.545, PDF 1.6, 정상 1쪽입니다. 본문은 한컴에서 출력했다고 적지만 Print 메뉴·원본 용지·100%·1-up 등 방법/설정 기록이 없습니다. Creator/Producer만으로 인쇄와 저장/내보내기를 구분할 수 없어 Print 계약은 미검증입니다. PDF 버전·한컴 연도 때문에 거부하거나 저장/내보내기로 생성됐다고 단정하지 않습니다. 기존 파일이 실제 Print였음을 확인하면 재출력 없이 재사용할 수 있습니다.

### P2 — 원시 TSV Git 포함 (로컬 보정 완료)

원 head는 `mydocs/pr/assets/issue_7593_dyad_arrow/`의 `silhouette-before-devel.tsv`, `silhouette-native.tsv`, `silhouette-wasm.tsv`를 커밋했습니다. 현행 지침은 원시 TSV를 ignored output에 두고 검증 범위 전체 Native TSV만 PR 첨부/다운로드 링크로 제공합니다. 세 파일은 `output/pr-review/pr7600/contributor-tsv/`에 보존하고 로컬 보정 commit에서만 Git 제외했습니다. 원 contributor branch에는 push하지 않았고 그 head는 바꾸지 않았습니다.

## 구현 주장과 검토 경로

| 주장·항목 | 실제 경로·근거 | 판정 |
| --- | --- | --- |
| glued dyad 토큰 분리 | tokenizer.rs `read_command` → `longest_keyword_prefix` → GLUE_SAFE에 dyad 추가; 단독·공백형과 붙여쓰기 3종 집중 검사 | 충족 |
| 양 끝 화살촉 | layout.rs `layout_decoration`의 body.width → SVG/Canvas2D/Skia Decoration → draw_decoration, Studio drawEquationDecoration. Vec 가로선/오른쪽 촉을 유지하고 Dyad에만 좌측 대칭 촉 추가 | 코드 확인 충족; 실행은 SVG 집중 검사와 Native Sweep 범위 |
| 측정·배치 규칙 | 장식의 측정/배치 구조·저장 LineSeg·줄/표 분할·요구 높이·clamp 변경 없음 | 해당 수치 변경 비해당 |
| 실제 반례 | 공백/단독/붙여쓰기, Vec의 선과 오른쪽 화살촉 대조, Native 4행/5장식 직접 판독 | 최소 입력 범위 충족 |
| 원본 사례와 증거 독립성 | 제출 최소 입력은 한컴 저장본/한컴 PDF. 앞선 실패 입력과 PDF는 미제공 | 원본 범위 미검증 |
| 기준값·예외 | baseline/golden/임계값 변경 없음, 글꼴 예외 미사용 | 기준값 변경 비해당 |
| 검증 주장 범위 | 원 PR Full CI와 통합 후보 집중/Native 검증을 구분. 통합 후보 fresh WASM·실제 Studio 정상 경로 미실행 | 제한 범위만 충족, 나머지 미검증 |

네 렌더 경로의 새 좌측 촉은 기존 Vec과 같은 fs 비율과 대상 폭을 사용합니다. 수정 코드에서 문서 ID 예외나 출력 은폐는 발견하지 않았습니다. SVG 테스트의 동일성 비교는 두 동등한 입력의 결과를 대조하며 특정 문서의 절대 위치/golden을 고정하지 않습니다. 새 테스트의 원본 관련 90% 선행 조건은 위 부족 자료로 확인되지 않았습니다.

작성자가 제시한 수정 전 3 FAIL은 직접 재실행하지 않았습니다. 당시 base와 최신 검토 base 사이의 equation 디렉터리 및 skia/equation_conv.rs diff가 비어 있어 해당 코드의 변화를 확인했지만, 이 확인을 실제 red 실행으로 기록하지 않습니다. 작성자 CanvasKit 일회용 점검에서 ① 줄이 안 보인다고 한 경로는 정상 제품 분기를 우회한 점검이라 새 회귀로 단정하지 않습니다.

## 검증 입력과 결과

수용 판정에 사용할 최소 입력과 PDF는 원 PR head와 체리픽 commit에 바이트 동일하게 포함돼 있습니다.

| 파일·역할 | SHA-256 |
| --- | --- |
| `mydocs/pr/assets/issue_7593_dyad_arrow/repro-dyad-min.hwpx` / 한컴 저장 최소 입력 | `c8605b6d47949a402de1955df3dd00a3dc2ae56dbab822d12b512fc108d4198d` |
| 같은 폴더 `repro-dyad-min.hancom.pdf` / 대응 한컴 PDF (Print 경로 미확인) | `103c6ffb34e64bad8992ce6b6284faa7d00a9d82095757ff0ad335ca4ccbe838` |

이슈 첨부 `repro-equation.hwpx`도 fork의 issue-assets ref `8e49f1b8e542f3514f0ab7029a3e7164942b671e`에서 탐색했습니다(SHA-256 `88a3683c34e92f8887e54b9b379f8c399a8ec844e46e127793c6486cba8d31ec`). 제출 최소 입력과 다른 문서이며 대응 PDF가 없어 시각 수용 근거로 사용하지 않았습니다. ignored output에만 보존했고, 79.8%·64.3% 입력과 같다고 추정하지 않았습니다.

- `node scripts/rust-test-suite-manifest.mjs --prepare` 후 `CARGO_TARGET_DIR=target/pr-review node scripts/run-rust-test.mjs issue_7593_eq_dyad_glue_and_arrow -- --cargo-profile release-test --target-dir target/pr-review`: **3 PASS / 0 FAIL** (필터 제외 234). 로그 `output/pr-review/pr7600/logs/focused.log`.
- `git diff --check upstream/devel...HEAD`, `git diff --check`, `git merge-tree --write-tree upstream/devel HEAD`: 충돌·공백 없음.
- 새 Native CLI SHA-256 `cc1d9c1f9219f3933ab1c45a5dbc323320390aaaf49ad0b762436bf31b4a161d`.
- 시스템 fontconfig의 한컴산뜻돋움·함초롬바탕·한컴바탕·Times New Roman 파일을 local font 공급 경로로 확인했습니다. `RHWP_FONT_PATH`는 별도 지정하지 않았습니다. 한컴2020 시스템 글꼴과 PDF2022의 수식 HYhwpEQ는 동일 face/빌드라고 주장하지 않으며 실제 공급 정보는 `output/pr-review/pr7600/font-supply.json`에 있습니다.
- Native 명령: `python3 scripts/visual_sweep.py --hwp mydocs/pr/assets/issue_7593_dyad_arrow/repro-dyad-min.hwpx --pdf mydocs/pr/assets/issue_7593_dyad_arrow/repro-dyad-min.hancom.pdf --key dyad-min --rhwp-bin target/pr-review/release-test/rhwp --pages 1 --dpi 96 --embed-fonts=full --font-path output/pr-review/pr7600/fonts --out output/pr-review/pr7600/native-review`: exit 0, p1 gate passed, flagged 0, 원문/PDF 전체 1쪽, 누락 0.
- TSV: 새 재출력의 rhwp_png/pdf_png에 `python3 scripts/visual_sweep.py --silhouette-only --png-pair output/pr-review/pr7600/native-review/dyad-min/rhwp_png output/pr-review/pr7600/native-review/dyad-min/pdf_png --out output/pr-review/pr7600/native-scores`: exit 0, p1 **96.46055%**, raw 동일, boundary 조정 0, below90/누락/측정 불가 0. 실루엣-only 판정 자체는 not_evaluated입니다.
- exact 원 head의 [Full CI](https://github.com/edwardkim/rhwp/actions/runs/37317981931), [Render Diff](https://github.com/edwardkim/rhwp/actions/runs/37317981119) 및 CodeQL/Adapter/Proptest/CI Impact Policy 성공을 확인했습니다. source/test 추가 보정이 없어 광범위 로컬 전체 회귀를 중복 실행하지 않았습니다. 이는 통합 PR의 최신 CI를 대신하지 않습니다.
- 원본/Print 증거 blocker가 확정돼 비용 큰 fresh WASM 재빌드·Studio 실행은 이번 검토에서 추가하지 않았습니다. 원 PR의 fresh WASM 96.21277%는 작성자 증거이며 통합 후보의 새 WASM 실행으로 기록하지 않습니다.

## 직접 시각 판독과 증적

[새 Native review](../assets/pr_7600_review_20261006/native-review.png), [standalone overlay](../assets/pr_7600_review_20261006/native-overlay.png)를 열고 해당 4행 영역을 확대해 대조했습니다. 붙여쓰기 결과 AB 표시, AB/AC/BD 위 대상 폭의 양 끝 화살촉 5개, 4행의 내용 보존을 확인했습니다. 화살촉 크기·수식 글자 모양/세로 원점·BOT 주변 간격 차이는 남습니다. 96.46055%는 실루엣 보조값이고 엄격 잉크 일치율은 8.82729%이므로 전체 글리프 모양의 일치를 뜻하지 않습니다. 원본 입력의 미달을 이 대조군 점수로 면제하지 않습니다.

## Merge 후 contributor PR comment 계획

현재 보류 상태이므로 merge 완료·시각 승인 코멘트는 게시하지 않습니다. 자료 보완 뒤 수용 가능한 head가 확정되면 [Visual Sweep 정본](../../manual/verification/visual_sweep_guide.md#github-merge-comment), 검증한 페이지/지표/PNG와 merge SHA 고정 URL을 기록하고 사용자 승인 범위에서 게시합니다. 첫 기여 안내와 감사는 한국어 존댓말로 작성하며, 원 기여의 수식 개선과 메인터너 기록 보정을 구분합니다.

## 사용자 승인 후 실제 원본 경계 재현

사용자가 메인터너 보정을 지시했고 다른 과거 문서는 없다고 확인했습니다. 공개 이슈 입력의 masterpage0.xml에는 colPr/colLine이 남아 있습니다. 이 입력의 MCP engine2020 Print(job `c0af74c9-1bcf-454d-b177-08ab91b24e59`)은 Hancom11.0.0.9136·PrintMethod0·1-up 실행 근거를 반환했습니다. 같은 입력의 새 Native Sweep은79.58463%/exit1이고 단 구분선 누락을 직접 판독했습니다. 이 실제 실행으로 해당 경계의 검증을 진행하며 과거79.8%/64.3%와 같은 파일이라고 주장하지 않습니다. [실행 기록](pr_7600_review_impl.md)에 보정 계약·검증 순서를 연결했습니다. 원시 TSV는 이미 local output 분리 보정했습니다.

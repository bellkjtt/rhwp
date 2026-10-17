# PR #7978 검토 — HF/FN Unicode scalar 입력·undo/redo

## 최종 판정

**승인 — #7977의 원 변경을 최신 devel에 통합한 후보에서 로컬 검증 완료.**
검증한 통합 code 후보는 `a7c9e30ffebc843d5f5763f349f79ad3ea93cfb1`다. 통합 PR의 원격 code CI·문서 trailing·최종 CI 및
사용자 원격 게시 승인은 남은 통합 조건이다. 원 PR의 녹색 CI와 동일시하지 않는다.

## 접수 정보

- 원 PR: https://github.com/edwardkim/rhwp/pull/7978 · 작성자 `donggyun112`(기존 기여자).
- 관련 이슈: https://github.com/edwardkim/rhwp/issues/7977 · 원 PR의 Closes 범위.
- 원 head: `5558cbcfb8e0d2cb30422e12aa719288371164a5` · 누적 적용 commit: `2d7c0bcf0f5d8be31bf900d350bb6fb63ba0bdfd`.
- 기준 devel: `f9d2ab17ab438c6bed97d91c3c2a98a5be27e573` · 메인터너 보정: `a7c9e30ffebc843d5f5763f349f79ad3ea93cfb1`(아래 개별 범위 구분).
- 리뷰어: `jangster77` 선행 요청 후 API 재조회 확인. 원 PR base devel, non-draft,
  MERGEABLE/CLEAN 및 최신 원 head check 실패 없음은 조사 시점 상태다.
- 원 PR [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/39027342904/job/117383140929)는 참고이며 통합 후보 검증을 대체하지 않는다.
- [적용 기록](pr_7978_review_impl.md) · [누적 계획](../../plans/pr_review_donggyun112_20261017.md).

## 개별 검토와 실제 소비 경로

일반 입력의 `onInput → insertTextInHeaderFooter/Footnote → record command → HF/FN cursor`와
`InsertTextInHeaderFooterCommand/InsertTextInFootnoteCommand`의 constructor·execute/redo 문맥을
대조했다. 코어 문단 위치가 Unicode scalar 단위라는 기존 계약에 맞춰 여섯 곳이 `charCount`를
소비한다. undo의 삭제 길이와 HF/FN 소유 문단·대상을 유지한다.

제품 보정은 이 원 PR의 scalar 변경에 추가하지 않았다. 동일 통합에 포함된 #7985의 빈 누름틀
사전 거절 보정은 별도 커밋이며 본 PR의 기여로 계산하지 않는다. `MANIFEST.md` 충돌은 두 PR의
항목을 모두 유지했다. 단위 대역에서는 3모드 × 4문자열(뒤 B 보존·반복 왕복·문맥)을 확인했고,
실제 브라우저에서는 3모드 × 3문자(가/😀/𠮷)의 실제 input·Ctrl+Z·Ctrl+Shift+Z를 확인했다.
음성 대조는 입력 후 커서 두 곳만 UTF-16 길이로 되돌렸을 때 assertion으로 실패했다.

## 공통 로컬 검증

| 검사 | 실제 결과 |
| --- | --- |
| `npm --prefix rhwp-studio ci --no-audit --no-fund` | exit 0 |
| `npx --prefix rhwp-studio tsc --noEmit -p rhwp-studio/tsconfig.json` | exit 0; 최종 head에서 재확인 |
| `npm --prefix rhwp-studio test` | 2,320건 중 2,318 PASS / 0 FAIL / 2 SKIP |
| `npm --prefix rhwp-studio run build` | exit 0; 기존 큰 chunk 경고 유지 |
| HF/FN·교차 구역 집중 단위 검사 | 2 PASS / 0 FAIL / 0 SKIP |
| `python3 scripts/check_e2e_manifest.py` | 파일 158 / 행 158, 누락 0 |
| fresh WASM wrapper | 공용 target, jobs 8, `--target web --out-dir pkg --no-opt`, exit 0 |
| 실제 Chrome 교차 구역 E2E | 8개 입력 진입점, 비어 있지 않은 undo/redo, 실제 빈 누름틀, 정상 대조군 PASS |
| 실제 Chrome undo E2E | 기존 편집 undo 계약 + HF/FN 9사례 입력·실키 undo/redo PASS |
| 음성 대조 | HF/FN 커서 길이와 command 사전 거절을 각각 메모리에서 되돌리면 assertion FAIL |

단위 검사 SKIP 2건은 기존 `pkg-node` 전용 혼합 모양/서식 왕복 검사다. 이번 신규 검사는 모두
실행했다. Node v24.15.0, npm 11.12.1, Rust 1.93.1, Linux x86_64,
Chrome for Testing 154.0.8037.57에서 실행했다. 실제 OS 클립보드·마우스 드래그·IME 후보창은
미검증이며 DOM clipboard/composition 이벤트와 실제 키보드 입력을 구분한다.
Docker 최적화 배포 검증이 아닌 host release `--no-opt`의 fresh WASM UI 검증이다.

검증 중 `pkg/`·Studio public의 JS/WASM 바이트 일치를 확인했다. 검증 후 tracked 생성 glue
`rhwp-studio/public/rhwp.js`는 검토 source에 섞이지 않도록 HEAD로 복원했다.
공유 `target/pr-review`는 보존했다.

## 조판 원칙 준수 검토

| 항목 | 근거 | 판정 |
| --- | --- | --- |
| 구현 근거와 일반성 | 아래 개별 계약과 실제 호출 경로; 문서 ID 예외·픽셀 보정 없음 | 충족 |
| 측정·배치 일관성 | 줄 메트릭·치수·레이아웃 계산을 변경하지 않음 | 비해당 |
| 최종 출력 소비 계약 | 편집기의 실제 WASM·브라우저 경로로 내용·커서·히스토리 검사 | 충족(편집 계약); 출력 피델리티 주장은 없음 |
| 분할·이어받기 | pagination·rowspan·continuation 변경 없음 | 비해당 |
| 줄 소속과 점유 높이 | 저장 LineSeg·높이·폭·글꼴 변경 없음 | 비해당 |
| 사례와 독립성 | scalar 위치 및 요청하지 않은 변경 금지 계약; 테스트 대역과 실제 WASM 구분 | 충족 |
| 기준값 변경 | golden·baseline·허용치 완화 없음 | 비해당 |
| 시각 회귀 추가 선행 조건 | 렌더링 위치/모양 회귀를 추가하지 않음; 편집 내용·순서·거절 검사만 추가 | 비해당 |
| 수식·간격 확대 판독 | 수식·paint·조판 치수 수정 및 피델리티 개선 주장 없음 | 비해당 |
| 주장과 증거 | 개별 검사 및 위 실제 실행 결과; 원 PR CI를 통합 head CI로 대용하지 않음 | 충족(로컬 검증 범위) |

Native/Print PDF Visual Sweep은 비해당이다. 바뀐 값은 HF/FN 편집기의 문자 위치와
지원하지 않는 선택 요청의 허용 여부다. 모델·serializer의 속성/저장 조판 정보·줄 메트릭·paint를
변경하지 않았으며, 일반 요청의 조판 소비 규칙도 바꾸지 않는다. 새로운 텍스트 내용이 화면에
반영되는 사실을 조판 알고리즘·한컴 출력 피델리티 개선으로 주장하지 않는다.

## Merge 후 contributor PR comment 계획

원 PR을 직접 merge한 것으로 표시하지 않는다. 통합 PR의 실제 MERGED·merge SHA,
최종 code/trailing CI와 devel의 개별 review·적용 기록 존재를 확인한 뒤 한국어 존댓말로
감사와 수용 범위를 게시한다. 해당 PR의 고유 검증·메인터너 보정 여부와 남은 미검증을
분리하고 통합 PR·개별 기록·실제 CI URL을 direct link로 연결한다. 게시 뒤 UTF-8 원문·선두
BOM·URL을 API로 확인하고 원 PR을 close한다. contributor fork는 보존한다.
시각 피델리티 수치나 미실행 Native/Print·OS IME 검사를 통과로 적지 않는다.

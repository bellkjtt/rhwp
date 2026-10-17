# PR #7985 검토 — 교차 구역 선택 편집 사전 거절

## 최종 판정

**메인터너 보정 후 수용 가능 — #7983 해결 범위의 로컬 검증 완료.**
검증한 통합 code 후보는 `a7c9e30ffebc843d5f5763f349f79ad3ea93cfb1`다. 통합 PR의 원격 code CI·문서 trailing·최종 CI 및
사용자 원격 게시 승인은 남은 통합 조건이다. 원 PR의 녹색 CI와 동일시하지 않는다.

## 접수 정보

- 원 PR: https://github.com/edwardkim/rhwp/pull/7985 · 작성자 `donggyun112`(기존 기여자).
- 관련 이슈: https://github.com/edwardkim/rhwp/issues/7983 · 원 PR의 Closes 범위.
- 원 head: `5dd0c86ac1a86c977e9d80a7fe263ac60358f720` · 누적 적용 commit: `3930b9cdbe8c8ec706e9dcb48564cfb17f43c63c`.
- 기준 devel: `f9d2ab17ab438c6bed97d91c3c2a98a5be27e573` · 메인터너 보정: `a7c9e30ffebc843d5f5763f349f79ad3ea93cfb1`(아래 개별 범위 구분).
- 리뷰어: `jangster77` 선행 요청 후 API 재조회 확인. 원 PR base devel, non-draft,
  MERGEABLE/CLEAN 및 최신 원 head check 실패 없음은 조사 시점 상태다.
- 원 PR [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/39033606255/job/117400939604)는 참고이며 통합 후보 검증을 대체하지 않는다.
- [적용 기록](pr_7985_review_impl.md) · [누적 계획](../../plans/pr_review_donggyun112_20261017.md).

## 개별 검토와 실제 소비 경로

코어 `deleteRange`는 끝 구역을 받지 않는다. `DeleteSelectionCommand` 생성 시 사전 거절하고
`canDeleteSelectionInFormMode → 키/일반 입력/IME/cut/paste → deleteSelection`을 대조했다.
구역 병합이나 실제 교차 구역 삭제 지원을 추가한 변경은 아니다. 최소 수정은 지원하지 않는
요청이 모델·선택·히스토리를 변경하지 않는다는 이슈의 명시적 계약을 충족한다.

원 head에는 누락 경계가 있었다. `onInput`이 거절 검사 전에
`prepareClickHereInputPosition`을 호출하면 실제 helper가 `clearSelection`·`setActiveField`를
수행한다. 그 뒤 `hasSelection`이 false가 되어 교차 구역 거절을 우회한다. 실제 helper를 소비하는
검사에서 선택이 null로 바뀌는 FAIL을 재현했고, 허용 검사를 앞당겨 PASS로 바꿨다. 정상적인
선택 없는 누름틀 입력은 계속 정규화·삽입되는 대조군을 유지했다.

실제 브라우저에서도 빈 누름틀에 끝나는 교차 구역 선택과 입력을 실행해 내용·선택·기존
undo/redo 보존을 확인했다. 8개 입력 진입점의 undo/redo 대조군을 양쪽 스택이 비어 있지 않게
강화했고, IME는 시작·input·종료 DOM 이벤트를 연속 실행했다. 같은 구역 삭제+undo와 양식
Tab/Shift+Tab 탐색도 통과했다. 기존 command 거절을 제거한 별도 메모리 음성 대조는 실패했다.

현재 base에서 누락돼 있던 기존 `middle-dot-canvas-paint.test.mjs`의 manifest 행도 등록했다.
이 보조 보정은 기존 테스트의 등록 누락 정정이며 해당 문서의 조판 성능/정확성을 새로 검증한
성과로 표시하지 않는다.

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

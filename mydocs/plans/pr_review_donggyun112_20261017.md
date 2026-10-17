---
kind: plan
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-17
---

# donggyun112 열린 PR 2개 누적 검토 계획

## 승인 범위와 시작점

사용자 요청: 열린 PR을 체리픽 통합한 뒤 개별 검토하고 필요하면 메인터너 보정한다.
리뷰어 선행 지정 요청에 따라 두 PR 모두 `jangster77` 요청을 게시하고 API로 확인했다.
기본 작업공간이 clean하므로 별도 worktree 없이 `review/donggyun112-20261017`에서 진행한다.
기준 devel은 `f9d2ab17ab438c6bed97d91c3c2a98a5be27e573`이다.

| 원 PR | 원 head | 검토 범위 |
| --- | --- | --- |
| [#7978](https://github.com/edwardkim/rhwp/pull/7978) | `5558cbcfb8e0d2cb30422e12aa719288371164a5` | [#7977](https://github.com/edwardkim/rhwp/issues/7977): HF/FN 문자 위치의 Unicode scalar 계약 |
| [#7985](https://github.com/edwardkim/rhwp/pull/7985) | `5dd0c86ac1a86c977e9d80a7fe263ac60358f720` | [#7983](https://github.com/edwardkim/rhwp/issues/7983): 단일 구역 코어 계약 밖의 선택 변경 사전 거절 |

원 기여자는 기존 기여자로, 첫 기여자 절차를 새로 적용하지 않는다.
원 author와 `cherry-pick -x` 연결을 보존하며 fork source를 수정하지 않는다.
원 PR CI와 통합 후보 로컬 검증을 구분한다. PR 생성·push·merge는 실제 승인 범위에 따라 처리한다.

## 독립 계약과 소비 경로

- #7977: 코어 문단 편집 위치는 Unicode scalar 단위다. `charCount` 결과가 일반 입력의
  HF/FN 커서와 입력 command의 초기/execute/redo 문맥으로 전달되는지 확인한다.
  후속 글자·문단 끝·중간 삽입, undo/redo 반복, 한글·이모지·보조 평면 한자를 대조한다.
- #7983: 코어 `deleteRange`가 받지 않는 끝 구역을 누락한 채 다른 내용을 지우면 안 된다.
  `canDeleteSelectionInFormMode`의 판정부터 실제 키·일반 입력·IME·cut/paste와
  `DeleteSelectionCommand` 생성까지 추적한다. 거절 때 모델·선택·히스토리 보존,
  같은 구역 편집과 양식 Tab 탐색의 정상 대조군을 확인한다.
- 모델·저장 조판 정보·치수·paint의 변경 여부를 실제 diff와 소비 경로에서 판정한다.
  편집기 위치/사전 거절만 바꾸는 범위는 출력 피델리티 개선으로 주장하지 않는다.
  조판 영향 보정이 필요해지면 Native/fresh WASM·독립 Print 시각 검증 경로를 추가한다.

## 실행 순서와 완료 조건

1. 위 순서로 원 기능 커밋을 누적 적용하고 충돌·추가 변경을 기록한다.
2. 원본 단위 검사의 수정 전 FAIL/후 PASS를 실제 구현의 격리된 메모리 변이로 확인한다.
3. 변경 범위의 타입 검사·전체 Studio 단위·production build·E2E manifest 검사를 수행한다.
4. 공용 `target/pr-review`에서 fresh WASM을 준비하고 실제 Chrome의 해당 E2E를 수행한다.
   Cargo 빌드를 겹치지 않으며 배포 파일을 fresh source 증거로 대용하지 않는다.
5. 발견한 결함은 원 기여와 별도 메인터너 커밋으로 보정하고 영향 검사를 재수행한다.
6. 원 PR별 archive review·적용 기록에 실제 source SHA·명령·결과·한계를 기록한다.
   통합 PR을 만들 때는 최신 base를 확인하고 실제 code CI 이후 같은 PR의 문서 trailing으로
   개별 검토와 오늘할일을 완료한다.

실행 로그와 원본 API 응답은 ignored `output/pr-review/donggyun112-20261017/`에 기록한다.
필수 증거가 남으면 완료·승인으로 표시하지 않는다.

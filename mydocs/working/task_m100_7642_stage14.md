# 이슈 #7642 — 최신 devel 통합과 시각·회귀 검증

이전 단계: [곱셈 잉크 보정](task_m100_7642_stage13.md).
현재 작업 브랜치는 `fix/equation-font-metrics-pagination-20261007`이다.
사용자가 승인한 최신 `upstream/devel` 위 rebase를 진행한다.

## 통합 기준과 보존

fetch한 기준은 `74f9b71b15a2db18b17c37ceab6cf2b3c20c03c3`이다.
통합 전 로컬 head와 변경 내역은 백업 ref와 ignored 실행 기록으로 보존한다.
이 브랜치의 기여자 체리픽·보정·개별 review 기록을 유지하며, 충돌은 양쪽 변경의 의미를
확인해 해결한다. 공유 Cargo target은 초기화하지 않는다. 실행 중인 시각 검증은 고정 바이너리를
사용하므로 이전 source의 대조 자료로 구분하고 통합 후 head의 증거로 승격하지 않는다.

## 검증과 후속 처리

통합 후 source에서 Native/fresh WASM과 영향 영역 확대 판독을 새로 실행한다.
2023년 9월 원본과 HWPX의 미주 내용 이월·하단 넘침, 2024년 구분선아래20 원본의
22쪽/Print 23쪽 불일치는 실제 미충족으로 추적한다. 쪽수·내용 소속 차이를 글꼴 예외나
페이지 점수로 면제하지 않는다. 모든 승인 범위의 원본을 90% 이상으로 보정·검증한 뒤
필요한 회귀 검사만 관계 검사로 변경하고 전체 회귀·필수 lint를 수행한다.

새 PR 생성 후 정확한 code head CI를 확인하고 review·오늘할일 trailing commit,
trailing CI, merge와 정식 후속 처리를 진행한다. 현재 새 PR·최종 통과 판정은 없다.

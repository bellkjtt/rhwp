---
kind: plan
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-10-06
---

# PR #7600 메인터너 보정 실행 기록

## 범위와 독립 근거

사용자가 메인터너 보정을 승인했습니다. 별도 과거 79.8%/64.3% 자료는 없다고 확인했습니다. 공개 #7593 원본은 확보했고 MCP engine2020의 Print PDF(job `c0af74c9-1bcf-454d-b177-08ab91b24e59`, Hancom11.0.0.9136, pdf_print_method0, hancom2020_pdf_driver_one_up)와 새 Native 전체1쪽에서79.58463%를 재현했습니다. 이 원본을 삭제/축소하지 않고 같은 독립 Print 기준으로 보정합니다. 미제공 문서와 같은 파일이라고 추정하지 않습니다.

## 단계

1. 원 기여 체리픽 `5d7a2ad98`, 원시 TSV local output 분리 `08abab871`와 초기 리뷰를 보존합니다.
2. 공개 원본/Print PDF를 커밋하고 마스터쪽 colPr 파싱 → MasterPage.paragraphs의 ColumnDef → build_master_page의 실제 paint 소비 경로를 고칩니다. 본문 단 구분선의 기존 column 영역 계산·선 스타일 결과를 재사용하고 바탕쪽 프레임 전체 높이를 적용합니다. 문서 전용 좌표·문서 ID·임계값 예외는 추가하지 않습니다.
3. 공개 원본을 먼저 재출력해90%와 내용/경계 직접 판독을 확인합니다. 새 회귀는 독립 Print·관계 기반이고 그 선행 조건을 통과한 뒤만 추가합니다. 기존 원 기여 테스트는 삭제하지 않습니다.
4. PR 최소 입력은 MCP가 기본 빈 OnDocument_New 스크립트 때문에 무인 변환을 거절했습니다. 매크로 실행·보안 설정 변경은 하지 않습니다. 기본 빈 템플릿을 정적으로 확인해 스크립트 없는 별도 검증 사본을 만들 경우 변경 entry를 공개하고 모든 조판/본문 XML의 동일성 및 Native 출력 동일성을 입증합니다. 원본은 보존합니다.
5. Native/fresh WASM과 바탕쪽 관련 대조군·정식 회귀 red/green, fmt/native/WASM32/workspace Clippy·base 정책·필요 전체 회귀를 순차 검증합니다. 로그/TSV/중간 JSON은 output에만 둡니다. source 변경 후 이전 캡처를 완료 근거로 쓰지 않습니다.
6. 결과를 개별 review와 오늘할일에 확정하고 원 PR 기여/보정 source와 실제 검증 head를 구분합니다. 원격 PR 생성·push·merge는 사용자의 해당 단계 지시에 따릅니다.

## 적용 계약과 반례

마스터쪽 단 구분선은 배경이지만 보이는 개체이며 본문 흐름을 전진시키지 않습니다. masterPage의 텍스트 프레임과 그 안 colPr 정의에 따라 위치/높이를 계산하고 별도 본문 단 설정으로 바꾸지 않습니다. 1단/구분선 없음에는 선을 추가하지 않으며 가변 너비·여백·여러 단의 위치도 기존 column 계산 계약을 재사용합니다. 본문 쪽수·다음 내용·번호와 다른 바탕쪽 개체 보존을 확인합니다. 분할/이어받기 규칙 변경은 없습니다.

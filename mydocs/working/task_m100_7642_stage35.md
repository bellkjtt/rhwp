# 이슈 #7642 — 복원 코드의 전쪽 대조와 최초 배치 차이 추적

시작 커밋: `5cb859903`. 생산 Rust 코드는 `e08eae718`과 동일하다.
이번 작업의 모든 회귀 실패는 현재 브랜치에서 보정하며 검사·원본을 이관하지 않는다.

## 검증 대상과 고정 근거

원본 `samples/3-11월_실전_통합_2022.hwpx`와 이미 커밋한
`pdf/issue7642-failed-regression-print/3-11월_실전_통합_2022-hwpx-print-2024.pdf`를
2024 조판으로 전쪽 대조한다. 15쪽만 95.75481%가 된 것을 전체 문서 통과로
보고하지 않는다. Native 바이너리는 Stage33의 SHA-256
`03507a186ea558abb586d207b5ef744339e2c688ee4be8dfdd3e61f9d16acc85`이다.
실행 전 `git diff e08eae718 -- src`가 비고 frozen build의 모든 기록 소스 해시가
현재 파일과 일치함을 확인했다. 실패한 Stage34 바이너리를 사용하지 않는다.

전쪽 TSV는 ignored `output/pr-review/issue7642/failed-inputs-20261008/`
`stage15-current-validation/endnote-stable-generation-native-all/`에 저장한다.
정확한 명령·소스 대응은 `endnote-stable-generation-native-all-provenance.json`,
실행 로그는 같은 접두어의 `.log`에 남긴다. timeout은 7200초로 지정했다.
MCP 재출력이나 PDF 복제 없이 원본에 대응하는 기존 Print를 사용한다.

## 다음 순서

1. 전쪽의 최저값·미달·누락·쪽수를 확인하고 최초 배치 차이부터 PDF/Native를 직접 확대 대조한다.
2. 현재 단의 실제 컷·원점과 측정 문맥을 대조한다. 입력 저장 정보와 현재 메트릭의 소비를
   확인한 뒤 구현을 수정한다. 물리 예산의 오류를 컷 변경이나 bleed로 숨기지 않는다.
3. 수정 영역과 정상 대조군을 다시 캡처한다. source가 변하면 해당 결과를 재검증한다.
4. 요청된 52개 원본의 최종 Native/fresh WASM 전쪽 및 미세 수식 차이를 확인한다.
5. 90% 이상 선행 조건 후 관계형 검사 교정, 전체 회귀·lint·정책·Studio 검증을 완료한다.

Stage31 fresh WASM은 이 생산 코드의 미주 보정을 포함하지 않는다. 이후 fresh WASM을
다시 빌드하며 이전 bundle을 최신 검증으로 보고하지 않는다. 새 PR·원격 push는 아직
실행하지 않았고 PR/CI/merge/후속 처리까지의 기존 목표를 유지한다.

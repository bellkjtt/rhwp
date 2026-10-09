# 이슈 #7642 — 미주 경계 간격의 예약과 소비 일치

시작 커밋은 `b55369c0d`다. 이번 회귀 실패는 현재 브랜치에서 보정한다.

## 독립 근거와 원인 경로

원본 `samples/3-11월_실전_통합_2022.hwpx`의 2024 Print 13쪽에서
(iii) n=9 그림 설명은 왼쪽 단 하단, 그림은 오른쪽 단 시작에 배치된다.
현재 출력은 설명까지 오른쪽으로 이월한다. 앞 미주 그림의 뒤 간격을
`prepare_endnote_emit → apply_endnote_tail_spacing`에서 현재 단에 예약하지만,
다음 제목의 `compute_endnote_metrics`가 직전 저장 bottom에서 다음 bottom까지
같은 간격을 다시 누적한다. 실제 렌더는 예약된 tail 간격을 한 번만 소비한다.

변경 전 기록은 ignored `stage15-current-validation/nov2022-before-p13-fit.log`와
`nov2022-p15-unmodified-vpos-debug.log`에 있다. pi593의 tail 예약 후 누계는
224.08이고 pi594는 자체 줄 전진 18.0267 대신 38.4533을 더한다. 중복분
20.4267은 이미 예약한 간격 증가분과 같다. 이는 진단값이며 구현 상수가 아니다.
실제 마지막 그림 점유 뒤 설명 줄은 들어가는데 누계의 초과 예약 때문에 이동한다.

## 보정 계약

- tail 간격을 소유한 현재 단에 실제 더한 증가분을 준비 결과에 전달한다.
- 새 미주의 첫 문단이 같은 단에 남고 저장 vpos 전진에 그 경계 간격이 포함된
  경우에만 이미 예약한 부분을 소비에서 제외한다.
- 문단 자체 줄 전진과 trailing 간격은 보존한다. 단 이동 후에는 앞 단의 예약을
  새 단에서 차감하지 않는다. 내부 되감김의 별도 줄 합산 경로도 그대로 보존한다.
- 저장 bottom 자체를 변경하거나 bleed·픽셀 허용치를 추가하지 않는다.
  원문 파일·Document/IR·기존 회귀 기대값·baseline은 변경하지 않는다.

생산 `state/commands.rs`의 실제 예약 → `prepare.rs`의 미주별 전달 →
`paragraph.rs`의 첫 문단·단 소유 확인 → `metrics.rs`의 fit/advance를 연결한다.
최종 render는 기존 tail 간격을 사용하며 따로 간격을 줄이지 않는다.

## 검증

Native build·Clippy 후 같은 2024 Print로 13·14·15·17쪽의 실제 출력과
HWP/2020 정상 대조를 확인한다. zero-gap HWPX는 예약이 없는 반례다.
개선되면 전쪽 Native와 fresh WASM을 실행하고 미세 수식·간격을 확대 판독한다.
90% 선행 조건을 만족하기 전에는 관계형 회귀로 기대값을 변경하지 않는다.

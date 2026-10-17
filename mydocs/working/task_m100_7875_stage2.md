# #7875 2단계 — feature 통합 보정과 검증

Issue: [#7875](https://github.com/edwardkim/rhwp/issues/7875)  
원 PR: [#7876](https://github.com/edwardkim/rhwp/pull/7876)

## 진입 근거와 적용

[1단계](task_m100_7875_stage1.md)에서 프로젝트 범위와 실제 시작/export 문제를 먼저 확인했다.
기준 devel은 `515bd72dcde0eecdeece333666b675354b7fa91d`이며 원 기능 1커밋을 적용했다.

- 원본: `b273e3fc44dd713f33d22846faa6fb1d98f01d3b`.
- 체리픽: `1869234ed8c2e98138677bee2c76baeb64ed5c23` (`-x`, 원 author·co-author 보존).
- 충돌은 Cargo.toml 기본 feature 목록 1곳이었다. 기존 `pdf-import`와 그 의존성 목록을 보존하고 `wasm-api`를 추가했다.
- 현재 기본값은 `console_error_panic_hook`, `pdf-import`, `wasm-api`다. 원 fork는 변경하지 않았다.

## 메인터너 보정 계획

1. 주석에서 일반 Rust main 자동 시작과 명시적 wasm-bindgen start 연결을 구별한다.
2. `wasm_api` Rust facade도 feature에 묶인다는 사실과, 기본 feature를 끈 기존 소비자의 명시적 opt-in을 설명한다.
   Cargo feature 통합은 additive이므로 다른 의존성이 켜면 다시 포함될 수 있음을 구분한다.
3. native·wasm lib의 `--no-default-features` Clippy와 native bin 선택을 실제 실행한다.
   실행으로 확인한 gate 누락만 보정하며 무관한 feature 조합·SDK·배포 채널을 추가하지 않는다.
4. 적용 전 기본 패키지의 JS·d.ts·WASM export 계약과 적용 후 기본 패키지를 대조한다.
   `.wasm` 차이가 생기면 실제 코드 변경과 feature metadata 차이를 독립 대조군으로 구분한다.
5. 1단계와 같은 소비 앱 소스를 그대로 사용해 main 자동 실행 복원·export 제거·파싱 보존을 확인한다.
6. Rust lint·focused·전체 integration·Native Skia·fresh WASM의 해당 검증을 순차 실행한다.
   자동 수집을 먼저 확인하고 불필요한 CI 호출은 추가하지 않는다.

## 원인과 기대 계약

`src/lib.rs`의 `pub mod wasm_api`와 wasm-bindgen export/start 속성이 JS 링크의 root다.
Cargo의 `wasm-api`가 켜진 기본 빌드에서는 같은 모듈·같은 속성을 소비하고,
꺼진 라이브러리 빌드에서만 facade와 자동 start/export를 제외한다.
parser/model/DocumentCore 구현·조판 조건·paint·font/resource·기준값은 변경하지 않는다.
`init_panic_hook`·`version`은 항상 Rust 함수로 남겨야 한다.

기대값은 1단계의 독립 소비 앱과 공식 wasm-bindgen 0.2.127 계약을 따른다.
일반 main은 feature off에서 자동 실행돼야 하며 앱이 명시한 start는 계속 실행돼야 한다.
라이브러리 모드에서 HwpDocument export는 없어야 하고 기존 HWP5/HWPX는 동일하게 파싱돼야 한다.
기본 모드의 API·CLI·Native 소비는 보존돼야 한다.

## 검증 상태

보정 후 검증은 진행 중이다. 이 문서의 계획을 PASS나 GitHub CI 완료로 세지 않는다.
전체 실행 로그·출처·일시적인 패키지는 `output/pr-review/pr7876-20261017/`에 보존한다.
최종 수용·remote PR·CI·merge 판정은 실제 결과가 나온 뒤 별도로 기록한다.

## 첫 실행 결과

체리픽 직후 `cargo clippy --locked -p rhwp --lib --no-default-features --target-dir /home/tsjang/rhwp/target/pr-review -- -D warnings`는 통과했다(exit 0, 2분 20초).
관측한 기능 gate 누락은 없었다. 이후 보정은 Cargo/lib의 일반 주석을 위에서 재현한 조건으로 정확히 고친 것이며 함수 본문·export 조건은 바꾸지 않았다.
이 결과는 뒤이어 실행할 최종 기본 mode lint·WASM/소비 앱·전체 회귀를 대신하지 않는다.

# 이슈 #7642 Stage 46 — 실제 미주 경계의 설정 간격과 그림 원점

Issue: #7642

## 시작점과 독립 근거

시작 source는 `981632ee9b8217d20a555bc90164ba8f4ec093b0`, 정책 base는
`79abd9e49675f4286eaed38197f7e837afb744e7`이다. 전체 요청 55문서·853쪽의
Native TSV를 이 source의 고정 바이너리로 산출 중이다. fresh WASM과 전체 회귀는
미완료이며 범위를 축소하지 않는다.

9/8/7 HWPX 10쪽의 미주 6 본문은 독립 Print와 같은 순서·거의 같은 위치다.
마지막 수식 줄의 Native 원점은 236.5, Print 잉크 상단은 236.48이다. 다음
미주 7 제목은 Native 254.5 / Print 278.664로 갈린다. 섹션 설정의 미주 사이
`betweenNotes=2267` HU는 30.2267(96 DPI)이며, 앞 수식 줄 상자 높이 12와 이
간격을 합하면 독립 제목 원점과 일치한다. 원본 마지막 줄의 간격은 452 HU다.
현재 layout은 앞줄 간격이 1000 HU를 넘는 삽입 마커에서만 미주 간격을 읽어
설정값을 잃는다. 이 진단 좌표는 원인 추적용이며 회귀 선택자·기대값으로 쓰지 않는다.

구분선 없는 HWPX 11쪽의 BMP 그림은 형상·물리 크기가 맞지만 실제 페이지에서
Native 상단 862.3 / 독립 Print 이미지 객체 상단 855.384로 갈린다. PDFium의
실제 이미지 객체 bounds로 확인했고 페이지 잉크 상단의 추정값으로 대체하지 않았다.
이전 단계의 100% 그림 프레임 보조값은 이 원점 차이를 판정하지 않는다.

## 소비 경로와 묶음 보정

- 섹션의 미주 사이 설정 → 실제 앞 미주 마지막 줄 소유 판정
  (`endnote_spacing::starts_note_after_complete_tail`) → layout의 실제 줄 점유 끝
  → `origin_after_gap` → HeightCursor의 후속 원점 이동을 연결한다. 완전한 미주
  경계에는 설정값을 사용하고, 첫 미주·같은 미주 내부·새 단 첫머리에 이를 덧붙이지 않는다.
  제목 문자열이나 삽입 마커의 크기로 실제 경계 소유를 대신하지 않는다.
- 저장 BMP 프레임 → `picture_frame::relative_frame` → 현재 단의 실제 paint
  `flow_bottom` → `picture_frame::at_origin` → float placement의 host·occupied·후속
  flow 소비를 연결한다. 실제 흐름 결과가 없는 경로의 기존 fallback은 명시하고
  미검증으로 남긴다. 그림 크기를 바꾸거나 고정 좌표로 원점을 맞추지 않는다.

## 검증 범위

관련 수정을 함께 빌드한 뒤 기존 다섯 쪽수 실패, 2022년 11월 정상 대조군,
9/8/7 HWPX 10·11쪽과 구분선 없는 HWPX 11쪽을 비교한다. 제목 간격과 그림
후속 수식까지 직접 판독한다. 개선된 범위를 확인한 뒤 같은 source의 전체
Native/fresh WASM TSV·확대 판독·관계형 회귀·전체 회귀를 이어간다. 모든
실행 로그·TSV·JSON·nextrun은 ignored output에 두며 Git에 포함하지 않는다.

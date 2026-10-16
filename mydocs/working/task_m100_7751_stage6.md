# #7751 그림 색조의 저장·출력 계약 — 6단계

2026-10-16, 시작 HEAD `e0b69960e`. 기존 독립383쪽 Print PDF
`3cdf329004cffad0ebacd122586e5e1702aff36b3015cbcb401d038857019593`를 유지한다.
전체 원본의 반복 MCP 출력은 하지 않는다.

## 독립 근거와 반례

한컴2024가 직접 저장한 합성 WMF HWP는 의미 밝기/명암
`(0,0),(-5,0),(8,0),(-20,0),(20,0),(0,-5),(0,8)`을 이진 레코드에
`(0,0),(0,-5),(0,8),(0,-20),(0,20),(-5,0),(8,0)`으로 기록한다.
job `5bf9b059-e3a6-4331-877a-439cbfbfe54a`, 저장본 SHA
`0413cd5f75c80abac870181cea1b865a1590ff00d9f561e554bd6db7a4fe9e9c`다.
따라서 현재 parser/serializer의 명암→밝기 순서를 단순히 되돌리지 않는다.

반면 HWPX→HWP adapter의 `normalize_picture_geometry_for_hwp`에는
serializer가 모델 순서대로 기록하던 시절의 `mem::swap`이 남아 있다.
실제로 rhwp 변환본은 첫 비영 조건을 `(-5,0)`으로 저장해 한컴에서 명암-5로
출력한다. 한컴 저장본과 현재 serializer의 계약을 중복 적용하는 오류다.
메타데이터 보존 경계에서 의미 값을 유지하고 파일 바이트 순서는 serializer가
한 번만 결정하도록 보정한다. 이미지 형식·묶음 여부로 이 교환을 분기하지 않는다.

130쪽의 별도 문제는 아직 미확정이다. 원본 그림376의 이진 `(0,-5)`는
의미 밝기-5이며 원본 TIFF354의 `(0,8)`는 밝기8이다. 실제 한컴 HWPX 저장도
이 해석과 같다. 그런데 기존 편람 Print의 WMF 흰색은 RGB249, 현재 Native는
242다. 합성 그림의 Print 및 기존 PDF에서 실제 원본 그림의 내부 픽셀·색공간·
변환/필터 경로를 대조한다. 합성 입력 통과를 원본 편람의 일치 증거로 대신하지 않는다.

## 실제 소비 경로와 검증 순서

1. HWPX semantic `ImageAttr` → HWP adapter → `serialize_picture_data`의
   두 i8 → HWP parser → 실제 한컴 Print를 대조한다. 정상 비영 값·부호·0을
   독립 저장본 상수와 비교하며 재저장 시 재교환하지 않는지 확인한다.
2. 원본 그림 → `resolve_image_payload`의 WMF→SVG → Native의 SVG raster와
   `adjusted_picture_bytes`/필터, fresh WASM의 같은 그림 소비 위치를 추적한다.
   변환 전후 형식이 바뀌어 보정의 색공간·순서가 달라지는지 확인한다.
3. 원본 그대로의130쪽과375쪽, 원점 변경으로 소폭 하락한224·226쪽을 먼저
   확인한다. 원인이 설명되지 않은 차이는 미검증으로 남기며 허용치를 바꾸지 않는다.
4. 새 Native/fresh WASM에서 전383쪽 raw TSV와 baseline 비교를 갱신한다.
   90%/98% 조건 및 직접 확대 판독이 충족된 뒤 보류 중 정식 회귀를 추가하고
   lint·필수 전체 검증을 진행한다. 최종 code head와 증적의 관계를 기록한다.

원시 진단 입력·job 상태·관측표는 ignored
`output/pr-review/pr7494-20261016/`에 보존한다. 변환기의 이중 교환 오류와
원본130쪽의 raw 하락은 구분하며, 하나의 수정으로 모두 해결됐다고 주장하지 않는다.

## 리베이스 전 체크포인트

adapter의 이중 교환을 제거한 Native CLI 빌드는 공용 `target/pr-review`,
jobs8, `--locked --profile release-test --features native-skia --bin rhwp`로
4분59초에 exit0이었다. 합성7조건을 HWPX→HWP 변환한 실제 i8 바이트는
한컴 직접 저장본의7조건과 모두 일치했다. 수정 전 결과는 앞 절의 반대 순서로
남아 있다. 이 검사는 저장 계약의 증거이며 편람130쪽의 시각 통과 증거가 아니다.

원본 그림376의 속성·WMF를 보존하고 본문을 한 그림으로 줄인 별도 진단 HWP는
한컴 Print1쪽을 만들었다(job `46c52893-c7ea-4108-a0c5-195fbcb8a360`).
그 PDF 내부 그림과 기존 편람130쪽 내부 그림의 JPEG 바이트가 동일했다.
공통 SHA는 `55be64ec566d4b858359e265d1176f5177a319bac8600a75192100cc5ced56ae`다.
기존 PDF의 잘못된 색을 가정하거나 새로운 PDF로 기준을 교체할 근거가 없음을
확인했다. 진단 첫 시도는 DocInfo 구역 수14와 실제1구역의 불일치로 preflight에서
실패했고, 진단본의 구역 수만 수정한 뒤 실행했다. 이 환경 실패를 결함 재현으로 세지 않는다.

`picture-contract-adapter-evidence.json`, `isolated-picture-canonical-equality.json`,
`logs/picture-contract-native-build.log`에 관측을 보존했다. 사용자의 동기화 지시에
따라 새 기준 `552514f423ec91f1b7171f6e6b0882a4efe034f2`로 리베이스한다.
상위 표 조판 변경을 포함하므로 이전 producer를 보존하고, 리베이스 후 최종
시각·회귀·lint 검증을 갱신한다. 현재 PR 제출 및130쪽 보정 완료는 보류다.

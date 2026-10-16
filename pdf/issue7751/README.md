# #7751 그림 색조의 독립 한컴 Print 기준

2026-10-16, 한컴2024 `13.0.0.3901`, `PrintMethod=0`,
`frame_print_to_pdf_ex_one_up`, 각1쪽. rhwp 출력으로 만든 기준이 아니다.

## 입력과 보존

RGB11패치 PNG를10개의 명시적 합성 그림 행으로 배치한 HWPX를 실제 한컴으로 인쇄했다.
편람에서 구역·스타일·그림 XML 틀을 가져왔으나 본문·그림 픽셀·밝기·대비는 교체했다.
이는 색조 계약의 합성 대조군이며 편람 원본의 시각 일치를 대신하지 않는다.
원본 편람은 별도383쪽 Native/fresh WASM 검증 대상이다.

생성 코드, 입력 HWPX, 원시 job JSON과 추출 PNG/JSON은 로컬 ignored
`output/pr-review/pr7494-20261016/`에 보존한다. 기존 입력은 사용하지 않는 원본 자원을
포함하므로 정식 sample/corpus에는 등록하지 않는다. 아래 입력 SHA는 인쇄한 바이트를 고정한다.

| Print PDF | 바이트 | SHA-256 | 입력 HWPX SHA-256 | 한컴 job |
| --- | ---: | --- | --- | --- |
| [hancom-color-probe-2024.pdf](hancom-color-probe-2024.pdf) | 18661 | `7c5e9ef524dee839e53f510dd08cf2539ddfa524969651a42ad8e0b45a35388d` | `ab2cfbe42a6b404bf0228ddd39294da07ffb27712a9a5c80db25cd289a95109d` | `4ac093ee-057b-455f-afe0-d621b0cdacad` |
| [hancom-color-probe-cross-2024.pdf](hancom-color-probe-cross-2024.pdf) | 18030 | `9d651d00828649ecdf383b24eff9a3ff00425e5ddb5e381ffde32b61a8a7b36d` | `af91f54dfe3c9368ae6791be6822a660bbc825241571d16dc4e28405baf82ecc` | `114173c0-fd06-4c97-a74d-be2d78670310` |
| [hancom-color-probe-boundary-2024.pdf](hancom-color-probe-boundary-2024.pdf) | 9854 | `0a7f823610eeee5eb4a6a6eff1067001655afd2473db6301111c455d9a8cbab6` | `463b14b05bc8d4a922a7edfab404d4387cf27792604fdeff914a00cc4d56b7eb` | `ec6b5b8b-56aa-47b8-bd69-df3de7ad6839` |

## RGB 관측값

`pdfimages -png <PDF> <prefix>`로 PDF 내 이미지 픽셀을 직접 추출했다.
아래 기대값은 그 픽셀의 측정값이다. 구현의 색조식을 다시 계산한 값이 아니다.
각 행의 대표 입력 RGB는 `(80,100,120)`이다. ±1 차이는 한컴의 정수 양자화 범위다.

| Print | 밝기 | 대비 | 효과 | 관측 RGB |
| --- | ---: | ---: | --- | --- |
| 기본 | 0 | 0 | REAL_PIC | `(80, 100, 120)` |
| 기본 | 8 | 0 | REAL_PIC | `(100, 120, 140)` |
| 기본 | 0 | 8 | REAL_PIC | `(76, 97, 119)` |
| 기본 | 20 | 0 | REAL_PIC | `(131, 151, 171)` |
| 기본 | 0 | 20 | REAL_PIC | `(68, 93, 118)` |
| 기본 | 20 | 20 | REAL_PIC | `(125, 150, 175)` |
| 기본 | -20 | 0 | REAL_PIC | `(29, 49, 69)` |
| 기본 | 0 | -20 | REAL_PIC | `(89, 105, 121)` |
| 기본 | 0 | 50 | GRAY_SCALE | `(64, 64, 64)` |
| 기본 | 10 | 0 | BLACK_WHITE | `(0, 0, 0)` |
| -cross | -20 | -50 | REAL_PIC | `(65, 75, 85)` |
| -cross | 20 | -50 | REAL_PIC | `(142, 152, 162)` |
| -cross | -20 | -20 | REAL_PIC | `(43, 59, 75)` |
| -cross | 20 | -20 | REAL_PIC | `(135, 151, 167)` |
| -cross | -20 | 20 | REAL_PIC | `(10, 35, 60)` |
| -cross | 20 | 20 | REAL_PIC | `(125, 150, 175)` |
| -cross | -20 | 50 | REAL_PIC | `(0, 0, 35)` |
| -cross | 20 | 50 | REAL_PIC | `(108, 148, 188)` |
| -cross | 0 | 50 | GRAY_SCALE | `(64, 64, 64)` |
| -cross | 50 | 0 | BLACK_WHITE | `(255, 255, 255)` |
| -boundary | -100 | -100 | REAL_PIC | `(0, 0, 0)` |
| -boundary | -20 | -100 | REAL_PIC | `(0, 0, 0)` |
| -boundary | 0 | -100 | REAL_PIC | `(1, 1, 1)` |
| -boundary | 20 | -100 | REAL_PIC | `(52, 52, 52)` |
| -boundary | 100 | -100 | REAL_PIC | `(255, 255, 255)` |
| -boundary | -100 | 100 | REAL_PIC | `(0, 0, 0)` |
| -boundary | -20 | 100 | REAL_PIC | `(0, 0, 0)` |
| -boundary | 0 | 100 | REAL_PIC | `(0, 0, 0)` |
| -boundary | 20 | 100 | REAL_PIC | `(51, 51, 51)` |
| -boundary | 100 | 100 | REAL_PIC | `(255, 255, 255)` |

회색조·대비50의 원색 빨강 `(255,0,0)`은 `(24,24,24)`다.
회색조를 바이트로 먼저 확정한 다음 대비를 적용한 출력이며, 기존 SVG의
linearRGB·바깥 회색조 중첩과 다르다. 대비±100은 중간 범위의 극한과 달라
기본값1 또는 원본128 초과의 이산 양자화 뒤 밝기를 적용한다.

## 재확인 명령

```bash
pdfinfo pdf/issue7751/hancom-color-probe-2024.pdf
pdfimages -png pdf/issue7751/hancom-color-probe-2024.pdf output/pr-review/pr7494-20261016/color-check
```

변환 호출은 `mydocs/manual/mcp_hwp2024Convert_usage.md`를 따른다. 인증 값은 기록하지 않는다.
이 자료만으로 미지원 이산 흑백·패턴·워터마크 Direct PDF 경계를 수용하지 않는다.

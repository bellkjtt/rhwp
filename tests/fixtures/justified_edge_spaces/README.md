# 양쪽 정렬의 줄 가장자리 공백 대조군

`leading-tracking-contract.hwpx`는 한컴 저장본이 아닌 수동 합성 입력이다. 10pt Times New Roman으로 `alpha beta gamma delta epsilon zeta eta theta `를 3회 반복하고, 선두 공백 0·1·3개 및 자간 0·−12%의 각 조건을 Left/Justify 두 문단에 적용했다. 매 조건 앞의 명시적 쪽 나눔으로 6쪽, 12문단이다.

## 독립 기준

- 입력 SHA-256: `cb7d4ae03e319c6f33b0432a118c2c25dbfee522a4bbb88bef8868aa480d899c`
- PDF: [원문 그대로 한컴 Print로 출력한 6쪽](../../../pdf/issue7642-leading-tracking-contract/leading-tracking-contract-2020.pdf)
- PDF SHA-256: `fd7ae62cb38455e2d114d233290babd48a595bb36c0c8b8582b65d90ddbb84a4`
- 한컴 MCP engine: `2020`, job `a2492590-4fec-4e6d-9cd9-45fdda8e328c`, target `pdf`. HWP/HWPX 재저장을 기준으로 사용하지 않았다.
- 검증 구현: `fa56a97f683fb7ca690ae6a155bf1902bcfc9406`, 96dpi, compat `2022`, Native Skia와 fresh WASM Canvas2D.

전체 6쪽에 누락·미달이 없으며 독립 PDF와 양쪽 출력의 쪽수가 같다.

| 쪽 | Native 관용 실루엣(%) | fresh WASM 관용 실루엣(%) |
| --- | ---: | ---: |
| 1 | 97.19000 | 97.46868 |
| 2 | 98.59310 | 98.63457 |
| 3 | 98.59813 | 98.63496 |
| 4 | 99.47322 | 99.64397 |
| 5 | 99.47879 | 99.64518 |
| 6 | 99.47766 | 99.64518 |

전쪽 최저값은 Native 97.19000%, WASM 97.46868%다. 앞 공백·단어 사이 간격·자동 줄바꿈·가시 끝을 같은 영역에서 직접 판독했다. 2쪽 첫 `a`의 독립 PDF 원점은 두 정렬 모두 55.10205pt(96dpi 약 73.4694px)다. 양쪽 98% 이상인 이 쪽만 ±2px로 보호하며 나머지는 문자 소유·순서·자연 전진 관계로 검사한다. 공개 JSON의 소수 첫째 자리 양자화 허용 오차는 배치의 px 고정 허용치와 구분한다.

실행 원장은 ignored `output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/`에 있다. Native 전쪽 TSV는 `stage1130-focused-native/head/native/leading-tracking-contract/head-native-leading-tracking-contract/silhouette.tsv`, fresh WASM은 `stage1171-browser-contract-tsv/silhouette.tsv`다. 후자는 `stage1171-browser-contract/verified-edge-spaces`의 같은 6쪽 PNG를 `--png-pair`로 측정했고, 원문·PDF·생산자 해시·전체 쪽수는 원래 실행 manifest에 연결되어 있다. 이 TSV만으로 원문 쪽수를 추정하지 않는다.

WASM 검증은 Google Chrome for Testing 152.0.7977.75 실행파일 SHA-256 `3c84cfdbadd0b5b9d1943568b67faea54cf818d20ca32a337fd8a68a0d61ac76`을 명시했다. 새 브라우저에서 6쪽을 다시 렌더한 결과이며 이전 Snap 캡처와 혼합하지 않았다. `stage1171-page1-detail-review.png`부터 `stage1171-page6-detail-review.png`는 같은 실행의 대표 영역을 자른 판독 증거다. 원본 review·standalone overlay도 해당 실행 아래 보존한다.

이 대조군의 통과는 원래 시험지·계약서 전체나 HML 수식의 완료 증거가 아니다. 생산 코드·입력·글꼴·backend가 바뀌면 관련 페이지를 다시 측정하고 직접 판독한다.

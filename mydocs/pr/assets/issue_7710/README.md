# #7710·PR #7715 인쇄 profile 검증 증적

## 원본과 기준

- 원본: [`samples/field-01.hwp`](../../../../samples/field-01.hwp), SHA-256 `518cb939079e6e0640a5f813597f744e2528a17ca52ee418929f1c8f4b5380c0`.
- 독립 Print: [`pdf/field-01-hwp-2020.pdf`](../../../../pdf/field-01-hwp-2020.pdf), 129044바이트·3쪽, SHA-256 `ec488080ac777c355444bf98e0d0fb34ef02600870313bcaa212aff4500d8d0b`.
- 기준 PDF는 oracle commit `9d18eb5a5fd1d9db28a331ec85727762a2ecec11`에 등록됐다. 당시와 현재 원본의 Git blob은 `99b2aca257d716b5d751052af288d82983c558ec`로 같다. Creator는 Hwp2022이며 파일명만으로 실제 MCP 엔진 연도·PrintMethod 영수증을 추정하지 않는다.
- devel 기준: `8766fb47d8ec11547f0d2ae941c53cc3c9780ab2`.
- 보정 제품 source: `cfd06cd2b5cc47a268134d0c09f657a7a9fd24d0`. 이후 `f1277506ac0d093f5109b16cbef0487f5f6a0ebd`는 E2E의 Windows 경로만 변경했다.
- Native Skia PNG와 fresh WASM Canvas2D PNG, Print profile·compat2022·96dpi·동일 글꼴 환경을 사용했다. Docker가 없는 호스트에서 WASM은 wrapper `--target web --out-dir pkg --no-opt`, jobs8·release codegen-units16으로 빌드했다. 최적화 배포 산출물 검증으로 표현하지 않는다.

## 전쪽 raw 실루엣 TSV

색상 경계 보정 전 `silhouette_raw_match_percent`로 판정한다. 2px 관용 실루엣은 전체 픽셀 fidelity와 다르다.

| 쪽 | devel Native | 보정 Native | devel WASM | 보정 WASM |
| --- | ---: | ---: | ---: | ---: |
| 1 | 99.99154 | 99.99154 | 99.98702 | 99.98702 |
| 2 | 98.73913 | 99.99813 | 98.73280 | 99.99907 |
| 3 | 100.00000 | 100.00000 | 100.00000 | 100.00000 |

모두 전체3쪽이며 90% 미만·누락·쪽수 차이0, Decimal 기준0.01%p 이상 하락0이다.
각 TSV를 원문으로 제출한다. SDK print SVG도 실제 MessageChannel 호출로3쪽을 받아 별도 raster했으며
99.97476/99.99220/100%다. SDK 경로는 Studio 편집 문서이므로 Native/WASM의 동일 producer 비교와 구분한다.

- [devel Native](field-01_devel_native_silhouette.tsv) · [devel WASM](field-01_devel_wasm_silhouette.tsv)
- [보정 Native](field-01_native_silhouette.tsv) · [보정 WASM](field-01_wasm_silhouette.tsv)
- [실제 SDK print SVG](field-01_sdk_print_silhouette.tsv)

## 직접 판독

Native/fresh WASM1·2·3쪽에서 제목·번호·필드 주변·뒤 내용을 대조했다. 2쪽03/04/05의
추가 안내문3개가 사라지고 번호 소유·순서는 유지됐다. Native1·3쪽 PNG는 기준과 바이트 동일하다. 2쪽 Screen profile PNG도 전후 바이트가 같아
편집 안내문 표시는 유지된다(SHA-256 `84c4ff4f4de3e6b519fa922623a972d6ee56c102bf701d6c1720842efe2e6b57`).
사진의 색상 차이는 남는다. 2쪽 엄격 내용 픽셀 일치율은 Native90.66288%·WASM90.03802%다.

- 보정 전: [review](native_before_review_002.png) · [overlay](native_before_overlay_002.png)
- 보정 후 Native: [review](native_review_002.png) · [overlay](native_overlay_002.png)
- 보정 후 fresh WASM: [review](wasm_review_002.png) · [overlay](wasm_overlay_002.png)

완료2쪽의 핵심 번호 정렬은 PDF의 x65.004768pt→96dpi86.673024px를 ±2px로 고정하고,
01–05 누락·중복·세로 순서 관계도 유지한다. 더 높은100%의3쪽은 글상자 안내문 경계를 실행하지
않아 해당 동작을 검증하는2쪽을 선정했다. 원본·PDF·TSV·PNG를 변경한 기준값 완화는 없다.

## 재현 명령

공용 `target/pr-review`의 최신 Native Skia CLI와 fresh `pkg/`를 준비한 뒤 저장소 루트에서 실행한다.

```bash
venv/bin/python scripts/visual_sweep.py --silhouette-only \
  --hwp samples/field-01.hwp --pdf pdf/field-01-hwp-2020.pdf --key field-01 \
  --rhwp-bin target/pr-review/debug/rhwp --compat 2022 --dpi 96 \
  --out output/pr-review/field-profile-native-scores
venv/bin/python scripts/visual_sweep.py --silhouette-only \
  --hwp samples/field-01.hwp --pdf pdf/field-01-hwp-2020.pdf --key field-01 \
  --rhwp-bin target/pr-review/debug/rhwp --wasm-pkg pkg --compat 2022 --dpi 96 \
  --out output/pr-review/field-profile-wasm-scores
```

일반 PNG 판독은 서로 다른 `--out`에 `--silhouette-only`를 빼고 `--pages 2`로 실행한다.
정식 Rust 회귀는 `node scripts/run-rust-test.mjs issue_7710_initial_field_container -- --cargo-profile release-test --target-dir /home/tsjang/rhwp/target/pr-review`다.

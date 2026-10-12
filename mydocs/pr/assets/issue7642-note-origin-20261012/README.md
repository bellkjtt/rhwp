# #7642 저장 미주 0 원점 보정 증거

- code source: `fcd86c8ae30515dfdac3cdbaedaa466f3b81ebe5`
- 입력: `samples/3-09월_교육_통합_2022.hwpx`
- 입력 SHA-256: `b3ed3d5c4f0f95f9a8990b17720da3bef566b5a36174153aa3def28fd46e6029`
- 독립 한컴 2024 Print: `mydocs/pr/assets/pr_7600_control_print_20261006/input-02-3-09월_교육_통합_2022-2024.pdf`
- PDF SHA-256: `052f390b36e53a2374b3883a82c96164802784497259f1e7d5e3fe06b9a163f6`
- Native binary SHA-256: `1fb5a8fda95bac598f7d0e8514bbc85e2e56745f9f7fb365c8a9fdc8d91d2f98`
- 동일 원문·PDF, compat 2024, 전체 글꼴 바이트, print profile, 96dpi다.

| 쪽 | Native 167 | Native 168 | 증거 |
| --- | ---: | ---: | --- |
| 10 | 35.26860% | 98.92453% | [전체 review](native168-p010-review.png) |
| 11 | 62.88459% | 98.82582% | [전체 review](native168-p011-review.png) |

전체 23쪽 Native 검증의 최저값은 91.85820%이며 모두 90% 이상이다. 쪽수는
23/23, 소스 해시 유지 true다. fresh WASM 실제 시각 검증은 진행 중이다. 대표 점수로 전체 검증을 대신하지 않는다. 미주 문7 마지막 수식 줄이
다음 쪽 첫 단에 표시되고 문8–13 내용의 단 소속이 독립 Print와 맞는 것을 확인했다.
수식의 장평·기준선·첨자·장식·미세 간격은 fresh WASM까지 같은 영역을 확대 확인해야
하며 이 자료만으로 최종 승인을 선언하지 않는다.

컴파일된 원본 파서 진단에서 `SO-SUEOP.hwpx` 미주 161의 연속줄 0,0은 0,1200으로
복원되고, 9월 원본 미주 7의 마지막 0은 보존됐다. 이는 파서 반례의 실제 실행
증거이며 전체 문서 시각 검증이나 정식 렌더링 회귀의 통과를 대신하지 않는다.

실행 명령·Native/WASM 영수증·전쪽 TSV·글꼴 공급/복원 해시는 ignored
`output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/`의
`stage168-*`에 보존한다. TSV와 중간 JSON/log는 Git에 넣지 않는다.

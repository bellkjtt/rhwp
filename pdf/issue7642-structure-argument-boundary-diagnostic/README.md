# 구조 명령 인자 경계의 독립 한컴 진단

회귀 fixture/golden의 승인 자료가 아니라 미달 원인의 진단 자료다.
입력 HWPX를 RHWP로 HWP5 전송본으로 변환한 뒤 한컴으로 일반 저장하고,
그 저장본을 별도 Print job으로 출력했다. 비교 대상은 실제 한컴 저장본이다.
원 HWPX의 직접 MCP 열기는 worker 오류로 실패했으며 그 입력의 일치로 보고하지 않는다.

한컴 저장 전후 수식 script 24개와 추가 대조군 30개는 원문 순서·바이트가 같다.
첫 입력은 누락·빈·정상 인자와 중첩 그룹·분수·LaTeX 이름의 문서 해석을 비교한다.
추가 입력은 MATRIX/PMATRIX/PILE/EQALIGN/CASES 각각의 누락·빈·공백·`#`·`&`·정상 인자를 비교한다.

## 출력 출처

- Hancom engine `2020`, build `11.0.0.9136`, PrintMethod `0`, `hancom2020_pdf_driver_one_up`.
- 첫 입력 저장 job: `a17bfb55-fa9c-4c02-9c42-c8b2d11d03e2`.
- 첫 Print job: `9d05293f-7e11-4403-a3b3-2ccce07bd1a9`, PDF 전체 1쪽.
- 추가 hwp job: `4b80ced6-f9a0-4683-98a3-f41830221de2`.
- 추가 pdf job: `6216a18a-ffc1-4630-b7c7-58c2b4079a71`.

추가 Print PDF도 전체 1쪽이다. 일반 저장으로 개체 크기·기준선·LineSeg가 확정되므로
전송 HWP/HWPX와 저장본을 구분한다. 명령·실행 응답·PDF trace·script 비교·TSV는
ignored `output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/stage318-*`,
`stage320-*`, `stage322-*`에 보존한다. endpoint·인증 값은 공개하지 않는다.

## 파일 SHA-256

- `structure-argument-boundary-hancom-2020.hwp`: `bdde1e2e7aa80c491e21b17a66cbd8b5ebe90c786de5d0807a8e81cddbe08de2`
- `structure-argument-boundary-hancom-2020.pdf`: `19c464e5f3157e96693e59d0bea4db04d6033bfb79d8768f2659b770c279600c`
- `structure-argument-boundary-input.hwpx`: `8ee4d26235fd1c841fba95a9d652973c664bc95d4c5e2f8fe78a3140129f5baa`
- `structure-argument-boundary-rhwp-input.hwp`: `f7b33f3c7bdf9b6047d5775b846bde93875e945eecf86992064af879d92b2c2d`
- `structure-empty-arguments-hancom-2020.hwp`: `9783a925dfbae87c1e5db899f9b405e589e01ffd7047594b1a46ea4261380d67`
- `structure-empty-arguments-hancom-2020.pdf`: `ab22ab17823189853b413ade1cbefbf54d067aebdc49d85509ec7a8f97e122e8`
- `structure-empty-arguments-input.hwpx`: `40f242f6d06c5c69df516f4f9f6f82e77b867837c6c3bd711085d153d2a34ac1`
- `structure-empty-arguments-rhwp-input.hwp`: `b204d519e2f3aa33c9b517d0b56bff9517940a730cec3033306a412a004e645d`

## 추가 명령·첨자 오류 경계의 Print 자료

- `native-keyword-precedence`: 저장 job `7062ed9a-1dd6-4a22-8626-b5c23dff378b`, Print job `e57ee230-482a-4ae4-90ab-319acefff042`, 전체 2쪽.
  - `native-keyword-precedence-hancom-2020.hwp`: `d6c1d3142b6b309707c7d8f96556e757df86c7dff841c175da87a0ba7cbd925a`.
  - `native-keyword-precedence-hancom-2020.pdf`: `3303a56f88c094776bc0345acad541d1ee7d5ca16e28e7bdfd77f551650d979c`.
  - `native-keyword-precedence-input.hwpx`: `3f274efc14f441d5a04d135fb082a97c42fd6b88e3814a4f2153b92296e29690`.
  - `native-keyword-precedence-rhwp-input.hwp`: `30d4a6a85a157bb7ec0bcf57e24bb48f543fc01dfe862226e85a0d529a94f1c8`.
- `script-failure-boundary`: 저장 job `ffa2df12-a677-4e0f-bbd7-2c8c0cc0d177`, Print job `10fe4c1a-d360-4d88-8368-5cfcafce9d2c`, 전체 1쪽.
  - `script-failure-boundary-hancom-2020.hwp`: `e01078beebe463b7c8bf7905f30d59403de5c40d2dac7a792dc3728504e0f7d8`.
  - `script-failure-boundary-hancom-2020.pdf`: `87abb0f9ea45ca63a536fe0107f7375584b20776f62ebdddedb3672665eda020`.
  - `script-failure-boundary-input.hwpx`: `9b698d7561a7d311474f4f9ff99cc20d15b062a9812cdf3f2f96cb22b5343d88`.
  - `script-failure-boundary-rhwp-input.hwp`: `5ec33a179e3424bcfd82fc9e0070d4f2c3809f8ba8979e8c04653d07a7118174`.

두 자료도 Hancom engine `2020`, build `11.0.0.9136`, PrintMethod `0`의
`hancom2020_pdf_driver_one_up`으로 출력했다. 모든 수식 script의 저장 전후 보존을 확인했다.
빈 첨자와 명령 우선순위의 진단 자료이며 아직 시각 통과 fixture로 등록하지 않는다.

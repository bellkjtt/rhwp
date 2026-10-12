# 축소 글자·복합 본체 첨자의 독립 Print 대조군

Issue: #7642

한컴 정상 HWPX의 스타일·구역을 재사용한 수동 구조 입력이다. 개체 크기·기준선·저장 LineSeg를 비우고 한컴 2020에서 HWP로 정상 저장한 뒤 그 저장본을 Print 출력했다. 정식 회귀 추가를 승인하는 증거는 아니며 Native/fresh WASM과 확대 판독은 별도로 확인한다.

실제 실행 제품은 `11.0.0.9136`, PDF는 `pdf_print_method: 0` / `hancom2020_pdf_driver_one_up`이다. 서비스 profile 명칭만으로 실제 OS를 추정하지 않는다.

## 입력·산출 고정

- hwp: job `5f4eddb9-06c9-4556-8c02-4b5269b067cb`, SHA-256 `ac7b964913c72e4d8f7d675bf8b467ebbc09017dc68e1260eb62f26328cd30db`.
- pdf: job `c694ca53-3c07-4c5d-b106-b81ed47b9113`, SHA-256 `4e40c2ecf230cd95a2266d32fcc77e3bc9c3d7274386adfc4db659e3568af098`.
- 입력 HWPX SHA-256: `058668115c67a5e09e2d4556c7061f5accf5df663c22983c3723c7c154eacde0`.

## 페이지·스크립트·논리 프레임

| 쪽 | 이름 | 스크립트 | BaseUnit | 높이(HU) | 기준선(%) |
|---:|---|---|---:|---:|---:|
| 1 | line_612 | `x` | 612 | 600 | 86 |
| 2 | sup_612 | `x^z` | 612 | 750 | 89 |
| 3 | sub_612 | `x_z` | 612 | 750 | 69 |
| 4 | left_sup_612 | `x LSUP z` | 612 | 750 | 89 |
| 5 | line_1020 | `x` | 1020 | 1050 | 85 |
| 6 | sup_1020 | `x^z` | 1020 | 1213 | 87 |
| 7 | sub_1020 | `x_z` | 1020 | 1249 | 71 |
| 8 | left_sup_1020 | `x LSUP z` | 1020 | 1213 | 87 |
| 9 | line_1496 | `x` | 1496 | 1500 | 85 |
| 10 | sup_1496 | `x^z` | 1496 | 1800 | 88 |
| 11 | sub_1496 | `x_z` | 1496 | 1822 | 70 |
| 12 | left_sup_1496 | `x LSUP z` | 1496 | 1800 | 88 |
| 13 | root_LSUB | `{SQRT{x}} LSUB i` | 1500 | 2046 | 74 |
| 14 | root_LSUP | `{SQRT{x}} LSUP i` | 1500 | 1995 | 89 |
| 15 | root_SUB | `{SQRT{x}} SUB i` | 1500 | 2046 | 74 |
| 16 | root_SUP | `{SQRT{x}} SUP i` | 1500 | 1995 | 89 |
| 17 | fraction_LSUB | `{{1}OVER{2}} LSUB i` | 1500 | 3771 | 60 |
| 18 | fraction_LSUP | `{{1}OVER{2}} LSUP i` | 1500 | 3720 | 68 |
| 19 | fraction_SUB | `{{1}OVER{2}} SUB i` | 1500 | 3771 | 60 |
| 20 | fraction_SUP | `{{1}OVER{2}} SUP i` | 1500 | 3720 | 68 |
| 21 | sum_LSUB | `{SUM} LSUB i` | 1500 | 2346 | 66 |
| 22 | sum_LSUP | `{SUM} LSUP i` | 1500 | 2295 | 79 |
| 23 | sum_SUB | `{SUM} SUB i` | 1500 | 2346 | 66 |
| 24 | sum_SUP | `{SUM} SUP i` | 1500 | 2295 | 79 |
| 25 | integral_LSUB | `{INT} LSUB i` | 1500 | 3151 | 59 |
| 26 | integral_LSUP | `{INT} LSUP i` | 1500 | 3100 | 69 |
| 27 | integral_SUB | `{INT} SUB i` | 1500 | 3151 | 59 |
| 28 | integral_SUP | `{INT} SUP i` | 1500 | 3100 | 69 |
| 29 | nested_left | `x LSUB {y LSUP z}` | 1500 | 1963 | 65 |
| 30 | four_slots | `x LSUB i LSUP j SUB k SUP m` | 1500 | 2121 | 75 |

612/1020/1496HU의 일반 위첨자와 왼쪽 위첨자 프레임은 각 크기에서 일치했다. 근호·분수·합·적분의 그룹 바깥 왼쪽/오른쪽 명령형 첨자도 같은 본체별 높이·기준선을 보였다. 독립 메트릭을 미측정 크기나 다른 복합 본체로 보간하지 않는다.

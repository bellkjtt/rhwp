# #7494·#7751 최종 제출 전 검증 — 7단계

시작 HEAD `619a12d91`, 기준 `552514f423ec91f1b7171f6e6b0882a4efe034f2`.

## 확인한 상태와 독립 근거

원본 편람과 기존 KoPub 한컴2024 Print PDF를 유지한 전383쪽 검증은 Native
최저92.53354%, fresh WASM 최저92.45371%이며 누락·90% 미만은 양쪽 모두0쪽이다.
원시 TSV는 `output/pr-review/pr7494-20261016/wmf-rgb555-v2-{native,wasm}-all/handbook/silhouette.tsv`다.

동일 입력·PDF·글꼴의 기준 Native 전383쪽 비교에서는224쪽 −0.01876%p,
226쪽 −0.01823%p가 검출됐다. 점수가99%대여도 하락 gate를 면제하지 않는다.
두 쪽은 바깥 표의 셀 안에 있는 TAC 표 원점만 수평 이동했다. 저장 줄 영역,
문단 들여쓰기, 표 바깥여백의 실제 소비를 추적하고 독립 PDF 괘선·글자를 대조한다.
기준 fresh WASM 빌드는 완료했으며 전쪽 실행 결과도 같은 방식으로 비교한다.

## 구현·검증 순서

1. 두 하락 페이지의 저장 줄·문단 속성에서 기대 원점을 구하고, pen 생산부터
   중첩 표 배치까지의 중복 적용 또는 잘못된 가정을 수정한다. 문서 ID나 점수에
   맞춘 상수·조건은 추가하지 않는다. 영향 페이지를 먼저 재출력한다.
2. 기존 완료 경계3건은 유지하고, 양쪽98% 이상인297·375쪽의 독립 기대값과
   소유·포함 관계를 확인해 중요한 경계를 보호한다. 130쪽은 WASM93.34522%이므로
   px 핀을 추가하지 않는다. 실제 WMF의 공통 payload·지연 로딩 계약도 정식
   `tests/cases/`에서 검사한다. 로컬 진단 파일을 fixture로 참조하지 않는다.
3. 최종 생산 소스에서 전쪽 Native/fresh WASM TSV·devel 하락 비교·대표 PNG
   확대 판독, Rust 전체·Native Skia·lint·Studio 적용 검증을 완료한다.
4. upstream head 재확인 후 준비가 끝나면 승인된 통합 PR을 생성한다. 정확한
   code head CI 성공 뒤 같은 PR에 개별 review·오늘할일 trailing commit을 넣고,
   trailing CI·merge·`post_merge.md`의 증적 보존·원 이슈/PR·정리 절차를 따른다.

WMF 저색수 팔레트 선택의 미확정 경계는6단계 기록을 유지한다. 편람의 전쪽
통과를 모든 WMF 입력의 완전한 호환으로 보고하지 않는다.

## 기준 전쪽 비교와 Studio 실행 결과

기준 fresh WASM 전383쪽까지 완료했다. 모든 비교 행을 Decimal로 계산했고,
`stage7-all-pages-base-head-comparison.tsv`에 네 원본 TSV를 페이지별로 결합했다.
동명의 JSON에는 원본 TSV SHA-256과 아래 하락을 기록했다.

| 경로 | 쪽 | 기준 raw | 후보 raw | 변화(%p) |
| --- | ---: | ---: | ---: | ---: |
| fresh WASM | 130 | 93.35693 | 93.34522 | −0.01171 |
| Native | 224 | 99.14080 | 99.12204 | −0.01876 |
| Native | 226 | 99.16137 | 99.14314 | −0.01823 |
| fresh WASM | 226 | 99.20548 | 99.17155 | −0.03393 |

이는3쪽·4경로의 하락이며 현재 판정은 `re_review_required`다. PR을 아직
생성하지 않는다. 기준 Native의 exit1은241·290쪽의 기존90% 미달 검출이고,
383행 누락이나 프로세스 중단이 아니다. 후보에서는 이 두 쪽도90% 이상이다.

Studio `npm test`는2,023 PASS /0 FAIL /0 SKIP,43.49초에 exit0이다.
`npx tsc --noEmit`도5.41초에 exit0이다. 로그는
`logs/stage7-studio-{unit,types}.log`, 명령·종료코드는
`logs/stage7-studio-results.json`에 보존했다. 아직 전체 Rust·Skia·lint의
최종 통과를 대신하지 않는다.

정식 `issue_7751_wmf_picture_payload`는3건을 실제 실행해3 PASS였다.
`issue_7751_handbook_completed_boundaries`는기존3건과추가2건을실행해5 PASS다.
297쪽은 주의표·연도·법령명의 내용 순서·같은 줄 소유와 독립 Print 시작 좌표를,
375쪽은 두 단 서식의 외곽 ±2px·셀 내용 보존·공백 포함 가운데 정렬·바깥여백
한 번 소비를 검사한다. 해당 페이지의 현재 Native/WASM raw는 각각
297쪽99.23737/99.24360%,375쪽99.37858/99.42850%다.

첫 경계 검사 호출은 새 테스트 편집 뒤 자동 suite 배정을 갱신하지 않아0건이었다.
이는 통과로 세지 않았다. manifest를 다시 준비한 후 실제5건을 실행한
`logs/stage7-completed-boundary-tests-r2.log`만 통과 증거다.
Visual Sweep 도구 검사는 `python3 -m unittest discover -s scripts/tests -p
'test_visual_sweep*.py'`에서156 PASS다(`logs/stage7-visual-sweep-tests.log`).

검증 뒤 원격 devel에 `fe5725dc384689b45d60ef8ea4c83138bc2701d8`(#7954)의
조회 CLI 통합이 들어온 것을 확인했다. 이 단계 변경을 먼저 커밋하고 리베이스한다.
기존 검증의 기준 SHA를 새 기준으로 바꿔 적지 않으며, 리베이스 이후 실제
diff·생산 소스 동일 여부와 적용 검증을 별도로 기록한다.

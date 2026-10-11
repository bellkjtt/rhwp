# 외부 생성 HWP3의 원본 줄 재구성과 표 뒤 흐름

- 원본: `samples/issue5699/20099369_yeongwol_forms.hwp`
- 독립 한컴 2020 Print PDF: `pdf/issue7642-failed-inputs-followup-20261011/20099369_yeongwol_forms-2020.pdf`
- Native 전4쪽: 99.26310 / 100.00000 / 99.19799 / 100.00000%.
- 실제 출력 쪽수: 원본 4 / 기준 PDF 4.
- 빌드: `stage146d-native-realign-build.json`, exit 0, 실행 전후 source 해시 일치.
- Native 바이너리 SHA-256: `dc3da30218ab08821c8cb95a96dccb2da9d9b891f8b8eb012053a710760206bd`.
- TSV·실행 증거: ignored `output/pr-review/issue7642/failed-inputs-20261008/stage15-current-validation/stage146d-realign-preflight/`.
- 대표 PNG는 이번 코드의 새 Native 출력이다. fresh WASM 및 전체 범위 검증은 미완료다.

직접 판독에서 첫 표 뒤 문단·서명 원점과 조건9·안전 안내의 두 줄 구성이 대응한다.
원본의 표 바깥 상자·서식별 쪽 소속을 보존했다. 괘선의 잉크 두께와 일부 글자 모양은
독립 Print와 차이가 남아 있으며 전체 시각 완료로 판정하지 않는다.
정상 저장 HWPX는 원인 진단에만 사용했고 원본 입력의 통과를 대체하지 않았다.

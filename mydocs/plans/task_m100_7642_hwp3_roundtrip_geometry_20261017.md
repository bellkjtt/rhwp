# #7642 HWP3 왕복 저장의 도형과 자연 페이지 소유 보존

## 독립 근거와 위반된 계약

원본 `samples/hwp3-sample16.hwp`과 한컴 저장본은 전쪽 Native/fresh WASM
90% 이상이다. RHWP HWPX 왕복 저장본을 그대로 한컴2020 Print한 독립 PDF는
64쪽이지만 Native/fresh WASM 8쪽이 미달이다. 원본과 왕복본의 typed IR을
대조했고, bold 불리언은 보존됐다. raw attr 비트 차이를 굵기 소실로 오판하지 않는다.

`src/parser/hwp3/drawing/shape_mapping.rs`는 사각형·글상자와 타원을 default
기하로 만든 뒤 크기·선·채움만 채운다. 기존 layout은 그 크기로 외곽을 표시하지만
`src/serializer/hwpx/shape.rs::write_rect_pts`와
`section.rs`의 ellipse geom_tail은 0 좌표를 그대로 쓴다. 한컴 저장본의 실제
꼭짓점·중심·축과 다르다. HWP3의 암묵 도형 상자를 parser 경계에서 공통 IR 기하로
정규화하고 serializer와 renderer가 같은 기하를 받도록 하는 것이 보정 후보이다.
원래 HWPX에서 명시된 퇴화 도형을 전역 크기 fallback으로 덮어쓰지 않는다.

원본의 자연 페이지 경계62개는 `page_break_synthesized=true`로만 IR에 남고
HWPX 저장에서 사라진다. 경계를 표준 강제 개행으로 모두 방출하면 다른 정상
문서가 빈 페이지를 만들므로 적용하지 않는다. 내용에 종속된 원본 소유 정보의
보존·재사용 조건을 먼저 확인하며 편집 후 낡은 정보는 수용하지 않는다.

## 배치 원인과 검증 순서

1. typed IR과 독립 한컴 저장본의 기하를 대조한다. 크기·변환·회전과 논리 좌표를
   구분하고 실제 원점·paint 소비를 추적한다.
2. 기존 실패 파일을 보존하고 기하만 바꾼 반례를 독립 Print한다. 진단 대조군
   통과를 원래 저장본 통과로 보고하지 않는다.
3. 일반 규칙의 parser/저장 보정을 모아 적용한 뒤 한 번의 Native/fresh WASM
   빌드로 검증한다. 원본·왕복 저장본과 HWP3 정상 대조군의 전체 쪽수·내용 순서·
   소유 및 전쪽 TSV를 대조한다. 전체 페이지 양쪽90%·확대 판독 전에는 정식
   렌더링 회귀를 추가하거나 기존 실패 기대값을 바꾸지 않는다.
4. 사용자 지시대로 새 이름의 정식 회귀를 만든다. 도형 외곽·텍스트 포함과
   자연 경계의 문단/표 앞뒤 순서·보존을 보호한다. 페이지별 양쪽98% 이상과
   직접 판독을 충족하는 핵심 쪽에만 독립 ±2px 좌표를 병행한다.

증거는 기존 stage2331~2349와 이어 기록한다. 원본·실패 입력·기준 PDF는
바꾸지 않고, TSV/로그/중간 JSON은 ignored output에 둔다.

## 진단 결과와 보정 후보의 소비 경로

기하만 복원한 `hwp3-sample16-geometry-counter.hwpx`의 원본 SHA-256은
`b584d034d48519d43e1bfdea24681da6744cb0d0820a8a05f497dec1af3a7ab2`다.
Print job `a4dd11a3-c3e8-42ec-a3c2-bb652aaa1690`는 engine2020 /
한컴11.0.0.9136 / Print0 / `hancom2020_pdf_driver_one_up`로64쪽을 출력했다.
PDF SHA-256은 `a9561557c0644ab9d8b0a1bfdb8c63ff8945a22cddb2c7651d6051c0c98a630f`다.
동일 producer cc4255의 stage2352 Native / stage2353 fresh WASM 전쪽 비교에서
1·3·5·18·19·39쪽의 미달이 해소됐고20·37쪽은 남았다. 전쪽 완료는 아니다.

20쪽은 원본 문단414의 첫 저장 원점1416HU가 원본 layout에는 반영되지만 왕복본
배치에서 사라진다. 원본의 `ColumnBreakType::Page`와
`page_break_synthesized=true`는 표준 강제 Page와 의미가 다르므로, 앞서 명시
pageBreak를 복원한 진단의 결과로 합성 경계 복원의 결과를 추정하지 않는다.

기하 보정은 HWP3 표68의 개체 크기에서 공통 IR의 사각형 꼭짓점·보통 타원
중심/축을 생산한다. renderer의 사각형·타원 paint는 기존 common 크기 경로를
유지하며 serializer가 같은 IR의 비퇴화 기하를 소비한다. HWPX의 명시적0좌표와
변형 타원의 별도 정보를 이 규칙으로 덮어쓰지 않는다.

자연 경계 보정 후보는 기존 HWP3 출처 마커의 버전2 payload에 구역별 자연 경계
문단 소유를 기록한다. header/구역 XML의 SHA-256·문단 수가 일치할 때만 파서가
공통 Page+합성 상태로 복원한다. 표준 XML의 pageBreak는 계속 생략한다.
낡은 header·구역·잘못된 인덱스/순서·명시적 다른 경계·변경 추적의 표시 투영은
복원 대상이 아니다. 옛 b"1" 마커와 HWP5-origin 저장본의 경로는 유지한다.
구역 파서 → 공통 Paragraph 상태 → 기존 pagination의 합성 경계 분기가 실제
소비 경로이며 renderer에 HWP3 전용 새 분기를 추가하지 않는다.

현재 코드는 보정 후보이며 새 producer의 빌드·재수출·독립 Print·전쪽 TSV·확대
판독은 아직 미실행이다. 기존 결과를 새 코드의 완료 증거로 사용하지 않는다.

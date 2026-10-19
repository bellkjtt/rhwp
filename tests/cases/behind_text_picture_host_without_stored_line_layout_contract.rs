//! [#7905] 글 뒤로(BehindText) 그림만 얹은 빈 첫 문단의 줄 계약 — 저장 줄 배치가 없는 기계 생성 HWPX.
//!
//! 법제처 국가법령정보센터 HWPX 는 `linesegarray` 가 없고 첫 문단이 QR 그림(BehindText, 문단 기준)만
//! 얹은 빈 문단이다. 한/글은 그 문단의 한 줄을 본문 흐름에 남기는데, 그림 항목이 BehindText 를
//! "비전진 레거시 경로"로 되돌리면 그 줄이 사라져 본문 전체가 한 줄(17.3px) 위로 올라온다.
//!
//! 독립 근거 — `samples/복학원서.hwpx` 는 한/글이 저장한 문서라 저장 줄 배치(`linesegarray`)가
//! 한/글이 둔 줄 위치를 그대로 증언한다(한컴 PDF `pdf/복학원서-hwpx-2024.pdf` 와 1쪽 실루엣 일치
//! 97.1%). 첫 문단이 BehindText 로고(문단 기준)만 얹은 빈 문단이다. 같은 파일에서 저장 줄 배치만
//! 떼어 기계 생성본 모양으로 만들었을 때, 위쪽 블록의 글줄은 원본과 같은 y 에 놓여야 한다 —
//! 규칙 적용 전에는 모든 줄이 18.7px(그 host 한 줄) 위에 놓였다.
//!
//! 이 한 가지 관계가 양쪽을 잠근다. 규칙이 저장 증거가 있는 원본까지 번지면 원본의 줄이 움직여,
//! 규칙이 기계 생성본에서 빠지면 사본의 줄이 움직여 둘의 y 가 어긋난다.
#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeMap;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use rhwp::document_core::DocumentCore;
use rhwp::renderer::render_tree::{RenderNode, RenderNodeType};

const SAMPLE: &str = "samples/복학원서.hwpx";

/// 위쪽 블록의 아래 경계(px). 한컴 기준 마지막 줄 `this reinstatement form.` 이 쪽 위에서 약 482px 이다.
/// 그 아래(접수증 영역)는 이 규칙과 무관한 다른 차이가 남아 있어 비교하지 않는다.
const TOP_BLOCK_BOTTOM_PX: f64 = 500.0;
/// 비교할 최소 글줄 수 — 줄을 거르다 빈손으로 통과하지 않게 하는 하한.
const MINIMUM_COMPARED_LINES: usize = 10;
const TOLERANCE_PX: f64 = 0.5;

fn sample_bytes() -> Vec<u8> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(SAMPLE);
    std::fs::read(&path).unwrap_or_else(|error| panic!("read {SAMPLE}: {error}"))
}

/// 문자열에서 `<hp:linesegarray>…</hp:linesegarray>` 블록을 전부 지우고 지운 개수를 돌려준다.
fn remove_line_segment_arrays(section_xml: &str) -> (String, usize) {
    const OPEN: &str = "<hp:linesegarray";
    const CLOSE: &str = "</hp:linesegarray>";
    let mut kept = String::with_capacity(section_xml.len());
    let mut rest = section_xml;
    let mut removed = 0;
    while let Some(start) = rest.find(OPEN) {
        let end = rest[start..]
            .find(CLOSE)
            .map(|offset| start + offset + CLOSE.len())
            .expect("여는 태그에는 닫는 태그가 짝으로 있다");
        kept.push_str(&rest[..start]);
        rest = &rest[end..];
        removed += 1;
    }
    kept.push_str(rest);
    (kept, removed)
}

/// 원본에서 저장 줄 배치만 뗀 "기계 생성본 모양" 패키지.
fn without_stored_line_layout() -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(sample_bytes())).expect("zip");
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let mut removed_total = 0;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).expect("entry");
        let name = entry.name().expect("entry name").to_string();
        let method = entry.compression();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).expect("read entry");
        if name.starts_with("Contents/section") && name.ends_with(".xml") {
            let text = String::from_utf8(data).expect("utf8 section");
            let (stripped, removed) = remove_line_segment_arrays(&text);
            removed_total += removed;
            data = stripped.into_bytes();
        }
        let options = zip::write::SimpleFileOptions::default().compression_method(method);
        out.start_file(name, options).expect("start file");
        out.write_all(&data).expect("write entry");
    }
    assert!(
        removed_total >= 70,
        "전제: 원본은 문단마다 저장 줄 배치를 가진다(73개), 지운 수 {removed_total}"
    );
    out.finish().expect("finish").into_inner()
}

/// 첫 쪽의 글자 있는 TextRun 을 글 내용 → 위쪽 y(px) 로 모은다. 같은 글이 여러 번이면 첫 것만 쓴다.
fn first_page_text_tops(bytes: &[u8]) -> BTreeMap<String, f64> {
    fn collect(node: &RenderNode, tops: &mut BTreeMap<String, f64>) {
        if let RenderNodeType::TextRun(run) = &node.node_type {
            let text = run.text.trim();
            if !text.is_empty() {
                tops.entry(text.to_string()).or_insert(node.bbox.y);
            }
        }
        for child in &node.children {
            collect(child, tops);
        }
    }
    let core = DocumentCore::from_bytes(bytes).expect("HWPX 로드");
    let page_root = core.build_page_render_tree(0).expect("첫 쪽 렌더").root;
    let mut tops = BTreeMap::new();
    collect(&page_root, &mut tops);
    tops
}

#[test]
fn stripped_copy_places_top_block_lines_where_the_stored_original_does() {
    let stored = first_page_text_tops(&sample_bytes());
    let stripped = first_page_text_tops(&without_stored_line_layout());
    let pairs: Vec<(&String, f64, f64)> = stored
        .iter()
        .filter(|(_, stored_top)| **stored_top < TOP_BLOCK_BOTTOM_PX)
        .filter_map(|(text, stored_top)| {
            stripped
                .get(text)
                .map(|stripped_top| (text, *stored_top, *stripped_top))
        })
        .collect();
    for (text, stored_top, stripped_top) in &pairs {
        println!(
            "[줄 위치] {text:?}: 원본 {stored_top:.2}px, 저장 줄 배치 없는 사본 {stripped_top:.2}px, 차 {:+.2}px",
            stripped_top - stored_top
        );
    }
    assert!(
        pairs.len() >= MINIMUM_COMPARED_LINES,
        "비교한 글줄이 너무 적다: {}줄 — 글 내용 대응이 깨졌는지 확인",
        pairs.len()
    );
    for (text, stored_top, stripped_top) in &pairs {
        assert!(
            (stripped_top - stored_top).abs() < TOLERANCE_PX,
            "{text:?}: 저장 줄 배치 없는 사본 {stripped_top:.2}px, 원본 {stored_top:.2}px — host 한 줄(18.7px)이 흐름에서 빠지면 사본이 그만큼 위에 놓인다"
        );
    }
}

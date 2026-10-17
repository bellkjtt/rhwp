//! HWP3 자연 쪽 경계를 같은 HWPX 저장 조판 정보에만 결부한다.
//!
//! 표준 pageBreak로 방출하면 한컴 재조판에 강제 개행이 추가된다. 기존 출처
//! 마커 안에 보조 정보를 두고, 글자/문단 속성 또는 구역 XML이 바뀌면 버린다.
use super::{
    document::{Document, Section},
    paragraph::ColumnBreakType,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const PREFIX: &[u8] = b"2:natural-page-ownership\n";

pub(crate) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Deserialize, Serialize)]
struct SectionOwnership {
    xml_sha256: String,
    paragraph_count: usize,
    natural_breaks: Vec<usize>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PageOwnership {
    header_sha256: String,
    sections: Vec<SectionOwnership>,
}

impl PageOwnership {
    pub(crate) fn for_document(doc: &Document, header: &[u8]) -> Option<Self> {
        doc.layout_profile().hwp3_native_layout().then(|| Self {
            header_sha256: digest(header),
            sections: Vec::with_capacity(doc.sections.len()),
        })
    }

    pub(crate) fn push_section(&mut self, section: &Section, xml: &[u8]) {
        self.sections.push(SectionOwnership {
            xml_sha256: digest(xml),
            paragraph_count: section.paragraphs.len(),
            natural_breaks: section
                .paragraphs
                .iter()
                .enumerate()
                .filter_map(|(index, para)| {
                    (para.page_break_synthesized && para.column_type == ColumnBreakType::Page)
                        .then_some(index)
                })
                .collect(),
        });
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, serde_json::Error> {
        let mut bytes = PREFIX.to_vec();
        serde_json::to_writer(&mut bytes, self)?;
        Ok(bytes)
    }

    pub(crate) fn decode(marker: &[u8], header: &[u8], section_count: usize) -> Option<Self> {
        // 옛 b"1" 마커의 계보 의미는 호출자가 유지한다. 알 수 없는 버전이나
        // 지나치게 큰 보조 데이터 때문에 본문 파싱을 실패시키지 않는다.
        if marker.len() > 1024 * 1024 {
            return None;
        }
        let value: Self = serde_json::from_slice(marker.strip_prefix(PREFIX)?).ok()?;
        (value.header_sha256 == digest(header) && value.sections.len() == section_count)
            .then_some(value)
    }

    pub(crate) fn restore_section(&self, index: usize, xml_digest: &str, section: &mut Section) {
        let Some(saved) = self.sections.get(index) else {
            return;
        };
        if saved.xml_sha256 != xml_digest
            || saved.paragraph_count != section.paragraphs.len()
            || !saved
                .natural_breaks
                .windows(2)
                .all(|pair| pair[0] < pair[1])
            || saved.natural_breaks.iter().any(|&index| {
                section.paragraphs.get(index).is_none_or(|para| {
                    para.column_type != ColumnBreakType::None || para.page_break_synthesized
                })
            })
        {
            return;
        }
        for &index in &saved.natural_breaks {
            let para = &mut section.paragraphs[index];
            para.column_type = ColumnBreakType::Page;
            para.page_break_synthesized = true;
        }
    }
}

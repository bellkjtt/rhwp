//! 저장 TAC 진입에 필요한 확정 문단 내용 하단·소유 조회.
use super::*;

impl TypesetState {
    /// 저장 TAC 줄 수용에 필요한 읽기 전용 페이지 관측값.
    pub(in crate::renderer::typeset) fn stored_tac_page(
        &self,
        paragraphs: &[crate::model::paragraph::Paragraph],
    ) -> StoredTacPage {
        StoredTacPage {
            profile: self.data.profile,
            current_height: self.data.current_height,
            vpos_col_anchor: self.data.vpos_col_anchor,
            vpos_page_base: self.data.vpos_page_base,
            vpos_lazy_base: self.data.vpos_lazy_base,
            side_wrap_empty: self.data.side_wrap_exclusions.is_empty(),
            previous_content_bottom: self.data.current_items.last().and_then(|item| {
                let (owner, end_line) = match item {
                    PageItem::FullParagraph { para_index } => {
                        (*para_index, paragraphs.get(*para_index)?.line_segs.len())
                    }
                    PageItem::PartialParagraph {
                        para_index,
                        end_line,
                        ..
                    } => (*para_index, *end_line),
                    _ => return None,
                };
                self.data
                    .paragraph_fragment_content_bottoms
                    .get(&(owner, end_line))
                    .map(|bottom| (owner, end_line, *bottom))
            }),
            // layout의 단 초기 base와 같은 첫 완전한 표의 저장 줄을 읽는다.
            // PartialTable에는 소비된 컷이 있고, 문단 시작의 들여쓰기·앞 간격은
            // 별도 계약이므로 이 표 원점 조회에 섞지 않는다.
            stored_table_column_base: self.data.current_items.first().and_then(|item| {
                let PageItem::Table { para_index, .. } = item else {
                    return None;
                };
                paragraphs
                    .get(*para_index)
                    .and_then(|para| para.line_segs.first())
                    .filter(|seg| {
                        seg.tag & crate::model::paragraph::LineSeg::TAG_IMPLEMENTATION_PROPERTY == 0
                    })
                    .map(|seg| seg.vertical_pos)
            }),
        }
    }
}

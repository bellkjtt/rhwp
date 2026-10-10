//! 줄 경계에서 소비하는 구분 공백과 후행 개체 앞의 작성된 공백을 구분한다.

use std::collections::BTreeSet;

use super::{Atom, InlineFlowBox, InlineFlowContent};

pub(super) fn has_flow_content(row: &[InlineFlowBox], chars: &[char]) -> bool {
    row.iter().any(|item| match &item.content {
        InlineFlowContent::Text { range, .. } => {
            chars[range.clone()].iter().any(|ch| !ch.is_whitespace())
        }
        InlineFlowContent::Table { .. } | InlineFlowContent::Picture { .. } => true,
        InlineFlowContent::FloatingTable { .. } => false,
    })
}

pub(super) fn classify(atoms: &[Atom], chars: &[char]) -> (BTreeSet<usize>, BTreeSet<usize>) {
    let mut separators = BTreeSet::new();
    let mut object_separators = BTreeSet::new();
    let mut followed_by_flow = false;
    let mut followed_by_object = false;
    for atom in atoms.iter().rev() {
        match atom {
            Atom::Box(InlineFlowBox {
                content: InlineFlowContent::Text { range, .. },
                ..
            }) if !range.is_empty() && chars[range.clone()].iter().all(|ch| *ch == ' ') => {
                if followed_by_flow {
                    separators.insert(range.start);
                }
                if followed_by_object {
                    object_separators.insert(range.start);
                }
            }
            Atom::Box(InlineFlowBox {
                content: InlineFlowContent::Table { .. } | InlineFlowContent::Picture { .. },
                ..
            }) => {
                followed_by_flow = true;
                followed_by_object = true;
            }
            Atom::Box(InlineFlowBox {
                content: InlineFlowContent::Text { .. },
                ..
            }) => {
                followed_by_flow = true;
                followed_by_object = false;
            }
            // 부동 앵커와 명시적 개행은 자기 줄을 소유한다.
            _ => {
                followed_by_flow = false;
                followed_by_object = false;
            }
        }
    }
    (separators, object_separators)
}

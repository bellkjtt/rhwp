//! 한컴 rm/it 전환을 현재 글줄·그룹의 범위로 묶는다.
//!
//! 공백은 그대로 보존하고, 그룹 내부 전환은 바깥 형제 요소에 전파하지 않는다.
//! LaTeX의 명시적 인자형 FontStyle은 기존의 인자 범위를 유지한다.

use super::ast::EqNode;
use super::symbols::FontStyleKind;

pub(super) fn resolve(children: Vec<EqNode>) -> Vec<EqNode> {
    let mut result = Vec::new();
    let mut style = None;
    let mut run = Vec::new();
    for child in children {
        if let EqNode::FontStyle { style: next, body } = &child {
            if matches!(next, FontStyleKind::Roman | FontStyleKind::Italic)
                && matches!(body.as_ref(), EqNode::Empty)
            {
                flush(&mut result, &mut run, style);
                style = Some(*next);
                continue;
            }
        }
        run.push(child);
    }
    flush(&mut result, &mut run, style);
    result
}

fn flush(result: &mut Vec<EqNode>, run: &mut Vec<EqNode>, style: Option<FontStyleKind>) {
    if run.is_empty() {
        return;
    }
    let Some(style) = style else {
        result.append(run);
        return;
    };
    let body = if run.len() == 1 {
        run.pop().unwrap()
    } else {
        EqNode::Row(std::mem::take(run))
    };
    result.push(EqNode::FontStyle {
        style,
        body: Box::new(body),
    });
}

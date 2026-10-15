//! 그림의 시각 일치가 아닌 SVG 자원 직렬화 계약이다. 실제 문서는 별도 Print 대조한다.
#![cfg(not(target_arch = "wasm32"))]

// 제품 API를 넓히지 않고 실제 직렬화 구현을 그대로 검사한다.
#[allow(dead_code)]
#[path = "../../src/wmf/converter/svg/node.rs"]
mod node;
mod wmf {
    pub mod imports {
        pub use std::collections::BTreeMap;
    }
}

use node::{share_repeated_bitmaps, Node};

fn image(x: i32, width: i32) -> Node {
    Node::new("image")
        .set("href", "data:image/png;base64,AQID")
        .set("width", width)
        .set("height", 30)
        .set("x", x)
        .set("y", 7)
}

#[test]
fn wmf_repeated_png_serialization_keeps_placement_order_clip_and_ids() {
    let first = image(11, 20).set("id", "elem1");
    let second = image(31, 20)
        .set("id", "elem2")
        .set("transform", "scale(-1,1)")
        .set("clip-path", "url(#defs0)");
    let (defs, elements) = share_repeated_bitmaps(vec![], vec![first, second]);
    assert_eq!(defs.len(), 1);
    let definition = defs[0].to_string();
    assert!(definition.contains("width=\"20\""));
    assert!(definition.contains("height=\"30\""));
    assert!(definition.contains("data:image/png;base64,AQID"));
    let output: Vec<_> = elements.iter().map(ToString::to_string).collect();
    assert!(output[0].contains("<use "));
    assert!(output[0].contains("id=\"elem1\""));
    assert!(output[0].contains("x=\"11\""));
    assert!(output[1].contains("id=\"elem2\""));
    assert!(output[1].contains("x=\"31\""));
    assert!(output[1].contains("clip-path=\"url(#defs0)\""));
    assert!(output[1].contains("transform=\"scale(-1,1)\""));
    assert!(output.iter().all(|text| text.contains("href=\"#wmf-bitmap0\"")));
}

#[test]
fn wmf_png_serialization_does_not_merge_distinct_size_aspect_or_extra_attributes() {
    let inputs = vec![
        image(1, 20),
        image(2, 40),
        image(3, 20).set("preserveAspectRatio", "none"),
        image(4, 20).set("opacity", "0.5"),
    ];
    let before: Vec<_> = inputs.iter().map(ToString::to_string).collect();
    let (defs, after) = share_repeated_bitmaps(vec![], inputs);
    assert!(defs.is_empty());
    assert_eq!(before, after.iter().map(ToString::to_string).collect::<Vec<_>>());
}

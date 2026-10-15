use super::{Node, NodeType};
use std::collections::BTreeMap;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BitmapKey {
    uri: String,
    width: String,
    height: String,
    aspect: Option<String>,
}

impl BitmapKey {
    fn from_node(node: &Node) -> Option<Self> {
        if !matches!(&node.typ, NodeType::Node(name) if name == "image")
            || !node.inner.is_empty()
            || node.attrs.keys().any(|name| {
                !matches!(
                    name.as_str(),
                    "href"
                        | "width"
                        | "height"
                        | "preserveAspectRatio"
                        | "x"
                        | "y"
                        | "id"
                        | "transform"
                        | "clip-path"
                )
            })
        {
            return None;
        }
        let uri = node.attrs.get("href")?;
        if !uri.starts_with("data:image/png;base64,") {
            return None;
        }
        Some(Self {
            uri: uri.clone(),
            width: node.attrs.get("width")?.clone(),
            height: node.attrs.get("height")?.clone(),
            aspect: node.attrs.get("preserveAspectRatio").cloned(),
        })
    }

    fn definition(&self, id: &str) -> Node {
        let mut image = Node::new("image")
            .set("id", id)
            .set("href", &self.uri)
            .set("width", &self.width)
            .set("height", &self.height);
        if let Some(aspect) = &self.aspect {
            image = image.set("preserveAspectRatio", aspect);
        }
        image
    }
}

fn count_bitmaps(node: &Node, counts: &mut BTreeMap<BitmapKey, usize>) {
    if let Some(key) = BitmapKey::from_node(node) {
        *counts.entry(key).or_default() += 1;
    }
    for child in &node.inner {
        count_bitmaps(child, counts);
    }
}

fn replace_bitmaps(node: &mut Node, references: &BTreeMap<BitmapKey, String>) {
    if let Some(id) = BitmapKey::from_node(node).and_then(|key| references.get(&key)) {
        // SVG2 5.5: image 참조에는 use의 width/height가 적용되지 않는다.
        // 크기·종횡비는 정의에, 이동·변환·클립·식별자는 원래 자리에 둔다.
        node.typ = NodeType::Node("use".into());
        for name in ["href", "width", "height", "preserveAspectRatio"] {
            node.attrs.remove(name);
        }
        node.attrs.insert("href".into(), format!("#{id}"));
    }
    for child in &mut node.inner {
        replace_bitmaps(child, references);
    }
}

/// 동일 표시 계약의 PNG만 공유한다. 서로 다른 크기·종횡비·추가 속성은 보존한다.
pub(crate) fn share_repeated_bitmaps(
    mut definitions: Vec<Node>,
    mut elements: Vec<Node>,
) -> (Vec<Node>, Vec<Node>) {
    let mut counts = BTreeMap::new();
    for element in &elements {
        count_bitmaps(element, &mut counts);
    }
    let mut references = BTreeMap::new();
    for (key, count) in counts {
        if count > 1 {
            let id = format!("wmf-bitmap{}", references.len());
            definitions.push(key.definition(&id));
            references.insert(key, id);
        }
    }
    for element in &mut elements {
        replace_bitmaps(element, &references);
    }
    (definitions, elements)
}

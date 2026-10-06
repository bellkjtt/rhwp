//! 시스템 글꼴은 파일명 대신 내부 family·굵기로 조회한다.

use std::path::PathBuf;
use std::sync::OnceLock;
use usvg::fontdb::{Database, Family, Query, Source, Weight};

pub(super) fn family_file(names: &[String], bold: bool) -> Option<PathBuf> {
    static FONTS: OnceLock<Database> = OnceLock::new();
    let fonts = FONTS.get_or_init(|| {
        let mut fonts = Database::new();
        fonts.load_system_fonts();
        fonts
    });
    // family 별 우선순위는 호출자가 정한다. generic 폴백을 요청하지 않아
    // 없는 이름을 무관한 시스템 글꼴로 잘못 내장하지 않는다.
    for name in names {
        let families = [Family::Name(name)];
        let Some(id) = fonts.query(&Query {
            families: &families,
            weight: if bold { Weight::BOLD } else { Weight::NORMAL },
            ..Default::default()
        }) else {
            continue;
        };
        let face = fonts.face(id)?;
        // Regular만 있는 family를 Bold face로 선언하지 않는다.
        // TTC의 face 선택은 기존 파일 임베딩 계약 밖이므로 여기서 확대하지 않는다.
        if (bold && face.weight < Weight::SEMIBOLD) || face.index != 0 {
            continue;
        }
        match &face.source {
            Source::File(path) | Source::SharedFile(path, _) if path.is_file() => {
                return Some(path.clone());
            }
            _ => {}
        }
    }
    None
}

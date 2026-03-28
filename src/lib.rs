pub mod eomi;
pub mod join;
pub mod merge;
pub mod syllable;
pub mod types;
pub mod yongeon;

pub use eomi::EomiGroup;
pub use types::{IrregularType, YongeonType};
pub use yongeon::Yongeon;

include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));

/// 용언 목록에서 기본형이 일치하는 용언을 찾는다.
/// 동음이의어가 있을 수 있으므로 Vec으로 반환한다.
pub fn find_yongeon<'a>(yongeons: &'a [Yongeon<'static>], word: &str) -> Vec<&'a Yongeon<'static>> {
    yongeons.iter().filter(|y| y.base_form == word).collect()
}

/// 용언 목록에서 어간이 일치하는 용언을 찾는다.
/// 동음이의어가 있을 수 있으므로 Vec으로 반환한다.
pub fn find_eogan<'a>(yongeons: &'a [Yongeon<'static>], eogan: &str) -> Vec<&'a Yongeon<'static>> {
    yongeons.iter().filter(|y| y.eogan_str() == eogan).collect()
}

/// 용언에 어미 그룹을 적용하여 활용형을 반환합니다.
///
/// `join` 모듈로 어미를 선택·접합한 뒤, `merge` 모듈로 음운 축약을 적용합니다.
pub fn postfix(yongeon: &Yongeon, group: &EomiGroup) -> String {
    let joined = join::select(yongeon, group);
    merge::apply(yongeon, &joined, group)
}

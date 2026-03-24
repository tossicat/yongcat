pub mod syllable;
pub mod types;
pub mod yongeon;

pub use types::{IrregularType, YongeonType};
pub use yongeon::Yongeon;

include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));

/// 용언 목록에서 기본형이 일치하는 용언을 찾는다.
/// 동음이의어가 있을 수 있으므로 Vec으로 반환한다.
pub fn find_yongeon<'a>(yongeons: &'a [Yongeon<'static>], word: &str) -> Vec<&'a Yongeon<'static>> {
    yongeons.iter().filter(|y| y.base_form == word).collect()
}

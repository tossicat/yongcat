//! ## yongcat
//! 한국어 용언(동사/형용사)의 활용형을 생성하는 라이브러리입니다.
//!
//! `data/yong_list.csv`에서 빌드 시 용언 목록을 가져오고,
//! 어미 그룹(`Eomi`)을 지정하여 활용형을 생성합니다.
//!
//! 주요 흐름: 용언 검색(`find_yongeon`) → 활용형 생성(`postfix` / `postfix_word`)
//!
//! 기본적으로 사용자가 제시한 용언에서 어간을 분리한 다음, 사용자가 제시한 어미를 그 어간에 맞게
//! 적절하게 바꿔서 이를 합쳐서 반환합니다.

pub mod eomi;
pub mod irregular;
pub mod join;
pub mod merge;
pub mod syllable;
pub mod types;
pub mod yongeon;

pub use eomi::Eomi;
pub use types::{IrregularType, YongeonType};
pub use yongeon::Yongeon;

include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));

/// 용언 목록에서 기본형이 일치하는 용언을 찾습니다.
/// 동음이의어가 있을 수 있으므로 `Vec`으로 용언 둘 이상을 반환합니다.
/// 물론 하나도 `Vec`으로 반환합니다.
pub fn find_yongeon<'a>(yongeons: &'a [Yongeon<'static>], word: &str) -> Vec<&'a Yongeon<'static>> {
    yongeons.iter().filter(|y| y.base_form == word).collect()
}

/// 용언 목록에서 어간이 일치하는 용언을 찾습니다.
/// 동음이의어가 있을 수 있으므로 `Vec`으로 반환합니다.
/// 물론 하나도 `Vec`으로 반환합니다.
pub fn find_eogan<'a>(yongeons: &'a [Yongeon<'static>], eogan: &str) -> Vec<&'a Yongeon<'static>> {
    yongeons.iter().filter(|y| y.eogan_str() == eogan).collect()
}

/// 단어 문자열로 용언을 찾아 어미를 적용하고, 동음이의어별 활용형을 반환합니다.
///
/// 동음이의어가 여러 개이면 각각의 `(Yongeon, 활용형)` 쌍을 모두 반환합니다.
/// 특정 용언 하나에 대해서만 활용형을 구하려면 `postfix_word`를 사용합니다.
pub fn postfix<'a>(
    yongeons: &'a [Yongeon<'static>],
    word: &str,
    eomi: &Eomi,
) -> Vec<(&'a Yongeon<'static>, String)> {
    find_yongeon(yongeons, word)
        .into_iter()
        .map(|y| (y, postfix_word(y, eomi)))
        .collect()
}

/// 단일 용언에 어미를 적용하여 활용형을 반환합니다.
///
/// `join` 모듈로 어미를 선택·접합한 뒤, `merge` 모듈로 음운 축약을 적용합니다.
pub fn postfix_word(yongeon: &Yongeon, eomi: &Eomi) -> String {
    let joined = join::select(yongeon, eomi);
    merge::apply(yongeon, &joined, eomi)
}

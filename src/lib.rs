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
pub use eomi::ah_eo::*;
pub use eomi::fixed::*;
pub use eomi::plain::*;
pub use types::{IrregularType, YongeonType};
pub use yongeon::Yongeon;

use std::sync::LazyLock;

/// 전역 용언 데이터입니다. 최초 접근 시 한 번만 로드됩니다.
static YONGEONS: LazyLock<Vec<Yongeon<'static>>> = LazyLock::new(load_yongeons);

/// 전역 어미 데이터입니다. 최초 접근 시 한 번만 로드됩니다.
static EOMIS: LazyLock<Vec<(&'static str, &'static Eomi)>> = LazyLock::new(load_eomis);

include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));
include!(concat!(env!("OUT_DIR"), "/eomi_data.rs"));

/// 어미 목록에서 형태가 일치하는 어미를 찾습니다.
/// 하나의 어미가 여러 형태를 가질 수 있으므로 `Vec`으로 반환합니다.
pub fn find_eomi<'a>(
    eomis: &'a [(&'static str, &'static Eomi)],
    s: &str,
) -> Vec<(&'static str, &'a Eomi)> {
    eomis
        .iter()
        .filter(|(_, eomi)| eomi.matches(s))
        .map(|(name, eomi)| (*name, *eomi))
        .collect()
}

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
    let merged = merge::apply(yongeon, &joined, eomi);
    syllable::combine_jamo(&merged)
}

/// 기본형으로 용언을 검색합니다. 첫 번째 일치하는 용언을 반환합니다.
///
/// 동음이의어를 모두 얻으려면 `lookup_all()`을 사용합니다.
///
/// # Panics
/// 일치하는 용언이 없으면 패닉합니다.
pub fn lookup(word: &str) -> &'static Yongeon<'static> {
    lookup_all(word)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("용언을 찾을 수 없습니다: \"{}\"", word))
}

/// 기본형으로 용언을 검색합니다. 동음이의어를 포함한 모든 일치 결과를 반환합니다.
pub fn lookup_all(word: &str) -> Vec<&'static Yongeon<'static>> {
    find_yongeon(&YONGEONS, word)
}

/// 용언에 어미를 적용하여 활용형을 반환합니다. `postfix_word`의 편의 별칭입니다.
pub fn conjugate(yongeon: &Yongeon, eomi: &Eomi) -> String {
    postfix_word(yongeon, eomi)
}

/// 어미 형태 문자열로 어미를 검색합니다. 정확히 일치하는 첫 번째 어미를 반환합니다.
///
/// 예: `find_eomi_exact("세요")` → `Some(&EUSEYO)`
pub fn find_eomi_exact(s: &str) -> Option<&'static Eomi> {
    EOMIS
        .iter()
        .find(|(_, eomi)| eomi.matches(s))
        .map(|(_, eomi)| *eomi)
}

/// 동사 전용 어미 목록입니다. 형용사에 적용하면 문법적으로 잘못된 활용형이 생성됩니다.
const VERB_ONLY_EOMIS: &[&Eomi] = &[
    &ABODA, &ADALLA, &AJUDA, &NEUN, &JA, &NEUNDE,
    &EURYEOGO, &EUREO, &EULGE, &EULLAE, &EUPSIDA, &NEUNDA,
];

/// 주어진 어미가 동사 전용인지 확인합니다.
fn is_verb_only(eomi: &Eomi) -> bool {
    VERB_ONLY_EOMIS.contains(&eomi)
}

/// 품사 제한을 검사하여 활용형을 생성합니다.
///
/// 형용사에 동사 전용 어미를 적용하면 `Err`을 반환합니다.
/// 제한이 없는 조합이면 `Ok(활용형)`을 반환합니다.
///
/// ```rust
/// use yongcat::*;
///
/// // 동사 + 동사 전용 어미 → Ok
/// let verb = lookup("먹다");
/// assert!(conjugate_checked(verb, &NEUN).is_ok());
///
/// // 형용사 + 동사 전용 어미 → Err
/// let adj = lookup("예쁘다");
/// assert!(conjugate_checked(adj, &NEUN).is_err());
/// ```
pub fn conjugate_checked(yongeon: &Yongeon, eomi: &Eomi) -> Result<String, String> {
    if yongeon.is_adjective() && is_verb_only(eomi) {
        return Err(format!(
            "형용사 \"{}\"에 동사 전용 어미를 적용할 수 없습니다",
            yongeon.base_form
        ));
    }
    Ok(conjugate(yongeon, eomi))
}

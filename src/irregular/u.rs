//! ## 우불규칙
//! 어간 모음 ㅜ가 아/어 계열 어미 앞에서 어미 첫 모음으로 대체되는 불규칙 활용을 처리합니다.
//!
//! ㅡ 탈락과 같은 패턴이지만 ㅜ에 적용됩니다. 푸다 하나만 해당합니다.
//!
//! 예: 푸다 + 어요 → 퍼요 (풔요가 아님), 푸다 + 었 → 펐

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// 우불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 ㅜ를 어미 첫 모음으로 대체합니다 (ㅝ 축약이 아님).
/// join 단계에는 개입하지 않습니다 (규칙 활용과 동일).
pub(super) fn merge(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(contract(yongeon, joined)),
        _ => None,
    }
}

/// ㅜ → 어미 첫 모음 대체를 적용합니다.
fn contract(yongeon: &Yongeon, joined: &str) -> String {
    let eogan = yongeon.eogan_str();
    let eomi = &joined[eogan.len()..];

    let eomi_syllables = syllable::decompose(eomi);
    let eomi_first = &eomi_syllables[0];

    let first_char_len = eomi.chars().next().unwrap().len_utf8();
    let eomi_rest = &eomi[first_char_len..];

    let mut modified_eogan = yongeon.eogan.clone();
    let last_idx = modified_eogan.len() - 1;

    modified_eogan[last_idx].vowel = eomi_first.vowel;
    modified_eogan[last_idx].coda = eomi_first.coda;

    format!("{}{}", syllable::compose(&modified_eogan), eomi_rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn u_verb() -> Yongeon<'static> {
        Yongeon::new("푸다", "", "푸", YongeonType::Verb, IrregularType::U)
    }

    #[test]
    fn test_merge_ayo() {
        // 푸 + 어요 → 퍼요 (ㅜ→ㅓ)
        let result = merge(&u_verb(), "푸어요", &ah_eo::AYO);
        assert_eq!(result, Some("퍼요".to_string()));
    }

    #[test]
    fn test_merge_past() {
        // 푸 + 었 → 펐 (ㅜ→ㅓ, ㅆ 이전)
        let result = merge(&u_verb(), "푸었", &ah_eo::ASS);
        assert_eq!(result, Some("펐".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&u_verb(), "푸면", &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }
}

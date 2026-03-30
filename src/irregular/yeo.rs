//! ## 여불규칙
//! "하다" 계열 용언의 불규칙 활용을 처리합니다.
//!
//! 아/어 계열 어미를 만나면 "여" 형태를 선택하고,
//! 하+여 → 해 축약을 적용합니다.

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// 여불규칙의 어간-어미 결합을 처리합니다.
///
/// `AhEo` 어미이면 "여" 형태(form.2)를 선택하여 접합합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let eogan = yongeon.eogan_str();
            Some(format!("{}{}", eogan, form.2))
        }
        _ => None,
    }
}

/// 여불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 하+여 → 해 축약(ㅏ→ㅐ)을 적용합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn merge(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(apply(yongeon, joined)),
        _ => None,
    }
}

/// 하+여 → 해 축약을 적용합니다.
fn apply(yongeon: &Yongeon, joined: &str) -> String {
    let eogan = yongeon.eogan_str();
    let eomi = &joined[eogan.len()..];

    let eomi_syllables = syllable::decompose(eomi);
    let eomi_first = &eomi_syllables[0];

    let first_char_len = eomi.chars().next().unwrap().len_utf8();
    let eomi_rest = &eomi[first_char_len..];

    let mut modified_eogan = yongeon.eogan.clone();
    let last_idx = modified_eogan.len() - 1;

    modified_eogan[last_idx].vowel = 'ㅐ';
    modified_eogan[last_idx].coda = eomi_first.coda;

    format!("{}{}", syllable::compose(&modified_eogan), eomi_rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn hada() -> Yongeon<'static> {
        Yongeon::new("하다", "", "하", YongeonType::Verb, IrregularType::Yeo)
    }

    // --- join ---

    #[test]
    fn test_join_ah_eo() {
        let result = join(&hada(), &ah_eo::AYO);
        assert_eq!(result, Some("하여요".to_string()));
    }

    #[test]
    fn test_join_plain() {
        let result = join(&hada(), &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_fixed() {
        let result = join(&hada(), &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }

    // --- merge ---

    #[test]
    fn test_merge_ah_eo() {
        let result = merge(&hada(), "하여요", &ah_eo::AYO);
        assert_eq!(result, Some("해요".to_string()));
    }

    #[test]
    fn test_merge_past() {
        let result = merge(&hada(), "하였", &ah_eo::ASS);
        assert_eq!(result, Some("했".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&hada(), "하은", &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_merge_fixed() {
        let result = merge(&hada(), "하고", &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }
}

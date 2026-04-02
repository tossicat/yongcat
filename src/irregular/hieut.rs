//! ## ㅎ불규칙
//! 어간 끝 받침 ㅎ이 모음 어미 앞에서 탈락하고,
//! 아/어 계열 어미와 만나면 어간 끝 모음이 ㅐ로 축약되는 불규칙 활용을 처리합니다.
//!
//! 대상은 모두 형용사(13개)입니다.
//!
//! 예: 그렇다 + 어요 → 그래요, 노랗다 + 아요 → 노래요, 하얗다 + 으면 → 하야면

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// ㅎ불규칙의 어간-어미 결합을 처리합니다.
///
/// 모음으로 시작하는 어미 앞에서 어간 끝 ㅎ을 제거합니다.
/// 자음으로 시작하는 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let stem = stem_without_hieut(yongeon);
            let suffix = yongeon.moeum_joha(form);
            Some(format!("{}{}", stem, suffix))
        }
        Eomi::Plain(coda_form, no_coda_form) => {
            if syllable::starts_with_vowel(coda_form) {
                let stem = stem_without_hieut(yongeon);
                Some(format!("{}{}", stem, no_coda_form))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// ㅎ불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 어간 끝 모음을 ㅐ로 바꾸고 어미 첫 음절을 흡수합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn merge(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(apply(yongeon, joined)),
        _ => None,
    }
}

/// 어간 끝 ㅎ을 제거한 문자열을 반환합니다.
fn stem_without_hieut(yongeon: &Yongeon) -> String {
    let mut modified = yongeon.eogan.clone();
    let last_idx = modified.len() - 1;
    modified[last_idx].coda = None;
    syllable::compose(&modified)
}

/// 어간 끝 모음 → ㅐ 축약을 적용합니다.
///
/// ㅎ 탈락 후 어간 끝 모음(ㅏ 또는 ㅓ)이 어미 첫 모음(아/어)을 흡수하면서
/// ㅐ로 바뀝니다. 어미 첫 음절의 받침이 있으면 함께 흡수합니다.
fn apply(yongeon: &Yongeon, joined: &str) -> String {
    let eogan = yongeon.eogan_str();
    let eomi = &joined[eogan.len()..];

    let eomi_syllables = syllable::decompose(eomi);
    let eomi_first = &eomi_syllables[0];

    let first_char_len = eomi.chars().next().unwrap().len_utf8();
    let eomi_rest = &eomi[first_char_len..];

    let mut modified_eogan = yongeon.eogan.clone();
    let last_idx = modified_eogan.len() - 1;

    modified_eogan[last_idx].vowel = match modified_eogan[last_idx].vowel {
        'ㅑ' => 'ㅒ', // 하얗 → 하얘
        _ => 'ㅐ',    // 그렇 → 그래, 노랗 → 노래
    };
    modified_eogan[last_idx].coda = eomi_first.coda;

    format!("{}{}", syllable::compose(&modified_eogan), eomi_rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn hieut_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Hieut)
    }

    // --- join ---

    #[test]
    fn test_join_negative() {
        // 그렇다: ㅓ → 음성 → 그러 + 어요
        let result = join(&hieut_adj("그렇다", "그렇"), &ah_eo::AYO);
        assert_eq!(result, Some("그러어요".to_string()));
    }

    #[test]
    fn test_join_positive() {
        // 노랗다: ㅏ → 양성 → 노라 + 아요
        let result = join(&hieut_adj("노랗다", "노랗"), &ah_eo::AYO);
        assert_eq!(result, Some("노라아요".to_string()));
    }

    #[test]
    fn test_join_past() {
        // 그렇다: ㅎ 탈락 → 그러 + 었
        let result = join(&hieut_adj("그렇다", "그렇"), &ah_eo::ASS);
        assert_eq!(result, Some("그러었".to_string()));
    }

    #[test]
    fn test_join_plain_vowel() {
        // 그렇다 + 으면/면 → 그러면 (모음 시작 → ㅎ 탈락, 무받침 형태 선택)
        let result = join(&hieut_adj("그렇다", "그렇"), &Eomi::Plain("으면", "면"));
        assert_eq!(result, Some("그러면".to_string()));
    }

    #[test]
    fn test_join_plain_consonant() {
        // 그렇다 + 습니다 → 개입 없음 (자음 시작)
        let result = join(&hieut_adj("그렇다", "그렇"), &Eomi::Plain("습니다", "ㅂ니다"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_fixed() {
        // 그렇다 + 고 → 개입 없음
        let result = join(&hieut_adj("그렇다", "그렇"), &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }

    // --- merge ---

    #[test]
    fn test_merge_negative() {
        // 그러 + 어요 → 그래요
        let result = merge(&hieut_adj("그렇다", "그렇"), "그러어요", &ah_eo::AYO);
        assert_eq!(result, Some("그래요".to_string()));
    }

    #[test]
    fn test_merge_positive() {
        // 노라 + 아요 → 노래요
        let result = merge(&hieut_adj("노랗다", "노랗"), "노라아요", &ah_eo::AYO);
        assert_eq!(result, Some("노래요".to_string()));
    }

    #[test]
    fn test_merge_past() {
        // 그러 + 었 → 그랬
        let result = merge(&hieut_adj("그렇다", "그렇"), "그러었", &ah_eo::ASS);
        assert_eq!(result, Some("그랬".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&hieut_adj("그렇다", "그렇"), "그러면", &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_merge_fixed() {
        let result = merge(&hieut_adj("그렇다", "그렇"), "그렇고", &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }
}

//! ## 르불규칙
//! 어간 끝 르가 모음 어미 앞에서 분리되어,
//! ㄹ이 앞 음절 받침으로 삽입되고 어미 첫 초성이 ㄹ로 바뀌는 불규칙 활용을 처리합니다.
//!
//! 동사(25개)와 형용사(11개) 모두 해당합니다.
//!
//! 예: 모르다 + 아요 → 몰라요, 기르다 + 어요 → 길러요, 떠오르다 + 아요 → 떠올라요

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// 르불규칙의 어간-어미 결합을 처리합니다.
///
/// `AhEo` 어미이면 어간 끝 르를 분리하여 앞 음절에 ㄹ 받침을 삽입하고,
/// 어미 첫 음절 초성을 ㄹ로 교체합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let stem = stem_with_rieul(yongeon);
            let suffix = yongeon.moeum_joha(form);
            let modified_suffix = replace_onset_with_rieul(suffix);
            Some(format!("{}{}", stem, modified_suffix))
        }
        _ => None,
    }
}

/// 르불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 축약 없이 그대로 반환합니다 (축약 억제).
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn merge(_yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(joined.to_string()),
        _ => None,
    }
}

/// 어간 끝 르를 제거하고 앞 음절에 ㄹ 받침을 삽입한 문자열을 반환합니다.
///
/// 예: 모르 → 몰, 떠오르 → 떠올
fn stem_with_rieul(yongeon: &Yongeon) -> String {
    let mut modified = yongeon.eogan[..yongeon.eogan.len() - 1].to_vec();
    let last_idx = modified.len() - 1;
    modified[last_idx].coda = Some('ㄹ');
    syllable::compose(&modified)
}

/// 어미 첫 음절의 초성을 ㄹ로 교체한 문자열을 반환합니다.
///
/// 예: 아요 → 라요, 었 → 렀
fn replace_onset_with_rieul(suffix: &str) -> String {
    let mut syllables = syllable::decompose(suffix);
    syllables[0].onset = 'ㄹ';
    let first = syllable::compose(&syllables[..1]);
    let first_char_len = suffix.chars().next().unwrap().len_utf8();
    format!("{}{}", first, &suffix[first_char_len..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn reu_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Reu)
    }

    fn reu_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Reu)
    }

    // --- join ---

    #[test]
    fn test_join_positive() {
        // 모르다: 앞 음절 모(ㅗ, 양성) → 몰 + 라요
        let result = join(&reu_verb("모르다", "모르"), &ah_eo::AYO);
        assert_eq!(result, Some("몰라요".to_string()));
    }

    #[test]
    fn test_join_negative() {
        // 기르다: 앞 음절 기(ㅣ, 음성) → 길 + 러요
        let result = join(&reu_verb("기르다", "기르"), &ah_eo::AYO);
        assert_eq!(result, Some("길러요".to_string()));
    }

    #[test]
    fn test_join_past() {
        // 모르다 + 았 → 몰랐
        let result = join(&reu_verb("모르다", "모르"), &ah_eo::ASS);
        assert_eq!(result, Some("몰랐".to_string()));
    }

    #[test]
    fn test_join_multi_syllable() {
        // 떠오르다: 앞 음절 오(ㅗ, 양성) → 떠올 + 라요
        let result = join(&reu_verb("떠오르다", "떠오르"), &ah_eo::AYO);
        assert_eq!(result, Some("떠올라요".to_string()));
    }

    #[test]
    fn test_join_adj() {
        // 빠르다: 앞 음절 빠(ㅏ, 양성) → 빨 + 라요
        let result = join(&reu_adj("빠르다", "빠르"), &ah_eo::AYO);
        assert_eq!(result, Some("빨라요".to_string()));
    }

    #[test]
    fn test_join_plain() {
        // 모르다 + 으면/면 → 개입 없음 (무받침 → 면)
        let result = join(&reu_verb("모르다", "모르"), &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_fixed() {
        // 모르다 + 고 → 개입 없음
        let result = join(&reu_verb("모르다", "모르"), &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }

    // --- merge (축약 억제) ---

    #[test]
    fn test_merge_suppresses_contraction() {
        // 몰라요 → 몰라요 (그대로)
        let result = merge(&reu_verb("모르다", "모르"), "몰라요", &ah_eo::AYO);
        assert_eq!(result, Some("몰라요".to_string()));
    }

    #[test]
    fn test_merge_past() {
        // 몰랐 → 몰랐 (그대로)
        let result = merge(&reu_verb("모르다", "모르"), "몰랐", &ah_eo::ASS);
        assert_eq!(result, Some("몰랐".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&reu_verb("모르다", "모르"), "모르면", &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_merge_fixed() {
        let result = merge(&reu_verb("모르다", "모르"), "모르고", &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }
}

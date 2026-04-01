//! ## ㅅ불규칙
//! 어간 끝 받침 ㅅ이 모음 어미 앞에서 탈락하는 불규칙 활용을 처리합니다.
//!
//! 규칙 활용과 달리 모음 축약이 적용되지 않습니다.
//!
//! 예: 짓다 + 어요 → 지어요 (져요가 아님), 낫다 + 아요 → 나아요 (나요가 아님)

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// ㅅ불규칙의 어간-어미 결합을 처리합니다.
///
/// 모음으로 시작하는 어미 앞에서 어간 끝 ㅅ을 제거합니다.
/// 자음으로 시작하는 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let stem = stem_without_siot(yongeon);

            let suffix = yongeon.moeum_joha(form);

            Some(format!("{}{}", stem, suffix))
        }
        Eomi::Plain(coda_form, no_coda_form) => {
            if syllable::starts_with_vowel(coda_form) {
                let stem = stem_without_siot(yongeon);
                Some(format!("{}{}", stem, no_coda_form))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// 어간 끝 ㅅ을 제거한 문자열을 반환합니다.
fn stem_without_siot(yongeon: &Yongeon) -> String {
    let mut modified = yongeon.eogan.clone();
    let last_idx = modified.len() - 1;
    modified[last_idx].coda = None;
    syllable::compose(&modified)
}

/// ㅅ불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 축약 없이 그대로 반환합니다 (축약 억제).
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn merge(_yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(joined.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn siot_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Siot)
    }

    fn siot_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Siot)
    }

    // --- join ---

    #[test]
    fn test_join_negative() {
        // 짓다: ㅅ 탈락 → 지 + 어요
        let result = join(&siot_verb("짓다", "짓"), &ah_eo::AYO);
        assert_eq!(result, Some("지어요".to_string()));
    }

    #[test]
    fn test_join_positive() {
        // 낫다: ㅅ 탈락 → 나 + 아요
        let result = join(&siot_adj("낫다", "낫"), &ah_eo::AYO);
        assert_eq!(result, Some("나아요".to_string()));
    }

    #[test]
    fn test_join_past() {
        // 짓다: ㅅ 탈락 → 지 + 었
        let result = join(&siot_verb("짓다", "짓"), &ah_eo::ASS);
        assert_eq!(result, Some("지었".to_string()));
    }

    #[test]
    fn test_join_plain_vowel() {
        // 짓다 + 으면/면 → 지면 (모음 시작 → ㅅ 탈락, 무받침 형태 선택)
        let result = join(&siot_verb("짓다", "짓"), &Eomi::Plain("으면", "면"));
        assert_eq!(result, Some("지면".to_string()));
    }

    #[test]
    fn test_join_plain_consonant() {
        // 짓다 + 습니다 → 개입 없음 (자음 시작)
        let result = join(&siot_verb("짓다", "짓"), &Eomi::Plain("습니다", "ㅂ니다"));
        assert_eq!(result, None);
    }

    // --- merge (축약 억제) ---

    #[test]
    fn test_merge_no_contraction() {
        // 지어요 → 지어요 (져요가 아님)
        let result = merge(&siot_verb("짓다", "짓"), "지어요", &ah_eo::AYO);
        assert_eq!(result, Some("지어요".to_string()));
    }

    #[test]
    fn test_merge_no_contraction_positive() {
        // 나아요 → 나아요 (나요가 아님)
        let result = merge(&siot_adj("낫다", "낫"), "나아요", &ah_eo::AYO);
        assert_eq!(result, Some("나아요".to_string()));
    }

    #[test]
    fn test_merge_past_no_contraction() {
        // 지었 → 지었 (축약 없이 그대로)
        let result = merge(&siot_verb("짓다", "짓"), "지었", &ah_eo::ASS);
        assert_eq!(result, Some("지었".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&siot_verb("짓다", "짓"), "짓은", &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }
}

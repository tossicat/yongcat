//! ## 러불규칙
//! 아/어 계열 어미의 "어"가 "러"로 바뀌는 불규칙 활용을 처리합니다.
//!
//! 어간의 ㅡ가 탈락하지 않고 유지됩니다.
//!
//! 예: 이르다 + 어요 → 이르러요, 푸르다 + 어요 → 푸르러요

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// 음성모음 어미의 첫 음절 초성 ㅇ→ㄹ 변환으로 러 형태를 생성합니다.
///
/// 예: "어요" → "러요", "었" → "렀"
fn to_reo(eomi: &str) -> String {
    let mut syllables = syllable::decompose(eomi);
    syllables[0].onset = 'ㄹ';
    let first = syllable::compose(&syllables[..1]);
    let rest_start = eomi.chars().next().unwrap().len_utf8();
    format!("{}{}", first, &eomi[rest_start..])
}

/// 러불규칙의 어간-어미 결합을 처리합니다.
///
/// `AhEo` 어미이면 음성 형태(form.1)에서 러 형태를 생성하여 접합합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let eogan = yongeon.eogan_str();
            let reo_form = to_reo(form.1);
            Some(format!("{}{}", eogan, reo_form))
        }
        _ => None,
    }
}

/// 러불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 축약 없이 그대로 반환합니다 (ㅡ 탈락 억제).
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

    fn reo_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Reo)
    }

    // --- join ---

    #[test]
    fn test_join_ayo() {
        // 이르다 + 어요 → 이르러요
        let result = join(&reo_verb("이르다", "이르"), &ah_eo::AYO);
        assert_eq!(result, Some("이르러요".to_string()));
    }

    #[test]
    fn test_join_past() {
        // 이르다 + 었 → 이르렀
        let result = join(&reo_verb("이르다", "이르"), &ah_eo::ASS);
        assert_eq!(result, Some("이르렀".to_string()));
    }

    #[test]
    fn test_join_plain() {
        let result = join(&reo_verb("이르다", "이르"), &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }

    // --- merge (축약 억제) ---

    #[test]
    fn test_merge_no_contraction() {
        // 이르러요 → 이르러요 (ㅡ 탈락 없음)
        let result = merge(&reo_verb("이르다", "이르"), "이르러요", &ah_eo::AYO);
        assert_eq!(result, Some("이르러요".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&reo_verb("이르다", "이르"), "이르면", &Eomi::Plain("으면", "면"));
        assert_eq!(result, None);
    }
}

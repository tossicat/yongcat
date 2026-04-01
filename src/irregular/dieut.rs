//! ## ㄷ불규칙
//! 어간 끝 받침 ㄷ이 모음 어미 앞에서 ㄹ로 바뀌는 불규칙 활용을 처리합니다.
//!
//! 예: 걷다 + 어요 → 걸어요, 듣다 + 어요 → 들어요

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// ㄷ불규칙의 어간-어미 결합을 처리합니다.
///
/// 모음으로 시작하는 어미 앞에서 어간 끝 ㄷ을 ㄹ로 바꿉니다.
/// 자음으로 시작하는 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let mut modified = yongeon.eogan.clone();
            let last_idx = modified.len() - 1;
            modified[last_idx].coda = Some('ㄹ');

            let suffix = yongeon.moeum_joha(form);

            Some(format!("{}{}", syllable::compose(&modified), suffix))
        }
        Eomi::Plain(coda_form, _) => {
            if syllable::starts_with_vowel(coda_form) {
                let mut modified = yongeon.eogan.clone();
                let last_idx = modified.len() - 1;
                modified[last_idx].coda = Some('ㄹ');
                Some(format!("{}{}", syllable::compose(&modified), coda_form))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn dieut_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Dieut)
    }

    #[test]
    fn test_join_ah_eo() {
        // 걷다: 걷(ㄷ→ㄹ)→걸 + 어요
        let result = join(&dieut_verb("걷다", "걷"), &ah_eo::AYO);
        assert_eq!(result, Some("걸어요".to_string()));
    }

    #[test]
    fn test_join_past() {
        // 듣다: 듣(ㄷ→ㄹ)→들 + 었
        let result = join(&dieut_verb("듣다", "듣"), &ah_eo::ASS);
        assert_eq!(result, Some("들었".to_string()));
    }

    #[test]
    fn test_join_plain_vowel() {
        // 걷다 + 으면 → 걸으면 (모음 시작 → ㄷ→ㄹ)
        let result = join(&dieut_verb("걷다", "걷"), &Eomi::Plain("으면", "면"));
        assert_eq!(result, Some("걸으면".to_string()));
    }

    #[test]
    fn test_join_plain_consonant() {
        // 걷다 + 습니다 → 개입 없음 (자음 시작)
        let result = join(&dieut_verb("걷다", "걷"), &Eomi::Plain("습니다", "ㅂ니다"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_fixed() {
        let result = join(&dieut_verb("걷다", "걷"), &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }
}

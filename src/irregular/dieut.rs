//! ## ㄷ불규칙
//! 어간 끝 받침 ㄷ이 모음 어미 앞에서 ㄹ로 바뀌는 불규칙 활용을 처리합니다.
//!
//! 예: 걷다 + 어요 → 걸어요, 듣다 + 어요 → 들어요

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// ㄷ불규칙의 어간-어미 결합을 처리합니다.
///
/// `AhEo` 어미이면 어간 끝 ㄷ을 ㄹ로 바꾼 뒤 모음조화에 따라 접합합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            let mut modified = yongeon.eogan.clone();
            let last_idx = modified.len() - 1;
            modified[last_idx].coda = Some('ㄹ');

            let suffix = if yongeon.is_positive_vowel() {
                form.0
            } else {
                form.1
            };

            Some(format!("{}{}", syllable::compose(&modified), suffix))
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
    fn test_join_plain() {
        let result = join(&dieut_verb("걷다", "걷"), &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_fixed() {
        let result = join(&dieut_verb("걷다", "걷"), &Eomi::Fixed("고"));
        assert_eq!(result, None);
    }
}

//! ## 어간-어미 결합 모듈
//! 용언의 어간에 어미를 선택하여 단순 접합하는 기능을 제공합니다.
//!
//! 모음조화 판별(1단계)과 단순 접합(2단계)을 담당합니다.

use crate::eomi::EomiGroup;
use crate::yongeon::Yongeon;

/// 어미 그룹에서 적절한 어미를 선택하여 어간과 접합합니다.
///
/// `EomiGroup`의 종류에 따라 어미 선택 규칙이 달라집니다.
/// - `AhEo`: 모음조화에 따라 양성/음성/"하다"용 중 선택
/// - `Plain`: 받침 유무에 따라 선택
/// - `Fixed`: 고정 형태 그대로 사용
pub fn select(yongeon: &Yongeon, group: &EomiGroup) -> String {
    let eogan = yongeon.eogan_str();
    let eomi = match group {
        EomiGroup::AhEo(form) => {
            if yongeon.base_form.ends_with("하다") {
                form.2
            } else if yongeon.is_positive_vowel() {
                form.0
            } else {
                form.1
            }
        }
        EomiGroup::Plain(coda, no_coda) => {
            if yongeon.has_coda() {
                coda
            } else {
                no_coda
            }
        }
        EomiGroup::Fixed(s) => s,
    };
    format!("{}{}", eogan, eomi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::AH_EO_GROUP;
    use crate::types::{IrregularType, YongeonType};

    fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
    }

    // --- AhEo 분기 ---

    #[test]
    fn test_ah_eo_positive_vowel() {
        // 가다: ㅏ(양성) → "아요" 선택
        let ga = verb("가다", "가");
        let result = select(&ga, &EomiGroup::AhEo(AH_EO_GROUP[1]));
        assert_eq!(result, "가아요");
    }

    #[test]
    fn test_ah_eo_negative_vowel() {
        // 먹다: ㅓ(음성) → "어요" 선택
        let meok = verb("먹다", "먹");
        let result = select(&meok, &EomiGroup::AhEo(AH_EO_GROUP[1]));
        assert_eq!(result, "먹어요");
    }

    #[test]
    fn test_ah_eo_hada() {
        // 하다: "하다"용 → "여요" 선택
        let ha = verb("하다", "하");
        let result = select(&ha, &EomiGroup::AhEo(AH_EO_GROUP[1]));
        assert_eq!(result, "하여요");
    }

    // --- Plain 분기 ---

    #[test]
    fn test_plain_with_coda() {
        // 먹다: 받침 있음 → "은"
        let meok = verb("먹다", "먹");
        let result = select(&meok, &EomiGroup::Plain("은", "ㄴ"));
        assert_eq!(result, "먹은");
    }

    #[test]
    fn test_plain_without_coda() {
        // 가다: 받침 없음 → "ㄴ"
        let ga = verb("가다", "가");
        let result = select(&ga, &EomiGroup::Plain("은", "ㄴ"));
        assert_eq!(result, "가ㄴ");
    }

    // --- Fixed 분기 ---

    #[test]
    fn test_fixed() {
        let meok = verb("먹다", "먹");
        let result = select(&meok, &EomiGroup::Fixed("고"));
        assert_eq!(result, "먹고");
    }
}

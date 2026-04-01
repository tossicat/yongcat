//! ## ㅂ불규칙
//! 어간 끝 받침 ㅂ이 모음 어미 앞에서 우로 바뀌는 불규칙 활용을 처리합니다.
//!
//! 모음조화가 특수합니다: 돕다/곱다만 양성(와), 나머지는 전부 음성(워)입니다.
//!
//! 예: 돕다 + 아요 → 도와요, 춥다 + 어요 → 추워요, 가깝다 + 어요 → 가까워요

use crate::eomi::Eomi;
use crate::syllable::{self, Syllable};
use crate::yongeon::Yongeon;

/// ㅂ불규칙의 어간-어미 결합을 처리합니다.
///
/// `AhEo` 어미이면 어간 끝 ㅂ을 제거하고 우를 삽입한 뒤,
/// 특수 모음조화(돕다/곱다만 양성)에 따라 접합합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(form) => {
            // ㅂ 제거
            let mut modified = yongeon.eogan.clone();
            let last_idx = modified.len() - 1;
            modified[last_idx].coda = None;

            // 우 삽입
            let wu = Syllable { onset: 'ㅇ', vowel: 'ㅜ', coda: None };

            // 특수 모음조화: 돕다/곱다(단음절, ㅗ)만 양성
            let suffix = if is_positive_bieut(yongeon) {
                form.0
            } else {
                form.1
            };

            Some(format!(
                "{}{}{}",
                syllable::compose(&modified),
                syllable::compose(&[wu]),
                suffix
            ))
        }
        _ => None,
    }
}

/// ㅂ불규칙의 음운 축약을 처리합니다.
///
/// `AhEo` 어미이면 우+어미 첫 모음 축약(ㅜ+ㅏ→ㅘ, ㅜ+ㅓ→ㅝ)을 적용합니다.
/// `Plain`/`Fixed` 어미에는 개입하지 않습니다.
pub(super) fn merge(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => Some(contract(yongeon, joined)),
        _ => None,
    }
}

/// ㅂ불규칙 모음조화: 돕다/곱다(단음절, ㅗ)만 양성입니다.
fn is_positive_bieut(yongeon: &Yongeon) -> bool {
    yongeon.eogan.len() == 1 && yongeon.last_syllable().vowel == 'ㅗ'
}

/// 우+어미 축약을 적용합니다.
fn contract(yongeon: &Yongeon, joined: &str) -> String {
    // 변환된 어간: ㅂ 제거
    let mut modified = yongeon.eogan.clone();
    let last_idx = modified.len() - 1;
    modified[last_idx].coda = None;
    let stem = syllable::compose(&modified);

    // 우 다음이 어미
    let wu_len = '우'.len_utf8();
    let eomi_str = &joined[stem.len() + wu_len..];

    // 어미 첫 음절 분해
    let eomi_syllables = syllable::decompose(eomi_str);
    let eomi_first = &eomi_syllables[0];
    let first_char_len = eomi_str.chars().next().unwrap().len_utf8();
    let eomi_rest = &eomi_str[first_char_len..];

    // 우 + 어미 첫 모음 축약
    let new_vowel = match eomi_first.vowel {
        'ㅏ' => 'ㅘ', // ㅜ+ㅏ → ㅘ
        'ㅓ' => 'ㅝ', // ㅜ+ㅓ → ㅝ
        _ => return joined.to_string(),
    };

    let contracted = Syllable {
        onset: 'ㅇ',
        vowel: new_vowel,
        coda: eomi_first.coda,
    };

    format!("{}{}{}", stem, syllable::compose(&[contracted]), eomi_rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::types::{IrregularType, YongeonType};

    fn bieut_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Bieut)
    }

    fn bieut_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Bieut)
    }

    // --- join ---

    #[test]
    fn test_join_positive() {
        // 돕다: 단음절 ㅗ → 양성 → 도우 + 아요
        let result = join(&bieut_verb("돕다", "돕"), &ah_eo::AYO);
        assert_eq!(result, Some("도우아요".to_string()));
    }

    #[test]
    fn test_join_negative() {
        // 춥다: ㅜ → 음성 → 추우 + 어요
        let result = join(&bieut_adj("춥다", "춥"), &ah_eo::AYO);
        assert_eq!(result, Some("추우어요".to_string()));
    }

    #[test]
    fn test_join_multi_syllable() {
        // 가깝다: 다음절 → 음성 → 가까우 + 어요
        let result = join(&bieut_adj("가깝다", "가깝"), &ah_eo::AYO);
        assert_eq!(result, Some("가까우어요".to_string()));
    }

    #[test]
    fn test_join_plain() {
        let result = join(&bieut_verb("돕다", "돕"), &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }

    // --- merge ---

    #[test]
    fn test_merge_positive() {
        // 도우 + 아요 → 도와요
        let result = merge(&bieut_verb("돕다", "돕"), "도우아요", &ah_eo::AYO);
        assert_eq!(result, Some("도와요".to_string()));
    }

    #[test]
    fn test_merge_negative() {
        // 추우 + 어요 → 추워요
        let result = merge(&bieut_adj("춥다", "춥"), "추우어요", &ah_eo::AYO);
        assert_eq!(result, Some("추워요".to_string()));
    }

    #[test]
    fn test_merge_multi_syllable() {
        // 가까우 + 어요 → 가까워요
        let result = merge(&bieut_adj("가깝다", "가깝"), "가까우어요", &ah_eo::AYO);
        assert_eq!(result, Some("가까워요".to_string()));
    }

    #[test]
    fn test_merge_past() {
        // 도우 + 았 → 도왔
        let result = merge(&bieut_verb("돕다", "돕"), "도우았", &ah_eo::ASS);
        assert_eq!(result, Some("도왔".to_string()));
    }

    #[test]
    fn test_merge_plain() {
        let result = merge(&bieut_verb("돕다", "돕"), "돕은", &Eomi::Plain("은", "ㄴ"));
        assert_eq!(result, None);
    }
}

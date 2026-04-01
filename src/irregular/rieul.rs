//! ## ㄹ불규칙
//! 어간 끝 받침 ㄹ이 ㄴ/ㅂ/ㅅ 앞에서 탈락하는 불규칙 활용을 처리합니다.
//!
//! Plain 어미에서는 항상 무받침 형태를 선택합니다.
//! AhEo 어미에서는 규칙 활용과 동일하므로 개입하지 않습니다.
//!
//! 예: 살다 + 는 → 사는, 살다 + 면 → 살면, 살다 + 아요 → 살아요

use crate::eomi::Eomi;
use crate::syllable;
use crate::yongeon::Yongeon;

/// ㄹ불규칙의 어간-어미 결합을 처리합니다.
///
/// - `AhEo`: 개입 없음 (규칙 활용과 동일)
/// - `Plain`: 항상 무받침 형태 선택, ㄴ/ㅂ/ㅅ 앞에서 ㄹ 탈락
/// - `Fixed`: ㄴ/ㅂ/ㅅ 앞에서 ㄹ 탈락
pub(super) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match eomi {
        Eomi::AhEo(_) => None,
        Eomi::Plain(_, no_coda_form) => {
            if should_drop_rieul(no_coda_form) {
                let stem = stem_without_rieul(yongeon);
                Some(format!("{}{}", stem, no_coda_form))
            } else {
                let stem = yongeon.eogan_str();
                Some(format!("{}{}", stem, no_coda_form))
            }
        }
        Eomi::Fixed(s) => {
            if should_drop_rieul(s) {
                let stem = stem_without_rieul(yongeon);
                Some(format!("{}{}", stem, s))
            } else {
                None
            }
        }
    }
}

/// ㄴ/ㅂ/ㅅ으로 시작하는지 확인합니다 (자모 또는 완성형 음절).
fn should_drop_rieul(s: &str) -> bool {
    let first = s.chars().next().unwrap_or('\0');
    // 자모 (ㄴ, ㅂ, ㅅ)
    if matches!(first, 'ㄴ' | 'ㅂ' | 'ㅅ') {
        return true;
    }
    // 완성형 음절의 초성
    syllable::decompose(s)
        .first()
        .map(|syl| matches!(syl.onset, 'ㄴ' | 'ㅂ' | 'ㅅ'))
        .unwrap_or(false)
}

/// 어간 끝 ㄹ을 제거한 문자열을 반환합니다.
fn stem_without_rieul(yongeon: &Yongeon) -> String {
    let mut modified = yongeon.eogan.clone();
    let last_idx = modified.len() - 1;
    modified[last_idx].coda = None;
    syllable::compose(&modified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::{ah_eo, fixed, plain};
    use crate::types::{IrregularType, YongeonType};

    fn rieul_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Rieul)
    }

    // --- AhEo: 개입 없음 ---

    #[test]
    fn test_join_ah_eo() {
        let result = join(&rieul_verb("살다", "살"), &ah_eo::AYO);
        assert_eq!(result, None);
    }

    // --- Plain: 무받침 형태 선택 ---

    #[test]
    fn test_join_plain_eun() {
        // 살다 + 은/ㄴ → ㄹ탈락 + ㄴ → 사ㄴ
        let result = join(&rieul_verb("살다", "살"), &plain::EUN);
        assert_eq!(result, Some("사ㄴ".to_string()));
    }

    #[test]
    fn test_join_plain_eumyeon() {
        // 살다 + 으면/면 → ㄹ유지 + 면 → 살면
        let result = join(&rieul_verb("살다", "살"), &plain::EUMYEON);
        assert_eq!(result, Some("살면".to_string()));
    }

    #[test]
    fn test_join_plain_euni() {
        // 살다 + 으니/니 → ㄹ탈락 + 니 → 사니
        let result = join(&rieul_verb("살다", "살"), &plain::EUNI);
        assert_eq!(result, Some("사니".to_string()));
    }

    #[test]
    fn test_join_plain_seumnida() {
        // 살다 + 습니다/ㅂ니다 → ㄹ탈락 + ㅂ니다 → 사ㅂ니다
        let result = join(&rieul_verb("살다", "살"), &plain::SEUMNIDA);
        assert_eq!(result, Some("사ㅂ니다".to_string()));
    }

    #[test]
    fn test_join_plain_eul() {
        // 살다 + 을/ㄹ → ㄹ유지 + ㄹ → 살ㄹ
        let result = join(&rieul_verb("살다", "살"), &plain::EUL);
        assert_eq!(result, Some("살ㄹ".to_string()));
    }

    // --- Fixed: ㄴ 앞 ㄹ 탈락 ---

    #[test]
    fn test_join_fixed_neun() {
        // 살다 + 는 → ㄹ탈락 → 사는
        let result = join(&rieul_verb("살다", "살"), &fixed::NEUN);
        assert_eq!(result, Some("사는".to_string()));
    }

    #[test]
    fn test_join_fixed_go() {
        // 살다 + 고 → 개입 없음 (ㄱ은 탈락 대상 아님)
        let result = join(&rieul_verb("살다", "살"), &fixed::GO);
        assert_eq!(result, None);
    }

    // --- 다음절 ---

    #[test]
    fn test_join_multi_syllable() {
        // 만들다 + 는 → 만드는
        let result = join(&rieul_verb("만들다", "만들"), &fixed::NEUN);
        assert_eq!(result, Some("만드는".to_string()));
    }
}

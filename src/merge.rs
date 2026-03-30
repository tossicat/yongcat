//! ## 음운 변환 모듈
//! 어간과 어미의 결합 결과에 음운 축약/탈락 규칙을 적용합니다.
//!
//! 불규칙 활용에 따른 어간 변환과 모음 축약을 담당합니다(3단계).

use crate::eomi::Eomi;
use crate::irregular;
use crate::syllable;
use crate::yongeon::Yongeon;

/// `join`의 단순 접합 결과에 음운 규칙을 적용하여 최종 활용형을 반환합니다.
///
/// 불규칙 활용이면 `irregular` 모듈에 위임하고,
/// 규칙 활용이면 모음 축약/탈락 규칙을 적용합니다.
pub(crate) fn apply(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> String {
    if let Some(result) = irregular::merge(yongeon, joined, eomi) {
        return result;
    }

    match eomi {
        Eomi::AhEo(_) => apply_ah_eo(yongeon, joined),
        Eomi::Plain(_, _) => joined.to_string(),
        Eomi::Fixed(_) => joined.to_string(),
    }
}

/// 아/어 계열 어미의 음운 축약을 적용합니다.
///
/// 어간에 받침이 없을 때 모음 축약이 발생합니다.
/// - ㅏ+ㅏ → ㅏ (가+아요 → 가요)
/// - ㅗ+ㅏ → ㅘ (오+아요 → 와요)
/// - ㅜ+ㅓ → ㅝ (주+어요 → 줘요)
/// - ㅣ+ㅓ → ㅕ (피+어요 → 펴요)
/// - ㅡ+ㅓ → ㅓ (크+어요 → 커요)
/// - ㅓ+ㅓ → ㅓ (서+어요 → 서요)
fn apply_ah_eo(yongeon: &Yongeon, joined: &str) -> String {
    // 받침이 있으면 축약 없음
    if yongeon.has_coda() {
        return joined.to_string();
    }

    let eogan = yongeon.eogan_str();
    let eomi = &joined[eogan.len()..];

    // 어미 첫 음절 분해
    let eomi_syllables = syllable::decompose(eomi);
    let eomi_first = &eomi_syllables[0];

    // 어미 나머지 (첫 글자 제외)
    let first_char_len = eomi.chars().next().unwrap().len_utf8();
    let eomi_rest = &eomi[first_char_len..];

    // 어간 마지막 음절 수정
    let mut modified_eogan = yongeon.eogan.clone();
    let last_idx = modified_eogan.len() - 1;

    // 모음 축약 규칙
    let new_vowel = match modified_eogan[last_idx].vowel {
        'ㅏ' => 'ㅏ', // ㅏ+ㅏ → ㅏ (동일모음 탈락)
        'ㅓ' => 'ㅓ', // ㅓ+ㅓ → ㅓ (동일모음 탈락)
        'ㅗ' => 'ㅘ', // ㅗ+ㅏ → ㅘ
        'ㅜ' => 'ㅝ', // ㅜ+ㅓ → ㅝ
        'ㅣ' => 'ㅕ', // ㅣ+ㅓ → ㅕ
        'ㅡ' => 'ㅓ', // ㅡ+ㅓ → ㅓ (ㅡ 탈락)
        _ => return joined.to_string(),
    };

    modified_eogan[last_idx].vowel = new_vowel;
    modified_eogan[last_idx].coda = eomi_first.coda; // 어미 첫 음절의 종성 이전 (았→ㅆ)

    format!("{}{}", syllable::compose(&modified_eogan), eomi_rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eomi::ah_eo;
    use crate::join;
    use crate::types::{IrregularType, YongeonType};

    fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
    }

    fn adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
        Yongeon::new(
            base,
            "",
            eogan,
            YongeonType::Adjective,
            IrregularType::Regular,
        )
    }

    /// join → merge 파이프라인을 실행합니다.
    fn conjugate(yongeon: &Yongeon, eomi: &Eomi) -> String {
        let joined = join::select(yongeon, eomi);
        apply(yongeon, &joined, eomi)
    }

    // --- 해요체 (ah_eo::AYO) ---

    #[test]
    fn test_positive_vowel_a() {
        // 가다: ㅏ+ㅏ → ㅏ → 가요
        assert_eq!(conjugate(&verb("가다", "가"), &ah_eo::AYO), "가요");
    }

    #[test]
    fn test_positive_vowel_o() {
        // 오다: ㅗ+ㅏ → ㅘ → 와요
        assert_eq!(conjugate(&verb("오다", "오"), &ah_eo::AYO), "와요");
    }

    #[test]
    fn test_negative_vowel_eo() {
        // 서다: ㅓ+ㅓ → ㅓ → 서요
        assert_eq!(conjugate(&verb("서다", "서"), &ah_eo::AYO), "서요");
    }

    #[test]
    fn test_negative_vowel_u() {
        // 주다: ㅜ+ㅓ → ㅝ → 줘요
        assert_eq!(conjugate(&verb("주다", "주"), &ah_eo::AYO), "줘요");
    }

    #[test]
    fn test_negative_vowel_i() {
        // 피다: ㅣ+ㅓ → ㅕ → 펴요
        assert_eq!(conjugate(&verb("피다", "피"), &ah_eo::AYO), "펴요");
    }

    #[test]
    fn test_eu_drop() {
        // 크다: ㅡ+ㅓ → ㅓ → 커요
        assert_eq!(conjugate(&adj("크다", "크"), &ah_eo::AYO), "커요");
    }

    #[test]
    fn test_hada() {
        // 하다: 여불규칙, 하+여 → 해 → 해요
        let ha = Yongeon::new("하다", "", "하", YongeonType::Verb, IrregularType::Yeo);
        assert_eq!(conjugate(&ha, &ah_eo::AYO), "해요");
    }

    #[test]
    fn test_coda_no_contraction() {
        // 먹다: 받침 있음 → 축약 없음 → 먹어요
        assert_eq!(conjugate(&verb("먹다", "먹"), &ah_eo::AYO), "먹어요");
    }

    // --- 과거 시제 (ah_eo::ASS) ---

    #[test]
    fn test_past_positive() {
        // 가다: 가+았 → 갔
        assert_eq!(conjugate(&verb("가다", "가"), &ah_eo::ASS), "갔");
    }

    #[test]
    fn test_past_negative() {
        // 먹다: 먹+었 → 먹었
        assert_eq!(conjugate(&verb("먹다", "먹"), &ah_eo::ASS), "먹었");
    }

    #[test]
    fn test_past_hada() {
        // 하다: 여불규칙, 하+였 → 했
        let ha = Yongeon::new("하다", "", "하", YongeonType::Verb, IrregularType::Yeo);
        assert_eq!(conjugate(&ha, &ah_eo::ASS), "했");
    }

    #[test]
    fn test_past_o() {
        // 오다: 오+았 → 왔
        assert_eq!(conjugate(&verb("오다", "오"), &ah_eo::ASS), "왔");
    }

    #[test]
    fn test_past_eu() {
        // 크다: 크+었 → 컸
        assert_eq!(conjugate(&adj("크다", "크"), &ah_eo::ASS), "컸");
    }
}

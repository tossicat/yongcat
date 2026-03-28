//! ## 용언 구조체
//! 이 모듈에는 `yongcat`이 처리할 수 있는 용언의 구조체를 정의하고 있습니다.
//!
//! `data/yong_list.csv`을 읽어서 `build.rs`으로 이 라이브러리를 컴파일 할 때
//! 처리할 수 있는 전체 용언을 구조체로 가져옵니다.
//!

use crate::syllable::{self, Syllable};
use crate::types::{IrregularType, YongeonType};

/// 한국어 용언(동사/형용사)을 나타내는 구조체입니다.
///
/// 기본형(`base_form`)과 사전 일련번호(`dict_id`)로 용언을 식별하고,
/// 어간(`eogan`)을 음절 단위로 분해하여 보관합니다.
/// 품사 유형(`yongeon_type`)과 불규칙 활용 유형(`irregular_type`)을 통해
/// 활용 규칙 적용 시 필요한 정보를 제공합니다.
#[derive(Debug, Clone, PartialEq)]
pub struct Yongeon<'a> {
    /// 기본형 (사전 등재 형태)
    /// 예: "먹다", "걷다", "예쁘다"
    pub base_form: &'a str,
    /// 『표준국어대사전』 표제어 일련번호 (동음이의어 구분)
    /// 동음이의어가 없으면 빈 문자열
    pub dict_id: &'a str,
    /// 어간을 음절 단위로 분해한 형태
    /// 예: "먹다" → [Syllable(ㅁ,ㅓ,ㄱ)]
    pub eogan: Vec<Syllable>,
    /// 용언 유형 (동사/형용사)
    pub yongeon_type: YongeonType,
    /// 불규칙 활용 유형
    pub irregular_type: IrregularType,
}

impl<'a> Yongeon<'a> {
    /// 기본형과 어간 문자열로부터 Yongeon을 생성한다.
    ///
    /// # 예시
    /// ```
    /// use yongcat::{Yongeon, YongeonType, IrregularType};
    ///
    /// let y = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
    /// assert_eq!(y.base_form, "먹다");
    /// assert_eq!(y.eogan_str(), "먹");
    /// ```
    pub fn new(
        base_form: &'a str,
        dict_id: &'a str,
        eogan: &str,
        yongeon_type: YongeonType,
        irregular_type: IrregularType,
    ) -> Self {
        Self {
            base_form,
            dict_id,
            eogan: syllable::decompose(eogan),
            yongeon_type,
            irregular_type,
        }
    }

    /// 어간을 문자열로 반환한다.
    /// 예: "먹다" → "먹", "예쁘다" → "예쁘"
    pub fn eogan_str(&self) -> String {
        syllable::compose(&self.eogan)
    }

    /// 어간의 마지막 음절을 반환한다.
    /// 활용 규칙 적용 시 가장 중요한 정보이다.
    pub fn last_syllable(&self) -> &Syllable {
        self.eogan.last().expect("eogan은 비어 있을 수 없음")
    }

    /// 어간에 받침이 있는지 확인한다.
    pub fn has_coda(&self) -> bool {
        self.last_syllable().has_coda()
    }

    /// 어간 마지막 모음이 양성모음(ㅏ, ㅗ)인지 확인한다.
    pub fn is_positive_vowel(&self) -> bool {
        self.last_syllable().is_positive_vowel()
    }

    /// 동사인지 확인한다.
    pub fn is_verb(&self) -> bool {
        self.yongeon_type.is_verb()
    }

    /// 형용사인지 확인한다.
    pub fn is_adjective(&self) -> bool {
        self.yongeon_type.is_adjective()
    }

    /// 규칙 활용인지 확인한다.
    pub fn is_regular(&self) -> bool {
        self.irregular_type.is_regular()
    }

    /// 불규칙 활용인지 확인한다.
    pub fn is_irregular(&self) -> bool {
        self.irregular_type.is_irregular()
    }
}

impl std::fmt::Display for Yongeon<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}, {})",
            self.base_form, self.yongeon_type, self.irregular_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- 생성 테스트 ---

    #[test]
    fn test_new_regular_verb() {
        let y = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        assert_eq!(y.base_form, "먹다");
        assert_eq!(y.eogan_str(), "먹");
        assert_eq!(y.yongeon_type, YongeonType::Verb);
        assert_eq!(y.irregular_type, IrregularType::Regular);
    }

    #[test]
    fn test_new_irregular_verb() {
        let y = Yongeon::new("걷다", "02", "걷", YongeonType::Verb, IrregularType::Dieut);
        assert_eq!(y.base_form, "걷다");
        assert_eq!(y.eogan_str(), "걷");
        assert_eq!(y.irregular_type, IrregularType::Dieut);
    }

    #[test]
    fn test_new_regular_adjective() {
        let y = Yongeon::new(
            "크다",
            "",
            "크",
            YongeonType::Adjective,
            IrregularType::Regular,
        );
        assert_eq!(y.base_form, "크다");
        assert_eq!(y.eogan_str(), "크");
        assert_eq!(y.yongeon_type, YongeonType::Adjective);
    }

    #[test]
    fn test_new_irregular_adjective() {
        let y = Yongeon::new(
            "춥다",
            "",
            "춥",
            YongeonType::Adjective,
            IrregularType::Bieut,
        );
        assert_eq!(y.eogan_str(), "춥");
        assert_eq!(y.irregular_type, IrregularType::Bieut);
    }

    #[test]
    fn test_new_multi_syllable_eogan() {
        let y = Yongeon::new(
            "아름답다",
            "",
            "아름답",
            YongeonType::Adjective,
            IrregularType::Bieut,
        );
        assert_eq!(y.eogan_str(), "아름답");
        assert_eq!(y.eogan.len(), 3);
    }

    // --- 어간 분석 테스트 ---

    #[test]
    fn test_last_syllable() {
        let y = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        let last = y.last_syllable();
        assert_eq!(last.onset, 'ㅁ');
        assert_eq!(last.vowel, 'ㅓ');
        assert_eq!(last.coda, Some('ㄱ'));
    }

    #[test]
    fn test_has_coda() {
        let meok = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        assert!(meok.has_coda());

        let ga = Yongeon::new(
            "가다",
            "01",
            "가",
            YongeonType::Verb,
            IrregularType::Regular,
        );
        assert!(!ga.has_coda());
    }

    #[test]
    fn test_is_positive_vowel() {
        // "가다" — ㅏ (양성)
        let ga = Yongeon::new(
            "가다",
            "01",
            "가",
            YongeonType::Verb,
            IrregularType::Regular,
        );
        assert!(ga.is_positive_vowel());

        // "오다" — ㅗ (양성)
        let o = Yongeon::new("오다", "", "오", YongeonType::Verb, IrregularType::Regular);
        assert!(o.is_positive_vowel());

        // "먹다" — ㅓ (음성)
        let meok = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        assert!(!meok.is_positive_vowel());

        // "크다" — ㅡ (음성)
        let keu = Yongeon::new(
            "크다",
            "",
            "크",
            YongeonType::Adjective,
            IrregularType::Regular,
        );
        assert!(!keu.is_positive_vowel());
    }

    // --- 품사/활용 유형 확인 테스트 ---

    #[test]
    fn test_is_verb_and_adjective() {
        let verb = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        assert!(verb.is_verb());
        assert!(!verb.is_adjective());

        let adj = Yongeon::new(
            "예쁘다",
            "",
            "예쁘",
            YongeonType::Adjective,
            IrregularType::Regular,
        );
        assert!(adj.is_adjective());
        assert!(!adj.is_verb());
    }

    #[test]
    fn test_is_regular_and_irregular() {
        let regular = Yongeon::new("먹다", "", "먹", YongeonType::Verb, IrregularType::Regular);
        assert!(regular.is_regular());
        assert!(!regular.is_irregular());

        let irregular = Yongeon::new("걷다", "02", "걷", YongeonType::Verb, IrregularType::Dieut);
        assert!(irregular.is_irregular());
        assert!(!irregular.is_regular());
    }

    // --- Display 테스트 ---

    #[test]
    fn test_display() {
        let y = Yongeon::new("걷다", "02", "걷", YongeonType::Verb, IrregularType::Dieut);
        assert_eq!(format!("{}", y), "걷다 (동사, ㄷ불규칙)");

        let y = Yongeon::new(
            "예쁘다",
            "",
            "예쁘",
            YongeonType::Adjective,
            IrregularType::Regular,
        );
        assert_eq!(format!("{}", y), "예쁘다 (형용사, 규칙)");
    }

    // --- 다양한 불규칙 용언 테스트 ---

    #[test]
    fn test_various_irregular_types() {
        let cases = vec![
            (
                "듣다",
                "",
                "듣",
                YongeonType::Verb,
                IrregularType::Dieut,
                Some('ㄷ'),
            ),
            (
                "돕다",
                "",
                "돕",
                YongeonType::Verb,
                IrregularType::Bieut,
                Some('ㅂ'),
            ),
            (
                "노랗다",
                "",
                "노랗",
                YongeonType::Adjective,
                IrregularType::Hieut,
                Some('ㅎ'),
            ),
            (
                "살다",
                "",
                "살",
                YongeonType::Verb,
                IrregularType::Rieul,
                Some('ㄹ'),
            ),
            (
                "짓다",
                "",
                "짓",
                YongeonType::Verb,
                IrregularType::Siot,
                Some('ㅅ'),
            ),
            (
                "모르다",
                "",
                "모르",
                YongeonType::Verb,
                IrregularType::Reu,
                None,
            ),
            ("푸다", "", "푸", YongeonType::Verb, IrregularType::U, None),
        ];

        for (base, dict_id, eogan, ytype, itype, expected_coda) in cases {
            let y = Yongeon::new(base, dict_id, eogan, ytype, itype);
            assert_eq!(y.eogan_str(), eogan, "어간 불일치: {}", base);
            assert_eq!(
                y.last_syllable().coda,
                expected_coda,
                "종성 불일치: {}",
                base
            );
        }
    }
}

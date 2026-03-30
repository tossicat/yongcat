//! ## 용언 타입 정의
//! 용언의 품사 유형(`YongeonType`)과 불규칙 활용 유형(`IrregularType`)을
//! 정의하고 있습니다.
//!
//! `build.rs`에서 CSV를 파싱할 때와 `Yongeon` 구조체에서 사용합니다.

/// 용언 유형 (동사/형용사)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YongeonType {
    /// 동사
    Verb,
    /// 형용사
    Adjective,
}

impl YongeonType {
    pub fn is_verb(&self) -> bool {
        *self == Self::Verb
    }

    pub fn is_adjective(&self) -> bool {
        *self == Self::Adjective
    }
}

impl std::fmt::Display for YongeonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verb => write!(f, "동사"),
            Self::Adjective => write!(f, "형용사"),
        }
    }
}

/// 불규칙 활용 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrregularType {
    /// 규칙 (기본값)
    Regular,
    /// ㄷ불규칙
    Dieut,
    /// ㅂ불규칙
    Bieut,
    /// ㅎ불규칙
    Hieut,
    /// ㄹ불규칙
    Rieul,
    /// ㅅ불규칙
    Siot,
    /// 르불규칙
    Reu,
    /// 우불규칙
    U,
    /// 여불규칙
    Yeo,
    /// 러불규칙
    Reo,
    /// 으불규칙
    Eu,
}

impl IrregularType {
    pub fn is_regular(&self) -> bool {
        *self == Self::Regular
    }

    pub fn is_irregular(&self) -> bool {
        !self.is_regular()
    }
}

impl std::fmt::Display for IrregularType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "규칙"),
            Self::Dieut => write!(f, "ㄷ불규칙"),
            Self::Bieut => write!(f, "ㅂ불규칙"),
            Self::Hieut => write!(f, "ㅎ불규칙"),
            Self::Rieul => write!(f, "ㄹ불규칙"),
            Self::Siot => write!(f, "ㅅ불규칙"),
            Self::Reu => write!(f, "르불규칙"),
            Self::U => write!(f, "우불규칙"),
            Self::Yeo => write!(f, "여불규칙"),
            Self::Reo => write!(f, "러불규칙"),
            Self::Eu => write!(f, "으불규칙"),
        }
    }
}

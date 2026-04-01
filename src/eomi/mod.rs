//! ## 어미 모듈
//! 이 모듈에는 `yongcat`이 처리할 수 있는 어미 목록들을 가지고 있습니다.
//!
//! 각 어미는 붙일 단어에 따라 변환할 때, 변환하기 쉽게 하기 위하여
//! 양성모음, 음성모음, "하다"용 세 가지 형태를 갖습니다.
//! 이는 이전 `tossicat`에서 조사의 (&str, &str, &str) 튜플 컨벤션과 동일한 아이디어입니다.

pub mod ah_eo;
pub mod fixed;

/// (양성모음 형태, 음성모음 형태)
pub type AhEoForm = (&'static str, &'static str);

/// 어미를 나타내는 열거형입니다.
///
/// 어미의 종류에 따라 어간과 결합하는 규칙이 다르므로,
/// 그룹별로 구분하여 `join`과 `merge` 모듈에서 분기 처리합니다.
pub enum Eomi {
    /// 아/어 계열: 모음조화에 따라 형태가 달라집니다.
    AhEo(AhEoForm),
    /// 받침 유무에 따라 형태가 달라집니다. (받침 있을 때, 없을 때)
    Plain(&'static str, &'static str),
    /// 형태가 고정된 어미입니다.
    Fixed(&'static str),
}

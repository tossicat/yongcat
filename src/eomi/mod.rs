//! ## 어미 모듈
//! 용언에 붙는 어미를 세 가지 유형으로 분류하여 정의합니다.
//!
//! - `AhEo`: 모음조화에 따라 양성/음성 두 가지 형태를 가지는 아/어 계열 어미
//! - `Plain`: 어간의 받침 유무에 따라 형태가 달라지는 어미
//! - `Fixed`: 어간에 관계없이 형태가 고정된 어미
//!
//! 각 유형의 상수는 하위 모듈(`ah_eo`, `plain`, `fixed`)에 정의되어 있습니다.

pub mod ah_eo;
pub mod fixed;
pub mod plain;

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

//! ## 불규칙 활용 모듈
//! 용언의 불규칙 활용 유형별 어간-어미 결합 및 음운 변환을 담당합니다.
//!
//! 각 불규칙 유형은 별도 파일에 구현되며,
//! `join`과 `merge` 함수가 유형별로 디스패치합니다.

pub mod bieut;
pub mod dieut;
pub mod reo;
pub mod rieul;
pub mod siot;
pub mod u;
pub mod yeo;

use crate::eomi::Eomi;
use crate::types::IrregularType;
use crate::yongeon::Yongeon;

/// 불규칙 활용의 어간-어미 결합을 처리합니다.
///
/// 해당 불규칙 유형이 join 단계에 개입하면 `Some(결과)`를,
/// 개입하지 않으면 `None`을 반환하여 규칙 활용 로직으로 위임합니다.
pub(crate) fn join(yongeon: &Yongeon, eomi: &Eomi) -> Option<String> {
    match yongeon.irregular_type {
        IrregularType::Bieut => bieut::join(yongeon, eomi),
        IrregularType::Dieut => dieut::join(yongeon, eomi),
        IrregularType::Reo => reo::join(yongeon, eomi),
        IrregularType::Rieul => rieul::join(yongeon, eomi),
        IrregularType::Siot => siot::join(yongeon, eomi),
        IrregularType::Yeo => yeo::join(yongeon, eomi),
        _ => None,
    }
}

/// 불규칙 활용의 음운 축약/탈락을 처리합니다.
///
/// 해당 불규칙 유형이 merge 단계에 개입하면 `Some(결과)`를,
/// 개입하지 않으면 `None`을 반환하여 규칙 활용 로직으로 위임합니다.
pub(crate) fn merge(yongeon: &Yongeon, joined: &str, eomi: &Eomi) -> Option<String> {
    match yongeon.irregular_type {
        IrregularType::Bieut => bieut::merge(yongeon, joined, eomi),
        IrregularType::Reo => reo::merge(yongeon, joined, eomi),
        IrregularType::Siot => siot::merge(yongeon, joined, eomi),
        IrregularType::U => u::merge(yongeon, joined, eomi),
        IrregularType::Yeo => yeo::merge(yongeon, joined, eomi),
        _ => None,
    }
}

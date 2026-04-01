//! ## 고정 형태 어미
//! 어간에 그대로 접합하는 고정 형태 어미를 정의하고 있습니다.

use super::Eomi;

/// 고 — 연결 (나열): 먹 + 고 → 먹고, 가 + 고 → 가고
pub const GO: Eomi = Eomi::Fixed("고");

/// 지 — 부정: 먹 + 지 → 먹지, 가 + 지 → 가지
pub const JI: Eomi = Eomi::Fixed("지");

/// 는 — 관형사형 (현재): 먹 + 는 → 먹는, 가 + 는 → 가는
pub const NEUN: Eomi = Eomi::Fixed("는");

/// 게 — 결과: 먹 + 게 → 먹게, 가 + 게 → 가게
pub const GE: Eomi = Eomi::Fixed("게");

/// 자 — 청유: 먹 + 자 → 먹자, 가 + 자 → 가자
pub const JA: Eomi = Eomi::Fixed("자");

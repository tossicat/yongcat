//! ## 받침 유무 어미
//! 어간의 받침 유무에 따라 형태가 달라지는 어미를 정의하고 있습니다.
//!
//! 각 상수는 (받침 있을 때, 받침 없을 때) 순서입니다.

use super::Eomi;

/// 은/ㄴ — 관형사형 (과거): 먹 + 은 → 먹은, 가 + ㄴ → 가ㄴ
pub const EUN: Eomi = Eomi::Plain("은", "ㄴ");

/// 을/ㄹ — 관형사형 (미래): 먹 + 을 → 먹을, 가 + ㄹ → 가ㄹ
pub const EUL: Eomi = Eomi::Plain("을", "ㄹ");

/// 으면/면 — 조건: 먹 + 으면 → 먹으면, 가 + 면 → 가면
pub const EUMYEON: Eomi = Eomi::Plain("으면", "면");

/// 으니/니 — 이유: 먹 + 으니 → 먹으니, 가 + 니 → 가니
pub const EUNI: Eomi = Eomi::Plain("으니", "니");

/// 습니다/ㅂ니다 — 종결 (합쇼체): 먹 + 습니다 → 먹습니다, 가 + ㅂ니다 → 가ㅂ니다
pub const SEUMNIDA: Eomi = Eomi::Plain("습니다", "ㅂ니다");

/// 으면서/면서 — 동시: 먹 + 으면서 → 먹으면서, 가 + 면서 → 가면서
pub const EUMYEONSEO: Eomi = Eomi::Plain("으면서", "면서");

/// 으려고/려고 — 의도: 먹 + 으려고 → 먹으려고, 가 + 려고 → 가려고
pub const EURYEOGO: Eomi = Eomi::Plain("으려고", "려고");

/// 으세요/세요 — 높임 명령: 먹 + 으세요 → 먹으세요, 가 + 세요 → 가세요
pub const EUSEYO: Eomi = Eomi::Plain("으세요", "세요");

/// 으러/러 — 목적 (이동): 먹 + 으러 → 먹으러, 가 + 러 → 가러
pub const EUREO: Eomi = Eomi::Plain("으러", "러");
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

/// 겠어요 — 추측/의지 해요체: 먹 + 겠어요 → 먹겠어요, 가 + 겠어요 → 가겠어요
pub const GESS_EOYO: Eomi = Eomi::Fixed("겠어요");

/// 겠습니다 — 추측/의지 합쇼체: 먹 + 겠습니다 → 먹겠습니다, 가 + 겠습니다 → 가겠습니다
pub const GESS_SEUMNIDA: Eomi = Eomi::Fixed("겠습니다");

/// 다 — 종결 (평서, 기본형): 먹 + 다 → 먹다, 가 + 다 → 가다
pub const DA: Eomi = Eomi::Fixed("다");

/// 지만 — 연결 (대조): 먹 + 지만 → 먹지만, 가 + 지만 → 가지만
pub const JIMAN: Eomi = Eomi::Fixed("지만");

/// 거나 — 연결 (선택): 먹 + 거나 → 먹거나, 가 + 거나 → 가거나
pub const GEONA: Eomi = Eomi::Fixed("거나");

/// 는데 — 연결 (배경·대조): 먹 + 는데 → 먹는데, 가 + 는데 → 가는데
pub const NEUNDE: Eomi = Eomi::Fixed("는데");

/// 다가 — 연결 (전환): 먹 + 다가 → 먹다가, 가 + 다가 → 가다가
pub const DAGA: Eomi = Eomi::Fixed("다가");

/// 도록 — 연결 (목적·정도): 먹 + 도록 → 먹도록, 가 + 도록 → 가도록
pub const DOROK: Eomi = Eomi::Fixed("도록");

/// 다면 — 연결 (가정): 먹 + 다면 → 먹다면, 가 + 다면 → 가다면
pub const DAMYEON: Eomi = Eomi::Fixed("다면");

/// 나 — 의문 (반말): 먹 + 나 → 먹나, 가 + 나 → 가나
pub const NA: Eomi = Eomi::Fixed("나");

/// 냐 — 의문 (반말, 구어): 먹 + 냐 → 먹냐, 가 + 냐 → 가냐
pub const NYA: Eomi = Eomi::Fixed("냐");
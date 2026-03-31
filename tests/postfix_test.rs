//! ## postfix / postfix_word 통합 테스트
//!
//! 용언 활용 파이프라인(join → merge)이 실제 용언 데이터와 함께
//! 올바른 활용형을 생성하는지 검증합니다.
//!
//! - `postfix_word`: 단일 `Yongeon`에 어미를 적용하여 활용형 생성
//! - `postfix`: 단어 문자열로 동음이의어를 포함한 전체 활용형 생성

use yongcat::eomi::ah_eo;
use yongcat::{find_yongeon, load_yongeons, postfix, postfix_word};

// --- postfix_word: 해요체 (ah_eo::AYO) ---
//
// 아/어 계열 해요체 어미("아요"/"어요"/"여요")를 적용합니다.
// 어간 마지막 모음에 따라 모음조화 선택과 축약이 달라지므로,
// 양성모음·음성모음·"하다"·ㅗ축약 네 가지 케이스를 검증합니다.

/// 양성모음 (ㅏ): 가다 + 아요 → 가아요 → ㅏ+ㅏ 탈락 → 가요
#[test]
fn test_postfix_word_positive_vowel() {
    let yongeons = load_yongeons();
    let ga = &find_yongeon(&yongeons, "가다")[0];
    assert_eq!(postfix_word(ga, &ah_eo::AYO), "가요");
}

/// 음성모음 (ㅓ): 먹다 + 어요 → 먹어요 (받침 있어 축약 없음)
#[test]
fn test_postfix_word_negative_vowel() {
    let yongeons = load_yongeons();
    let meok = &find_yongeon(&yongeons, "먹다")[0];
    assert_eq!(postfix_word(meok, &ah_eo::AYO), "먹어요");
}

/// "하다"용: 하다 + 여요 → 하여요 → 하+여→해 축약 → 해요
#[test]
fn test_postfix_word_hada() {
    let yongeons = load_yongeons();
    let ha = &find_yongeon(&yongeons, "하다")[0];
    assert_eq!(postfix_word(ha, &ah_eo::AYO), "해요");
}

/// ㅗ+ㅏ 축약: 오다 + 아요 → 오아요 → ㅗ+ㅏ→ㅘ → 와요
#[test]
fn test_postfix_word_o_contraction() {
    let yongeons = load_yongeons();
    let o = &find_yongeon(&yongeons, "오다")[0];
    assert_eq!(postfix_word(o, &ah_eo::AYO), "와요");
}

// --- postfix_word: 과거 시제 (ah_eo::ASS) ---
//
// 과거 시제 어미("았"/"었"/"였")를 적용합니다.
// 어미 첫 음절에 종성 ㅆ이 있으므로, 축약 시 종성이
// 어간 마지막 음절로 이전되는지 함께 검증합니다.

/// 양성모음 과거: 가다 + 았 → 가았 → ㅏ+ㅏ 탈락 + ㅆ 이전 → 갔
#[test]
fn test_postfix_word_past_positive() {
    let yongeons = load_yongeons();
    let ga = &find_yongeon(&yongeons, "가다")[0];
    assert_eq!(postfix_word(ga, &ah_eo::ASS), "갔");
}

/// 음성모음 과거: 먹다 + 었 → 먹었 (받침 있어 축약 없음)
#[test]
fn test_postfix_word_past_negative() {
    let yongeons = load_yongeons();
    let meok = &find_yongeon(&yongeons, "먹다")[0];
    assert_eq!(postfix_word(meok, &ah_eo::ASS), "먹었");
}

/// "하다" 과거: 하다 + 였 → 하였 → 하+여→해 축약 + ㅆ 이전 → 했
#[test]
fn test_postfix_word_past_hada() {
    let yongeons = load_yongeons();
    let ha = &find_yongeon(&yongeons, "하다")[0];
    assert_eq!(postfix_word(ha, &ah_eo::ASS), "했");
}

// --- postfix_word: 다양한 어미 ---
//
// 해요체·과거 외의 아/어 계열 어미가 동일한 파이프라인으로
// 올바르게 처리되는지 검증합니다.

/// 연결 (이유): 가다 + 아서 → 가아서 → ㅏ+ㅏ 탈락 → 가서
#[test]
fn test_postfix_word_reason() {
    let yongeons = load_yongeons();
    let ga = &find_yongeon(&yongeons, "가다")[0];
    assert_eq!(postfix_word(ga, &ah_eo::ASEO), "가서");
}

/// 연결 (양보): 먹다 + 어도 → 먹어도 (받침 있어 축약 없음)
#[test]
fn test_postfix_word_concession() {
    let yongeons = load_yongeons();
    let meok = &find_yongeon(&yongeons, "먹다")[0];
    assert_eq!(postfix_word(meok, &ah_eo::ADO), "먹어도");
}

/// 명령: 가다 + 아라 → 가아라 → ㅏ+ㅏ 탈락 → 가라
#[test]
fn test_postfix_word_command() {
    let yongeons = load_yongeons();
    let ga = &find_yongeon(&yongeons, "가다")[0];
    assert_eq!(postfix_word(ga, &ah_eo::ARA), "가라");
}

// --- postfix_word: 여불규칙 (복합 하다 용언) ---
//
// "하다"뿐 아니라 "공부하다", "감사하다" 등 복합 하다 용언도
// 여불규칙이 올바르게 적용되는지 검증합니다.

/// 동사: 공부하다 + 여요 → 공부해요
#[test]
fn test_postfix_word_yeo_verb() {
    let yongeons = load_yongeons();
    let gongbu = &find_yongeon(&yongeons, "공부하다")[0];
    assert_eq!(postfix_word(gongbu, &ah_eo::AYO), "공부해요");
}

/// 형용사: 감사하다 + 여요 → 감사해요
#[test]
fn test_postfix_word_yeo_adjective() {
    let yongeons = load_yongeons();
    let gamsa = &find_yongeon(&yongeons, "감사하다")[0];
    assert_eq!(postfix_word(gamsa, &ah_eo::AYO), "감사해요");
}

/// 여불규칙 과거: 공부하다 + 였 → 공부했
#[test]
fn test_postfix_word_yeo_past() {
    let yongeons = load_yongeons();
    let gongbu = &find_yongeon(&yongeons, "공부하다")[0];
    assert_eq!(postfix_word(gongbu, &ah_eo::ASS), "공부했");
}

/// 여불규칙 연결: 감사하다 + 여서 → 감사해서
#[test]
fn test_postfix_word_yeo_reason() {
    let yongeons = load_yongeons();
    let gamsa = &find_yongeon(&yongeons, "감사하다")[0];
    assert_eq!(postfix_word(gamsa, &ah_eo::ASEO), "감사해서");
}

// --- postfix: 동음이의어 처리 ---
//
// `postfix`는 단어 문자열을 받아 동음이의어 전체에 대해 활용형을 생성합니다.
// 각 결과에는 `Yongeon` 참조가 포함되어 `dict_id`로 구별할 수 있습니다.

/// 동음이의어가 있는 단어는 여러 결과를 반환합니다.
/// "쓰다"는 "쓰다01"(사용), "쓰다02"(착용) 등 2개 이상 존재합니다.
#[test]
fn test_postfix_returns_all_homonyms() {
    let yongeons = load_yongeons();
    let results = postfix(&yongeons, "쓰다", &ah_eo::AYO);
    assert!(
        results.len() >= 2,
        "쓰다는 동음이의어가 2개 이상이어야 함"
    );
}

/// 동음이의어는 `dict_id`로 구별할 수 있어야 합니다.
/// 각 결과의 `Yongeon`에 포함된 `dict_id`가 서로 달라야 합니다.
#[test]
fn test_postfix_includes_dict_id() {
    let yongeons = load_yongeons();
    let results = postfix(&yongeons, "쓰다", &ah_eo::AYO);
    let dict_ids: Vec<&str> = results.iter().map(|(y, _)| y.dict_id).collect();
    assert!(
        dict_ids.windows(2).all(|w| w[0] != w[1]),
        "동음이의어의 dict_id는 서로 달라야 함"
    );
}

/// 존재하지 않는 단어를 넣으면 빈 Vec을 반환합니다.
#[test]
fn test_postfix_nonexistent() {
    let yongeons = load_yongeons();
    let results = postfix(&yongeons, "없는단어다", &ah_eo::AYO);
    assert!(results.is_empty());
}

/// 동음이의어 모두 동일한 규칙 활용이면 활용형도 같아야 합니다.
/// "먹다"는 동음이의어가 있지만 모두 규칙 동사이므로 결과가 동일합니다.
#[test]
fn test_postfix_all_conjugated() {
    let yongeons = load_yongeons();
    let results = postfix(&yongeons, "먹다", &ah_eo::AYO);
    assert!(!results.is_empty());
    assert!(results.iter().all(|(_, conjugated)| conjugated == "먹어요"));
}

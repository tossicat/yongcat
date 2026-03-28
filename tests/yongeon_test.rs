//! ## 용언 검색 통합 테스트
//!
//! `load_yongeons()`로 빌드 시 생성된 실제 용언 데이터를 로드한 뒤,
//! `find_yongeon`(기본형 검색)과 `find_eogan`(어간 검색)이
//! 올바른 결과를 반환하는지 검증합니다.

use yongcat::{find_eogan, find_yongeon, load_yongeons, IrregularType, YongeonType};

// --- find_yongeon: 기본형으로 검색 ---
//
// 사전 기본형("먹다", "가깝다" 등)을 입력하여 일치하는 용언을 찾습니다.
// 동음이의어가 있으면 여러 개, 없으면 빈 Vec을 반환합니다.

/// 존재하는 동사를 검색하면 결과가 반환되고, 품사가 동사(Verb)여야 합니다.
#[test]
fn test_find_existing_verb() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "먹다");
    assert!(!results.is_empty());
    assert!(results.iter().all(|y| y.base_form == "먹다"));
    assert!(results.iter().all(|y| y.yongeon_type == YongeonType::Verb));
}

/// 존재하는 형용사를 검색하면 품사가 형용사(Adjective)이고,
/// "가깝다"는 ㅂ불규칙 활용이어야 합니다.
#[test]
fn test_find_existing_adjective() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "가깝다");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].yongeon_type, YongeonType::Adjective);
    assert_eq!(results[0].irregular_type, IrregularType::Bieut);
}

/// "쓰다"는 동음이의어가 2개 이상 존재합니다.
/// 각각 다른 `dict_id`로 구별됩니다.
#[test]
fn test_find_homonyms() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "쓰다");
    assert!(results.len() >= 2, "쓰다는 동음이의어가 2개 이상이어야 함");
}

/// 존재하지 않는 단어를 검색하면 빈 Vec을 반환합니다.
#[test]
fn test_find_nonexistent() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "없는단어다");
    assert!(results.is_empty());
}

// --- find_eogan: 어간으로 검색 ---
//
// 어간("먹", "가깝" 등)을 입력하여 일치하는 용언을 찾습니다.
// 기본형 검색과 달리 "다"를 제외한 어간 부분으로 검색합니다.

/// 어간 "먹"으로 검색하면 "먹다"에 해당하는 용언이 반환됩니다.
#[test]
fn test_find_eogan_existing() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "먹");
    assert!(!results.is_empty());
    assert!(results.iter().all(|y| y.eogan_str() == "먹"));
}

/// 어간 "가깝"으로 검색하면 ㅂ불규칙 형용사 "가깝다"가 반환됩니다.
#[test]
fn test_find_eogan_irregular() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "가깝");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].irregular_type, IrregularType::Bieut);
}

/// 존재하지 않는 어간을 검색하면 빈 Vec을 반환합니다.
#[test]
fn test_find_eogan_nonexistent() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "없는어간");
    assert!(results.is_empty());
}

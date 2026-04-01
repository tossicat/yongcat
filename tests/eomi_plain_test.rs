//! ## 받침 유무 어미 통합 테스트
//!
//! 각 plain 상수가 받침 있는/없는 어간에서 올바른 형태를 선택하는지 검증합니다.

use yongcat::eomi::plain;
use yongcat::{postfix_word, IrregularType, Yongeon, YongeonType};

fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
}

// --- 받침 있는 어간 ---

#[test]
fn test_eun_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUN), "먹은");
}

#[test]
fn test_eul_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUL), "먹을");
}

#[test]
fn test_eumyeon_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUMYEON), "먹으면");
}

#[test]
fn test_euni_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUNI), "먹으니");
}

#[test]
fn test_seumnida_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::SEUMNIDA), "먹습니다");
}

// --- 받침 없는 어간 ---

#[test]
fn test_eun_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUN), "가ㄴ");
}

#[test]
fn test_eul_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUL), "가ㄹ");
}

#[test]
fn test_eumyeon_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUMYEON), "가면");
}

#[test]
fn test_euni_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUNI), "가니");
}

#[test]
fn test_seumnida_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::SEUMNIDA), "가ㅂ니다");
}

// --- 으면서/면서 ---

#[test]
fn test_eumyeonseo_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUMYEONSEO), "먹으면서");
}

#[test]
fn test_eumyeonseo_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUMYEONSEO), "가면서");
}

// --- 으려고/려고 ---

#[test]
fn test_euryeogo_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EURYEOGO), "먹으려고");
}

#[test]
fn test_euryeogo_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EURYEOGO), "가려고");
}

// --- 으세요/세요 ---

#[test]
fn test_euseyo_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUSEYO), "먹으세요");
}

#[test]
fn test_euseyo_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUSEYO), "가세요");
}

// --- 으러/러 ---

#[test]
fn test_eureo_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &plain::EUREO), "먹으러");
}

#[test]
fn test_eureo_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &plain::EUREO), "가러");
}

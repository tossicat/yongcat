//! ## 고정 형태 어미 통합 테스트
//!
//! 각 fixed 상수가 받침 있는/없는 어간에서 올바르게 접합되는지 검증합니다.

use yongcat::eomi::fixed;
use yongcat::{postfix_word, IrregularType, Yongeon, YongeonType};

fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
}

// --- 받침 있는 어간 ---

#[test]
fn test_go_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GO), "먹고");
}

#[test]
fn test_ji_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JI), "먹지");
}

#[test]
fn test_neun_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::NEUN), "먹는");
}

#[test]
fn test_ge_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GE), "먹게");
}

#[test]
fn test_ja_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JA), "먹자");
}

// --- 받침 없는 어간 ---

#[test]
fn test_go_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::GO), "가고");
}

#[test]
fn test_ji_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JI), "가지");
}

#[test]
fn test_neun_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::NEUN), "가는");
}

#[test]
fn test_ge_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::GE), "가게");
}

#[test]
fn test_ja_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JA), "가자");
}

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

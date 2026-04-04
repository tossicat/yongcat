//! ## 고정 형태 어미 통합 테스트
//!
//! 각 fixed 상수가 받침 있는/없는 어간에서 올바르게 접합되는지 검증합니다.

use yongcat::eomi::fixed;
use yongcat::{postfix_word, IrregularType, Yongeon, YongeonType};

fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
}

fn rieul_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Rieul)
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

// --- ㄹ불규칙 (ㄴ 앞 ㄹ 탈락) ---

#[test]
fn test_rieul_neun() {
    // 살다 + 는 → 사는 (ㄹ탈락 before ㄴ)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::NEUN), "사는");
}

#[test]
fn test_rieul_go() {
    // 살다 + 고 → 살고 (ㄱ은 탈락 대상 아님)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::GO), "살고");
}

#[test]
fn test_rieul_ji() {
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::JI), "살지");
}

#[test]
fn test_rieul_ge() {
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::GE), "살게");
}

#[test]
fn test_rieul_ja() {
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::JA), "살자");
}

// --- 새 Fixed 어미 ---

#[test]
fn test_da_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DA), "먹다");
}

#[test]
fn test_da_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::DA), "가다");
}

#[test]
fn test_jiman_with_coda() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JIMAN), "먹지만");
}

#[test]
fn test_jiman_without_coda() {
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JIMAN), "가지만");
}

#[test]
fn test_geona() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GEONA), "먹거나");
}

#[test]
fn test_neunde() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::NEUNDE), "먹는데");
}

#[test]
fn test_daga() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DAGA), "먹다가");
}

#[test]
fn test_dorok() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DOROK), "먹도록");
}

#[test]
fn test_damyeon() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DAMYEON), "먹다면");
}

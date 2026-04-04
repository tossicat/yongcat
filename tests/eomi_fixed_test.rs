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
    // 먹다 + 고 → 먹고 (나열)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GO), "먹고");
}

#[test]
fn test_ji_with_coda() {
    // 먹다 + 지 → 먹지 (부정)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JI), "먹지");
}

#[test]
fn test_neun_with_coda() {
    // 먹다 + 는 → 먹는 (관형사형, 현재)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::NEUN), "먹는");
}

#[test]
fn test_ge_with_coda() {
    // 먹다 + 게 → 먹게 (결과)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GE), "먹게");
}

#[test]
fn test_ja_with_coda() {
    // 먹다 + 자 → 먹자 (청유)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JA), "먹자");
}

// --- 받침 없는 어간 ---

#[test]
fn test_go_without_coda() {
    // 가다 + 고 → 가고
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::GO), "가고");
}

#[test]
fn test_ji_without_coda() {
    // 가다 + 지 → 가지
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JI), "가지");
}

#[test]
fn test_neun_without_coda() {
    // 가다 + 는 → 가는
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::NEUN), "가는");
}

#[test]
fn test_ge_without_coda() {
    // 가다 + 게 → 가게
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::GE), "가게");
}

#[test]
fn test_ja_without_coda() {
    // 가다 + 자 → 가자
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JA), "가자");
}

// --- ㄹ불규칙 + Fixed ---

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
    // 살다 + 지 → 살지 (ㅈ은 탈락 대상 아님)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::JI), "살지");
}

#[test]
fn test_rieul_ge() {
    // 살다 + 게 → 살게 (ㄱ은 탈락 대상 아님)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::GE), "살게");
}

#[test]
fn test_rieul_ja() {
    // 살다 + 자 → 살자 (ㅈ은 탈락 대상 아님)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &fixed::JA), "살자");
}

// --- 새 Fixed 어미 ---

#[test]
fn test_da_with_coda() {
    // 먹다 + 다 → 먹다 (종결, 평서)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DA), "먹다");
}

#[test]
fn test_da_without_coda() {
    // 가다 + 다 → 가다
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::DA), "가다");
}

#[test]
fn test_jiman_with_coda() {
    // 먹다 + 지만 → 먹지만 (대조)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::JIMAN), "먹지만");
}

#[test]
fn test_jiman_without_coda() {
    // 가다 + 지만 → 가지만
    assert_eq!(postfix_word(&verb("가다", "가"), &fixed::JIMAN), "가지만");
}

#[test]
fn test_geona() {
    // 먹다 + 거나 → 먹거나 (선택)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::GEONA), "먹거나");
}

#[test]
fn test_neunde() {
    // 먹다 + 는데 → 먹는데 (배경·대조)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::NEUNDE), "먹는데");
}

#[test]
fn test_daga() {
    // 먹다 + 다가 → 먹다가 (전환)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DAGA), "먹다가");
}

#[test]
fn test_dorok() {
    // 먹다 + 도록 → 먹도록 (목적·정도)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DOROK), "먹도록");
}

#[test]
fn test_damyeon() {
    // 먹다 + 다면 → 먹다면 (가정)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &fixed::DAMYEON), "먹다면");
}

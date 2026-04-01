//! ## 아/어 계열 어미 통합 테스트
//!
//! 각 ah_eo 상수가 규칙/불규칙 활용에서 올바른 활용형을 생성하는지 검증합니다.

use yongcat::eomi::ah_eo;
use yongcat::{postfix_word, IrregularType, Yongeon, YongeonType};

fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
}

fn yeo_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Yeo)
}

fn dieut_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Dieut)
}

fn bieut_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Bieut)
}

fn siot_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Siot)
}

fn rieul_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Rieul)
}

fn u_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::U)
}

fn reo_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Reo)
}

fn eu_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Eu)
}

// --- 규칙 활용 ---

#[test]
fn test_regular_positive() {
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::AYO), "가요");
}

#[test]
fn test_regular_negative() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::AYO), "먹어요");
}

#[test]
fn test_regular_past() {
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ASS), "갔");
}

#[test]
fn test_regular_reason() {
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ASEO), "가서");
}

#[test]
fn test_regular_concession() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::ADO), "먹어도");
}

#[test]
fn test_regular_condition() {
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::AYA), "가야");
}

#[test]
fn test_regular_command() {
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::ARA), "먹어라");
}

#[test]
fn test_regular_banmal() {
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::A), "가");
}

// --- 불규칙 활용 ---

#[test]
fn test_yeo() {
    assert_eq!(postfix_word(&yeo_verb("하다", "하"), &ah_eo::AYO), "해요");
}

#[test]
fn test_dieut() {
    assert_eq!(postfix_word(&dieut_verb("걷다", "걷"), &ah_eo::AYO), "걸어요");
}

#[test]
fn test_bieut() {
    assert_eq!(postfix_word(&bieut_verb("돕다", "돕"), &ah_eo::AYO), "도와요");
}

#[test]
fn test_siot() {
    assert_eq!(postfix_word(&siot_verb("짓다", "짓"), &ah_eo::AYO), "지어요");
}

#[test]
fn test_rieul() {
    // ㄹ불규칙은 AhEo에서 규칙 활용과 동일
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &ah_eo::AYO), "살아요");
}

#[test]
fn test_u() {
    assert_eq!(postfix_word(&u_verb("푸다", "푸"), &ah_eo::AYO), "퍼요");
}

#[test]
fn test_reo() {
    assert_eq!(postfix_word(&reo_verb("이르다", "이르"), &ah_eo::AYO), "이르러요");
}

#[test]
fn test_eu() {
    // 으불규칙은 규칙 활용의 ㅡ 탈락으로 처리
    assert_eq!(postfix_word(&eu_adj("바쁘다", "바쁘"), &ah_eo::AYO), "바빠요");
}

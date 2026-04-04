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

fn hieut_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Hieut)
}

fn reu_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Reu)
}

fn eu_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Eu)
}

// --- 규칙 활용 ---

#[test]
fn test_regular_positive() {
    // 가다 + 아요 → 가요 (양성모음, ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::AYO), "가요");
}

#[test]
fn test_regular_negative() {
    // 먹다 + 어요 → 먹어요 (음성모음, 받침 있어 축약 없음)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::AYO), "먹어요");
}

#[test]
fn test_regular_past() {
    // 가다 + 았 → 갔 (ㅏ+ㅏ 탈락, ㅆ 이전)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ASS), "갔");
}

#[test]
fn test_regular_reason() {
    // 가다 + 아서 → 가서 (ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ASEO), "가서");
}

#[test]
fn test_regular_concession() {
    // 먹다 + 어도 → 먹어도 (받침 있어 축약 없음)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::ADO), "먹어도");
}

#[test]
fn test_regular_condition() {
    // 가다 + 아야 → 가야 (ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::AYA), "가야");
}

#[test]
fn test_regular_command() {
    // 먹다 + 어라 → 먹어라 (받침 있어 축약 없음)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::ARA), "먹어라");
}

#[test]
fn test_regular_banmal() {
    // 가다 + 아 → 가 (ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::A), "가");
}

// --- 불규칙 활용 ---

#[test]
fn test_yeo() {
    // 하다 + 어요 → 해요 (여불규칙: ㅓ→ㅕ, ㅏ→ㅐ 축약)
    assert_eq!(postfix_word(&yeo_verb("하다", "하"), &ah_eo::AYO), "해요");
}

#[test]
fn test_dieut() {
    // 걷다 + 어요 → 걸어요 (ㄷ불규칙: ㄷ→ㄹ)
    assert_eq!(postfix_word(&dieut_verb("걷다", "걷"), &ah_eo::AYO), "걸어요");
}

#[test]
fn test_bieut() {
    // 돕다 + 아요 → 도와요 (ㅂ불규칙: ㅂ→우, ㅜ+ㅏ→ㅘ)
    assert_eq!(postfix_word(&bieut_verb("돕다", "돕"), &ah_eo::AYO), "도와요");
}

#[test]
fn test_siot() {
    // 짓다 + 어요 → 지어요 (ㅅ불규칙: ㅅ 탈락, 축약 억제)
    assert_eq!(postfix_word(&siot_verb("짓다", "짓"), &ah_eo::AYO), "지어요");
}

#[test]
fn test_rieul() {
    // ㄹ불규칙은 AhEo에서 규칙 활용과 동일
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &ah_eo::AYO), "살아요");
}

#[test]
fn test_u() {
    // 푸다 + 어요 → 퍼요 (우불규칙: ㅜ→ㅓ)
    assert_eq!(postfix_word(&u_verb("푸다", "푸"), &ah_eo::AYO), "퍼요");
}

#[test]
fn test_reo() {
    // 이르다 + 어요 → 이르러요 (러불규칙: 어→러)
    assert_eq!(postfix_word(&reo_verb("이르다", "이르"), &ah_eo::AYO), "이르러요");
}

#[test]
fn test_hieut_negative() {
    // 그렇다 + 어요 → 그래요 (ㅎ불규칙: ㅎ 탈락, ㅓ→ㅐ)
    assert_eq!(postfix_word(&hieut_adj("그렇다", "그렇"), &ah_eo::AYO), "그래요");
}

#[test]
fn test_hieut_positive() {
    // 노랗다 + 아요 → 노래요 (ㅎ불규칙: ㅎ 탈락, ㅏ→ㅐ)
    assert_eq!(postfix_word(&hieut_adj("노랗다", "노랗"), &ah_eo::AYO), "노래요");
}

#[test]
fn test_reu_positive() {
    // 모르다 + 아요 → 몰라요 (르불규칙: ㄹ 삽입, 초성 교체)
    assert_eq!(postfix_word(&reu_verb("모르다", "모르"), &ah_eo::AYO), "몰라요");
}

#[test]
fn test_reu_negative() {
    // 기르다 + 어요 → 길러요 (르불규칙: ㄹ 삽입, 초성 교체)
    assert_eq!(postfix_word(&reu_verb("기르다", "기르"), &ah_eo::AYO), "길러요");
}

#[test]
fn test_eu() {
    // 으불규칙은 규칙 활용의 ㅡ 탈락으로 처리
    assert_eq!(postfix_word(&eu_adj("바쁘다", "바쁘"), &ah_eo::AYO), "바빠요");
}

// --- 새 AhEo 어미 ---

#[test]
fn test_ajida_positive() {
    // 좋다 + 아지다 → 좋아지다 (피동·상태변화)
    let adj = Yongeon::new("좋다", "", "좋", YongeonType::Adjective, IrregularType::Regular);
    assert_eq!(postfix_word(&adj, &ah_eo::AJIDA), "좋아지다");
}

#[test]
fn test_ajida_negative() {
    // 크다 + 어지다 → 커지다 (ㅡ 탈락)
    let adj = Yongeon::new("크다", "", "크", YongeonType::Adjective, IrregularType::Regular);
    assert_eq!(postfix_word(&adj, &ah_eo::AJIDA), "커지다");
}

#[test]
fn test_aboda_positive() {
    // 가다 + 아보다 → 가보다 (ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ABODA), "가보다");
}

#[test]
fn test_aboda_negative() {
    // 먹다 + 어보다 → 먹어보다 (받침 있어 축약 없음)
    assert_eq!(postfix_word(&verb("먹다", "먹"), &ah_eo::ABODA), "먹어보다");
}

#[test]
fn test_adalla_positive() {
    // 가다 + 아달라 → 가달라 (ㅏ+ㅏ 탈락)
    assert_eq!(postfix_word(&verb("가다", "가"), &ah_eo::ADALLA), "가달라");
}

#[test]
fn test_adalla_negative() {
    // 읽다 + 어달라 → 읽어달라 (받침 있어 축약 없음)
    assert_eq!(postfix_word(&verb("읽다", "읽"), &ah_eo::ADALLA), "읽어달라");
}

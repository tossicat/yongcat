//! ## 받침 유무 어미 통합 테스트
//!
//! 각 plain 상수가 받침 있는/없는 어간에서 올바른 형태를 선택하는지 검증합니다.

use yongcat::eomi::plain;
use yongcat::{postfix_word, IrregularType, Yongeon, YongeonType};

fn verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Regular)
}

fn dieut_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Dieut)
}

fn bieut_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Bieut)
}

fn siot_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Siot)
}

fn rieul_verb(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Verb, IrregularType::Rieul)
}

fn hieut_adj(base: &'static str, eogan: &str) -> Yongeon<'static> {
    Yongeon::new(base, "", eogan, YongeonType::Adjective, IrregularType::Hieut)
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

// --- ㄷ불규칙 + Plain ---

#[test]
fn test_dieut_eumyeon() {
    // 걷다 + 으면 → 걸으면 (모음 시작 → ㄷ→ㄹ)
    assert_eq!(postfix_word(&dieut_verb("걷다", "걷"), &plain::EUMYEON), "걸으면");
}

#[test]
fn test_dieut_seumnida() {
    // 걷다 + 습니다 → 걷습니다 (자음 시작 → 변환 없음)
    assert_eq!(postfix_word(&dieut_verb("걷다", "걷"), &plain::SEUMNIDA), "걷습니다");
}

// --- ㅂ불규칙 + Plain ---

#[test]
fn test_bieut_eumyeon() {
    // 가깝다 + 으면/면 → 가까우면 (ㅂ→우, 무받침형)
    assert_eq!(postfix_word(&bieut_adj("가깝다", "가깝"), &plain::EUMYEON), "가까우면");
}

#[test]
fn test_bieut_seumnida() {
    // 가깝다 + 습니다 → 가깝습니다 (자음 시작 → 변환 없음)
    assert_eq!(postfix_word(&bieut_adj("가깝다", "가깝"), &plain::SEUMNIDA), "가깝습니다");
}

// --- ㅅ불규칙 + Plain ---

#[test]
fn test_siot_eumyeon() {
    // 짓다 + 으면/면 → 지면 (ㅅ 탈락, 무받침형)
    assert_eq!(postfix_word(&siot_verb("짓다", "짓"), &plain::EUMYEON), "지면");
}

#[test]
fn test_siot_seumnida() {
    // 짓다 + 습니다 → 짓습니다 (자음 시작 → 변환 없음)
    assert_eq!(postfix_word(&siot_verb("짓다", "짓"), &plain::SEUMNIDA), "짓습니다");
}

// --- ㄹ불규칙 + Plain ---

#[test]
fn test_rieul_eumyeon() {
    // 살다 + 으면/면 → 살면 (ㄹ유지, 무받침형)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &plain::EUMYEON), "살면");
}

#[test]
fn test_rieul_seumnida() {
    // 살다 + 습니다/ㅂ니다 → 사ㅂ니다 (ㄹ탈락 before ㅂ)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &plain::SEUMNIDA), "사ㅂ니다");
}

#[test]
fn test_rieul_euni() {
    // 살다 + 으니/니 → 사니 (ㄹ탈락 before ㄴ)
    assert_eq!(postfix_word(&rieul_verb("살다", "살"), &plain::EUNI), "사니");
}

// --- ㅎ불규칙 + Plain ---

#[test]
fn test_hieut_eumyeon() {
    // 그렇다 + 으면/면 → 그러면 (ㅎ 탈락, 무받침형)
    assert_eq!(postfix_word(&hieut_adj("그렇다", "그렇"), &plain::EUMYEON), "그러면");
}

#[test]
fn test_hieut_eun() {
    // 그렇다 + 은/ㄴ → 그러ㄴ (ㅎ 탈락, 무받침형)
    assert_eq!(postfix_word(&hieut_adj("그렇다", "그렇"), &plain::EUN), "그러ㄴ");
}

#[test]
fn test_hieut_seumnida() {
    // 그렇다 + 습니다 → 그렇습니다 (자음 시작 → 변환 없음)
    assert_eq!(postfix_word(&hieut_adj("그렇다", "그렇"), &plain::SEUMNIDA), "그렇습니다");
}

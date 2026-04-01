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

// --- postfix_word: ㄹ불규칙 ---
//
// ㄴ/ㅂ/ㅅ 앞에서 ㄹ 탈락, Plain은 항상 무받침 형태 선택.

/// 살다 + 아요 → 살아요 (AhEo는 규칙 활용)
#[test]
fn test_postfix_word_rieul_ah_eo() {
    let yongeons = load_yongeons();
    let sal = &find_yongeon(&yongeons, "살다")[0];
    assert_eq!(postfix_word(sal, &ah_eo::AYO), "살아요");
}

/// 살다 + 는 → 사는 (Fixed, ㄹ탈락 before ㄴ)
#[test]
fn test_postfix_word_rieul_neun() {
    let yongeons = load_yongeons();
    let sal = &find_yongeon(&yongeons, "살다")[0];
    assert_eq!(postfix_word(sal, &yongcat::eomi::fixed::NEUN), "사는");
}

/// 살다 + 면 → 살면 (Plain, ㄹ유지)
#[test]
fn test_postfix_word_rieul_myeon() {
    let yongeons = load_yongeons();
    let sal = &find_yongeon(&yongeons, "살다")[0];
    assert_eq!(postfix_word(sal, &yongcat::eomi::plain::EUMYEON), "살면");
}

/// 만들다 + 는 → 만드는 (다음절, ㄹ탈락)
#[test]
fn test_postfix_word_rieul_multi_syllable() {
    let yongeons = load_yongeons();
    let mandeul = &find_yongeon(&yongeons, "만들다")[0];
    assert_eq!(postfix_word(mandeul, &yongcat::eomi::fixed::NEUN), "만드는");
}

/// 알다 + 니 → 아니 (Plain, ㄹ탈락 before ㄴ)
#[test]
fn test_postfix_word_rieul_euni() {
    let yongeons = load_yongeons();
    let al = &find_yongeon(&yongeons, "알다")[0];
    assert_eq!(postfix_word(al, &yongcat::eomi::plain::EUNI), "아니");
}

// --- postfix_word: ㅅ불규칙 ---
//
// 어간 끝 ㅅ이 탈락하고, 모음 축약이 적용되지 않습니다.

/// 짓다 + 어요 → 지어요 (져요가 아님)
#[test]
fn test_postfix_word_siot_ayo() {
    let yongeons = load_yongeons();
    let jit = &find_yongeon(&yongeons, "짓다")[0];
    assert_eq!(postfix_word(jit, &ah_eo::AYO), "지어요");
}

/// 낫다01(동사, 병이~) + 아요 → 나아요 (나요가 아님)
#[test]
fn test_postfix_word_siot_positive() {
    let yongeons = load_yongeons();
    let nat = find_yongeon(&yongeons, "낫다")
        .into_iter()
        .find(|y| y.dict_id == "01")
        .unwrap();
    assert_eq!(postfix_word(nat, &ah_eo::AYO), "나아요");
}

/// 짓다 + 었 → 지었
#[test]
fn test_postfix_word_siot_past() {
    let yongeons = load_yongeons();
    let jit = &find_yongeon(&yongeons, "짓다")[0];
    assert_eq!(postfix_word(jit, &ah_eo::ASS), "지었");
}

/// 농사짓다 + 어요 → 농사지어요
#[test]
fn test_postfix_word_siot_multi_syllable() {
    let yongeons = load_yongeons();
    let nongsa = &find_yongeon(&yongeons, "농사짓다")[0];
    assert_eq!(postfix_word(nongsa, &ah_eo::AYO), "농사지어요");
}

// --- postfix_word: ㅂ불규칙 ---
//
// 어간 끝 ㅂ이 우로 바뀌고, 규칙 축약(ㅜ+ㅏ→ㅘ, ㅜ+ㅓ→ㅝ)이 적용됩니다.
// 돕다/곱다만 양성(와), 나머지는 전부 음성(워)입니다.

/// 돕다(양성, 단음절 ㅗ) + 아요 → 도와요
#[test]
fn test_postfix_word_bieut_positive() {
    let yongeons = load_yongeons();
    let dop = &find_yongeon(&yongeons, "돕다")[0];
    assert_eq!(postfix_word(dop, &ah_eo::AYO), "도와요");
}

/// 춥다(음성) + 어요 → 추워요
#[test]
fn test_postfix_word_bieut_negative() {
    let yongeons = load_yongeons();
    let chup = &find_yongeon(&yongeons, "춥다")[0];
    assert_eq!(postfix_word(chup, &ah_eo::AYO), "추워요");
}

/// 가깝다(다음절, 음성) + 어요 → 가까워요
#[test]
fn test_postfix_word_bieut_multi_syllable() {
    let yongeons = load_yongeons();
    let gakkap = &find_yongeon(&yongeons, "가깝다")[0];
    assert_eq!(postfix_word(gakkap, &ah_eo::AYO), "가까워요");
}

/// 돕다 + 았 → 도왔
#[test]
fn test_postfix_word_bieut_past() {
    let yongeons = load_yongeons();
    let dop = &find_yongeon(&yongeons, "돕다")[0];
    assert_eq!(postfix_word(dop, &ah_eo::ASS), "도왔");
}

/// 아름답다(다음절, 음성) + 어요 → 아름다워요
#[test]
fn test_postfix_word_bieut_areumdap() {
    let yongeons = load_yongeons();
    let areumdap = &find_yongeon(&yongeons, "아름답다")[0];
    assert_eq!(postfix_word(areumdap, &ah_eo::AYO), "아름다워요");
}

// --- postfix_word: ㅡ 탈락 모음조화 ---
//
// 다음절 ㅡ 어간은 앞 음절 모음으로 양성/음성을 판별합니다.

/// 바쁘다: 앞 음절 바(ㅏ, 양성) → 아요 → 바빠요
#[test]
fn test_postfix_word_eu_positive() {
    let yongeons = load_yongeons();
    let bba = &find_yongeon(&yongeons, "바쁘다")[0];
    assert_eq!(postfix_word(bba, &ah_eo::AYO), "바빠요");
}

/// 예쁘다: 앞 음절 예(ㅔ, 음성) → 어요 → 예뻐요
#[test]
fn test_postfix_word_eu_negative() {
    let yongeons = load_yongeons();
    let yye = &find_yongeon(&yongeons, "예쁘다")[0];
    assert_eq!(postfix_word(yye, &ah_eo::AYO), "예뻐요");
}

/// 바쁘다 과거: 바쁘 + 았 → 바빴
#[test]
fn test_postfix_word_eu_positive_past() {
    let yongeons = load_yongeons();
    let bba = &find_yongeon(&yongeons, "바쁘다")[0];
    assert_eq!(postfix_word(bba, &ah_eo::ASS), "바빴");
}

// --- postfix_word: ㄷ불규칙 ---
//
// 어간 끝 ㄷ이 모음 어미 앞에서 ㄹ로 바뀌는지 검증합니다.

/// 걷다 + 어요 → 걸어요
#[test]
fn test_postfix_word_dieut_ayo() {
    let yongeons = load_yongeons();
    let geot = &find_yongeon(&yongeons, "걷다")[0];
    assert_eq!(postfix_word(geot, &ah_eo::AYO), "걸어요");
}

/// 듣다 + 어요 → 들어요
#[test]
fn test_postfix_word_dieut_deut() {
    let yongeons = load_yongeons();
    let deut = &find_yongeon(&yongeons, "듣다")[0];
    assert_eq!(postfix_word(deut, &ah_eo::AYO), "들어요");
}

/// 걷다 + 었 → 걸었
#[test]
fn test_postfix_word_dieut_past() {
    let yongeons = load_yongeons();
    let geot = &find_yongeon(&yongeons, "걷다")[0];
    assert_eq!(postfix_word(geot, &ah_eo::ASS), "걸었");
}

/// 묻다03(ㄷ불규칙, 길을 ~) + 어서 → 물어서
#[test]
fn test_postfix_word_dieut_reason() {
    let yongeons = load_yongeons();
    let mut_da = find_yongeon(&yongeons, "묻다")
        .into_iter()
        .find(|y| y.dict_id == "03")
        .unwrap();
    assert_eq!(postfix_word(mut_da, &ah_eo::ASEO), "물어서");
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

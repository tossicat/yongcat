//! ## postfix / postfix_word 통합 테스트
//!
//! 용언 활용 파이프라인(join → merge)이 실제 용언 데이터와 함께
//! 올바른 활용형을 생성하는지 검증합니다.
//!
//! - `postfix_word`: 단일 `Yongeon`에 어미를 적용하여 활용형 생성
//! - `postfix`: 단어 문자열로 동음이의어를 포함한 전체 활용형 생성

use yongcat::eomi::ah_eo;
use yongcat::{
    conjugate, conjugate_checked, find_yongeon, load_yongeons, lookup, lookup_all, postfix,
    postfix_word,
};

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

// --- postfix_word: 으불규칙 ---
//
// 규칙 활용의 ㅡ 탈락/모음조화로 처리됩니다 (별도 불규칙 모듈 불필요).

/// 잠그다 + 아요 → 잠가요 (앞 음절 ㅏ 양성)
#[test]
fn test_postfix_word_eu_jamgeu() {
    let yongeons = load_yongeons();
    let jamgeu = &find_yongeon(&yongeons, "잠그다")[0];
    assert_eq!(postfix_word(jamgeu, &ah_eo::AYO), "잠가요");
}

/// 끄다 + 어요 → 꺼요 (단음절 음성)
#[test]
fn test_postfix_word_eu_kkeu() {
    let yongeons = load_yongeons();
    let kkeu = &find_yongeon(&yongeons, "끄다")[0];
    assert_eq!(postfix_word(kkeu, &ah_eo::AYO), "꺼요");
}

/// 슬프다 + 어요 → 슬퍼요 (앞 음절 ㅡ→ㅓ 음성)
#[test]
fn test_postfix_word_eu_seulpeu() {
    let yongeons = load_yongeons();
    let seulpeu = &find_yongeon(&yongeons, "슬프다")[0];
    assert_eq!(postfix_word(seulpeu, &ah_eo::AYO), "슬퍼요");
}

/// 아프다 + 았 → 아팠 (앞 음절 ㅏ 양성 과거)
#[test]
fn test_postfix_word_eu_apeu_past() {
    let yongeons = load_yongeons();
    let apeu = &find_yongeon(&yongeons, "아프다")[0];
    assert_eq!(postfix_word(apeu, &ah_eo::ASS), "아팠");
}

// --- postfix_word: 러불규칙 ---
//
// 어미 "어"가 "러"로 바뀌고, ㅡ 탈락 없이 어간이 유지됩니다.

/// 이르다01(도착하다) + 어요 → 이르러요
#[test]
fn test_postfix_word_reo_ayo() {
    let yongeons = load_yongeons();
    let ireu = find_yongeon(&yongeons, "이르다")
        .into_iter()
        .find(|y| y.dict_id == "01")
        .unwrap();
    assert_eq!(postfix_word(ireu, &ah_eo::AYO), "이르러요");
}

/// 이르다01 + 었 → 이르렀
#[test]
fn test_postfix_word_reo_past() {
    let yongeons = load_yongeons();
    let ireu = find_yongeon(&yongeons, "이르다")
        .into_iter()
        .find(|y| y.dict_id == "01")
        .unwrap();
    assert_eq!(postfix_word(ireu, &ah_eo::ASS), "이르렀");
}

/// 푸르다 + 어요 → 푸르러요
#[test]
fn test_postfix_word_reo_pureu() {
    let yongeons = load_yongeons();
    let pureu = &find_yongeon(&yongeons, "푸르다")[0];
    assert_eq!(postfix_word(pureu, &ah_eo::AYO), "푸르러요");
}

// --- postfix_word: 우불규칙 ---
//
// 푸다 하나만 해당. ㅜ가 어미 첫 모음으로 대체됩니다.

/// 푸다 + 어요 → 퍼요
#[test]
fn test_postfix_word_u_ayo() {
    let yongeons = load_yongeons();
    let pu = &find_yongeon(&yongeons, "푸다")[0];
    assert_eq!(postfix_word(pu, &ah_eo::AYO), "퍼요");
}

/// 푸다 + 었 → 펐
#[test]
fn test_postfix_word_u_past() {
    let yongeons = load_yongeons();
    let pu = &find_yongeon(&yongeons, "푸다")[0];
    assert_eq!(postfix_word(pu, &ah_eo::ASS), "펐");
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

// --- postfix_word: ㅎ불규칙 ---
//
// 어간 끝 ㅎ이 모음 어미 앞에서 탈락하고,
// 아/어 계열 어미와 만나면 어간 끝 모음이 ㅐ로 축약됩니다.

/// 그렇다(음성) + 어요 → 그래요
#[test]
fn test_postfix_word_hieut_negative() {
    let yongeons = load_yongeons();
    let geureot = &find_yongeon(&yongeons, "그렇다")[0];
    assert_eq!(postfix_word(geureot, &ah_eo::AYO), "그래요");
}

/// 노랗다(양성) + 아요 → 노래요
#[test]
fn test_postfix_word_hieut_positive() {
    let yongeons = load_yongeons();
    let norat = &find_yongeon(&yongeons, "노랗다")[0];
    assert_eq!(postfix_word(norat, &ah_eo::AYO), "노래요");
}

/// 그렇다 + 었 → 그랬
#[test]
fn test_postfix_word_hieut_past() {
    let yongeons = load_yongeons();
    let geureot = &find_yongeon(&yongeons, "그렇다")[0];
    assert_eq!(postfix_word(geureot, &ah_eo::ASS), "그랬");
}

/// 하얗다(양성) + 아서 → 하얘서
#[test]
fn test_postfix_word_hieut_reason() {
    let yongeons = load_yongeons();
    let hayat = &find_yongeon(&yongeons, "하얗다")[0];
    assert_eq!(postfix_word(hayat, &ah_eo::ASEO), "하얘서");
}

/// 어떻다(음성) + 어요 → 어때요
#[test]
fn test_postfix_word_hieut_eotteo() {
    let yongeons = load_yongeons();
    let eotteo = &find_yongeon(&yongeons, "어떻다")[0];
    assert_eq!(postfix_word(eotteo, &ah_eo::AYO), "어때요");
}

/// 까맣다(양성) + 았 → 까맸
#[test]
fn test_postfix_word_hieut_kkamah_past() {
    let yongeons = load_yongeons();
    let kkamah = &find_yongeon(&yongeons, "까맣다")[0];
    assert_eq!(postfix_word(kkamah, &ah_eo::ASS), "까맸");
}

/// 이렇다 + 으면 → 이러면 (Plain, ㅎ 탈락)
#[test]
fn test_postfix_word_hieut_plain() {
    let yongeons = load_yongeons();
    let ireot = &find_yongeon(&yongeons, "이렇다")[0];
    assert_eq!(postfix_word(ireot, &yongcat::eomi::plain::EUMYEON), "이러면");
}

/// 그렇다 + 고 → 그렇고 (Fixed, 변환 없음)
#[test]
fn test_postfix_word_hieut_fixed() {
    let yongeons = load_yongeons();
    let geureot = &find_yongeon(&yongeons, "그렇다")[0];
    assert_eq!(postfix_word(geureot, &yongcat::eomi::fixed::GO), "그렇고");
}

// --- postfix_word: 르불규칙 ---
//
// 어간 끝 르가 분리되어 ㄹ이 앞 음절 받침으로 삽입되고,
// 어미 첫 초성이 ㄹ로 바뀝니다.

/// 모르다(양성) + 아요 → 몰라요
#[test]
fn test_postfix_word_reu_positive() {
    let yongeons = load_yongeons();
    let moreu = &find_yongeon(&yongeons, "모르다")[0];
    assert_eq!(postfix_word(moreu, &ah_eo::AYO), "몰라요");
}

/// 기르다(음성) + 어요 → 길러요
#[test]
fn test_postfix_word_reu_negative() {
    let yongeons = load_yongeons();
    let gireu = &find_yongeon(&yongeons, "기르다")[0];
    assert_eq!(postfix_word(gireu, &ah_eo::AYO), "길러요");
}

/// 모르다 + 았 → 몰랐
#[test]
fn test_postfix_word_reu_past() {
    let yongeons = load_yongeons();
    let moreu = &find_yongeon(&yongeons, "모르다")[0];
    assert_eq!(postfix_word(moreu, &ah_eo::ASS), "몰랐");
}

/// 빠르다(양성) + 아요 → 빨라요
#[test]
fn test_postfix_word_reu_adj() {
    let yongeons = load_yongeons();
    let bbareu = &find_yongeon(&yongeons, "빠르다")[0];
    assert_eq!(postfix_word(bbareu, &ah_eo::AYO), "빨라요");
}

/// 떠오르다(다음절) + 아요 → 떠올라요
#[test]
fn test_postfix_word_reu_multi_syllable() {
    let yongeons = load_yongeons();
    let tteoreu = &find_yongeon(&yongeons, "떠오르다")[0];
    assert_eq!(postfix_word(tteoreu, &ah_eo::AYO), "떠올라요");
}

/// 다르다 + 아서 → 달라서
#[test]
fn test_postfix_word_reu_reason() {
    let yongeons = load_yongeons();
    let dareu = &find_yongeon(&yongeons, "다르다")[0];
    assert_eq!(postfix_word(dareu, &ah_eo::ASEO), "달라서");
}

/// 모르다 + 면 → 모르면 (Plain, 변환 없음)
#[test]
fn test_postfix_word_reu_plain() {
    let yongeons = load_yongeons();
    let moreu = &find_yongeon(&yongeons, "모르다")[0];
    assert_eq!(postfix_word(moreu, &yongcat::eomi::plain::EUMYEON), "모르면");
}

/// 모르다 + 고 → 모르고 (Fixed, 변환 없음)
#[test]
fn test_postfix_word_reu_fixed() {
    let yongeons = load_yongeons();
    let moreu = &find_yongeon(&yongeons, "모르다")[0];
    assert_eq!(postfix_word(moreu, &yongcat::eomi::fixed::GO), "모르고");
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

// --- 편의 API: lookup, lookup_all, conjugate ---
//
// 전역 데이터를 사용하는 편의 함수가 올바르게 동작하는지 검증합니다.

/// lookup: 기본형으로 검색하면 해당 용언을 반환합니다.
#[test]
fn test_lookup() {
    let verb = lookup("먹다");
    assert_eq!(verb.base_form, "먹다");
}

/// lookup_all: 동음이의어가 있는 단어는 여러 결과를 반환합니다.
#[test]
fn test_lookup_all_homonyms() {
    let results = lookup_all("걷다");
    assert!(results.len() >= 2, "걷다는 동음이의어가 2개 이상이어야 함");
}

/// lookup_all: 동음이의어가 없는 단어도 Vec으로 반환합니다.
#[test]
fn test_lookup_all_single() {
    let results = lookup_all("가다");
    assert!(!results.is_empty());
}

/// lookup: 존재하지 않는 단어를 넣으면 패닉합니다.
#[test]
#[should_panic(expected = "용언을 찾을 수 없습니다")]
fn test_lookup_nonexistent() {
    lookup("없는단어다");
}

/// conjugate는 postfix_word와 동일한 결과를 반환합니다.
#[test]
fn test_conjugate_equals_postfix_word() {
    let verb = lookup("먹다");
    assert_eq!(conjugate(verb, &ah_eo::AYO), postfix_word(verb, &ah_eo::AYO));
}

/// lookup + conjugate 조합: 공격하다 + 았습니다 → 공격했습니다
#[test]
fn test_conjugate_with_lookup() {
    assert_eq!(conjugate(lookup("공격하다"), &ah_eo::ASS_SEUMNIDA), "공격했습니다");
}

/// 하다를 분리해서 명사와 조합: "공격" + 했습니다 → "공격했습니다"
#[test]
fn test_conjugate_hada_separated() {
    let ha = lookup("하다");
    let result = format!("공격{}", conjugate(ha, &ah_eo::ASS_SEUMNIDA));
    assert_eq!(result, "공격했습니다");
}

// --- conjugate_checked: 품사별 어미 제한 ---
//
// 형용사에 동사 전용 어미를 적용하면 Err을 반환합니다.
// 동사에는 모든 어미를 적용할 수 있습니다.

/// 동사 + 동사 전용 어미 → Ok
#[test]
fn test_checked_verb_with_verb_only_eomi() {
    let verb = lookup("먹다");
    assert!(conjugate_checked(verb, &yongcat::NEUN).is_ok());
    assert_eq!(conjugate_checked(verb, &yongcat::NEUN).unwrap(), "먹는");
}

/// 형용사 + 동사 전용 어미 → Err
#[test]
fn test_checked_adj_with_verb_only_eomi() {
    let adj = lookup("예쁘다");
    assert!(conjugate_checked(adj, &yongcat::NEUN).is_err());
}

/// 형용사 + 공용 어미 → Ok
#[test]
fn test_checked_adj_with_common_eomi() {
    let adj = lookup("예쁘다");
    assert!(conjugate_checked(adj, &ah_eo::AYO).is_ok());
    assert_eq!(conjugate_checked(adj, &ah_eo::AYO).unwrap(), "예뻐요");
}

/// 동사 전용 어미 10개 모두 형용사에서 Err 반환
#[test]
fn test_checked_all_verb_only_eomis() {
    let adj = lookup("예쁘다");
    let verb_only = [
        &yongcat::ABODA, &yongcat::ADALLA, &yongcat::NEUN,
        &yongcat::JA, &yongcat::NEUNDE, &yongcat::EURYEOGO,
        &yongcat::EUREO, &yongcat::EULGE, &yongcat::EULLAE,
        &yongcat::EUPSIDA,
    ];
    for eomi in verb_only {
        assert!(
            conjugate_checked(adj, eomi).is_err(),
            "형용사에 동사 전용 어미가 통과되었습니다"
        );
    }
}

/// 동사 전용 어미 10개 모두 동사에서 Ok 반환
#[test]
fn test_checked_all_verb_only_eomis_with_verb() {
    let verb = lookup("먹다");
    let verb_only = [
        &yongcat::ABODA, &yongcat::ADALLA, &yongcat::NEUN,
        &yongcat::JA, &yongcat::NEUNDE, &yongcat::EURYEOGO,
        &yongcat::EUREO, &yongcat::EULGE, &yongcat::EULLAE,
        &yongcat::EUPSIDA,
    ];
    for eomi in verb_only {
        assert!(
            conjugate_checked(verb, eomi).is_ok(),
            "동사에 동사 전용 어미가 거부되었습니다"
        );
    }
}

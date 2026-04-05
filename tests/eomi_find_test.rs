//! ## 어미 검색 통합 테스트
//!
//! `load_eomis()`로 빌드 시 생성된 어미 목록을 로드한 뒤,
//! `find_eomi`와 `find_eomi_exact`가 올바른 결과를 반환하는지 검증합니다.

use yongcat::{find_eomi, find_eomi_exact, load_eomis};

// --- load_eomis ---

#[test]
fn test_load_eomis_count() {
    // 전체 어미 상수 42개 (AhEo 12 + Fixed 14 + Plain 16)
    let eomis = load_eomis();
    assert_eq!(eomis.len(), 46);
}

// --- find_eomi ---

#[test]
fn test_find_ah_eo() {
    // "어요" → AYO (AhEo의 음성 형태와 정확 일치)
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "어요");
    let names: Vec<&str> = results.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"AYO"));
}

#[test]
fn test_find_positive_form() {
    // "아요" → AYO (AhEo의 양성 형태와 정확 일치)
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "아요");
    let names: Vec<&str> = results.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"AYO"));
}

#[test]
fn test_find_fixed() {
    // "고" → GO (Fixed와 정확 일치)
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "고");
    let names: Vec<&str> = results.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"GO"));
}

#[test]
fn test_find_plain() {
    // "은" → EUN (Plain의 받침형과 정확 일치)
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "은");
    let names: Vec<&str> = results.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"EUN"));
}

#[test]
fn test_find_multiple_matches() {
    // "었어요" → ASS_EOYO (form.1과 정확 일치)
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "었어요");
    let names: Vec<&str> = results.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"ASS_EOYO"));
}

#[test]
fn test_find_nonexistent() {
    // 존재하지 않는 어미는 빈 결과
    let eomis = load_eomis();
    let results = find_eomi(&eomis, "없는어미");
    assert!(results.is_empty());
}

// --- find_eomi_exact ---

#[test]
fn test_find_eomi_exact_plain() {
    // "세요" → EUSEYO (Plain의 무받침형과 정확 일치)
    let eomi = find_eomi_exact("세요");
    assert!(eomi.is_some());
}

#[test]
fn test_find_eomi_exact_ah_eo() {
    // "었" → ASS (form.1과 정확 일치, ASS_EOYO "었어요"는 불일치)
    let eomi = find_eomi_exact("었");
    assert!(eomi.is_some());
}

#[test]
fn test_find_eomi_exact_fixed() {
    // "다" → DA (Fixed와 정확 일치)
    let eomi = find_eomi_exact("다");
    assert!(eomi.is_some());
}

#[test]
fn test_find_eomi_exact_nonexistent() {
    // 존재하지 않는 어미는 None
    assert!(find_eomi_exact("없는어미").is_none());
}

#[test]
fn test_find_eomi_exact_conjugate() {
    // find_eomi_exact + conjugate 조합으로 활용형 생성
    let verb = yongcat::lookup("쉬다");
    let eomi = find_eomi_exact("세요").unwrap();
    assert_eq!(yongcat::conjugate(verb, eomi), "쉬세요");
}

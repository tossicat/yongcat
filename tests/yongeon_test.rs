use yongcat::{find_eogan, find_yongeon, load_yongeons, IrregularType, YongeonType};

#[test]
fn test_find_existing_verb() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "먹다");
    assert!(!results.is_empty());
    assert!(results.iter().all(|y| y.base_form == "먹다"));
    assert!(results.iter().all(|y| y.yongeon_type == YongeonType::Verb));
}

#[test]
fn test_find_existing_adjective() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "가깝다");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].yongeon_type, YongeonType::Adjective);
    assert_eq!(results[0].irregular_type, IrregularType::Bieut);
}

#[test]
fn test_find_homonyms() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "쓰다");
    assert!(results.len() >= 2, "쓰다는 동음이의어가 2개 이상이어야 함");
}

#[test]
fn test_find_nonexistent() {
    let yongeons = load_yongeons();
    let results = find_yongeon(&yongeons, "없는단어다");
    assert!(results.is_empty());
}

#[test]
fn test_find_eogan_existing() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "먹");
    assert!(!results.is_empty());
    assert!(results.iter().all(|y| y.eogan_str() == "먹"));
}

#[test]
fn test_find_eogan_irregular() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "가깝");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].irregular_type, IrregularType::Bieut);
}

#[test]
fn test_find_eogan_nonexistent() {
    let yongeons = load_yongeons();
    let results = find_eogan(&yongeons, "없는어간");
    assert!(results.is_empty());
}

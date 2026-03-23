const HANGUL_BASE: u32 = 0xAC00;
const ONSET_COUNT: u32 = 19;
const VOWEL_COUNT: u32 = 21;
const CODA_COUNT: u32 = 28;

const ONSETS: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ',
    'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

const VOWELS: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ',
    'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ',
    'ㅣ',
];

const CODAS: [Option<char>; 28] = [
    None,
    Some('ㄱ'), Some('ㄲ'), Some('ㄳ'), Some('ㄴ'), Some('ㄵ'), Some('ㄶ'),
    Some('ㄷ'), Some('ㄹ'), Some('ㄺ'), Some('ㄻ'), Some('ㄼ'), Some('ㄽ'),
    Some('ㄾ'), Some('ㄿ'), Some('ㅀ'), Some('ㅁ'), Some('ㅂ'), Some('ㅄ'),
    Some('ㅅ'), Some('ㅆ'), Some('ㅇ'), Some('ㅈ'), Some('ㅊ'), Some('ㅋ'),
    Some('ㅌ'), Some('ㅍ'), Some('ㅎ'),
];

#[derive(Debug, Clone, PartialEq)]
pub struct Syllable {
    pub onset: char,
    pub vowel: char,
    pub coda: Option<char>,
}

impl Syllable {
    pub fn has_coda(&self) -> bool {
        self.coda.is_some()
    }

    pub fn is_positive_vowel(&self) -> bool {
        matches!(self.vowel, 'ㅏ' | 'ㅗ')
    }
}

fn is_hangul(c: char) -> bool {
    let code = c as u32;
    code >= HANGUL_BASE && code < HANGUL_BASE + ONSET_COUNT * VOWEL_COUNT * CODA_COUNT
}

fn split(c: char) -> Option<Syllable> {
    if !is_hangul(c) {
        return None;
    }
    let code = c as u32 - HANGUL_BASE;
    let onset_idx = code / (VOWEL_COUNT * CODA_COUNT);
    let vowel_idx = (code % (VOWEL_COUNT * CODA_COUNT)) / CODA_COUNT;
    let coda_idx = code % CODA_COUNT;

    Some(Syllable {
        onset: ONSETS[onset_idx as usize],
        vowel: VOWELS[vowel_idx as usize],
        coda: CODAS[coda_idx as usize],
    })
}

fn join(s: &Syllable) -> char {
    let onset_idx = ONSETS.iter().position(|&c| c == s.onset).unwrap() as u32;
    let vowel_idx = VOWELS.iter().position(|&c| c == s.vowel).unwrap() as u32;
    let coda_idx = match s.coda {
        None => 0,
        Some(c) => CODAS.iter().position(|&x| x == Some(c)).unwrap() as u32,
    };
    char::from_u32(HANGUL_BASE + (onset_idx * VOWEL_COUNT + vowel_idx) * CODA_COUNT + coda_idx)
        .unwrap()
}

pub fn decompose(s: &str) -> Vec<Syllable> {
    s.chars().filter_map(split).collect()
}

pub fn compose(syllables: &[Syllable]) -> String {
    syllables.iter().map(join).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_with_coda() {
        let s = split('먹').unwrap();
        assert_eq!(s.onset, 'ㅁ');
        assert_eq!(s.vowel, 'ㅓ');
        assert_eq!(s.coda, Some('ㄱ'));
    }

    #[test]
    fn test_split_without_coda() {
        let s = split('가').unwrap();
        assert_eq!(s.onset, 'ㄱ');
        assert_eq!(s.vowel, 'ㅏ');
        assert_eq!(s.coda, None);
    }

    #[test]
    fn test_split_non_hangul() {
        assert!(split('A').is_none());
        assert!(split('1').is_none());
    }

    #[test]
    fn test_join_with_coda() {
        let s = Syllable { onset: 'ㅁ', vowel: 'ㅓ', coda: Some('ㄱ') };
        assert_eq!(join(&s), '먹');
    }

    #[test]
    fn test_join_without_coda() {
        let s = Syllable { onset: 'ㄱ', vowel: 'ㅏ', coda: None };
        assert_eq!(join(&s), '가');
    }

    #[test]
    fn test_decompose_compose_roundtrip() {
        let words = ["먹다", "가깝", "아름답", "예쁘"];
        for word in words {
            let syllables = decompose(word);
            let result = compose(&syllables);
            assert_eq!(result, word, "라운드트립 실패: {}", word);
        }
    }

    #[test]
    fn test_decompose_skips_non_hangul() {
        let syllables = decompose("AB먹C");
        assert_eq!(syllables.len(), 1);
        assert_eq!(compose(&syllables), "먹");
    }

    #[test]
    fn test_has_coda() {
        assert!(split('먹').unwrap().has_coda());
        assert!(!split('가').unwrap().has_coda());
    }

    #[test]
    fn test_is_positive_vowel() {
        assert!(split('가').unwrap().is_positive_vowel());
        assert!(split('오').unwrap().is_positive_vowel());
        assert!(!split('먹').unwrap().is_positive_vowel());
        assert!(!split('크').unwrap().is_positive_vowel());
    }
}

# 작업 리뷰

## 프로젝트 구조

```
yongcat/
├── Cargo.toml
├── build.rs
├── .gitattributes             # data/*.csv LF 강제
├── .gitignore
├── about_data.md              # 데이터 설명 문서
├── README.md                  # 프로젝트 소개, 사용법, 활용 규칙 현황, 한계
├── QUICKSTART.md              # 시작 가이드
├── RULES.md                   # 활용 규칙 구현 가이드
├── REVIEW.md                  # 이 파일
├── USER_DATA.md               # 사용자 용언 데이터 추가 가이드
├── build/
│   └── validate.rs            # CSV 행 검증 로직
├── data/
│   ├── yong_list.csv          # 용언 1,721개
│   └── example.csv            # 사용자 CSV 형식 참고용 예시
├── tests/
│   ├── eomi_ah_eo_test.rs     # 아/어 계열 어미 테스트 (26개)
│   ├── eomi_find_test.rs      # 어미 검색 테스트 (12개)
│   ├── eomi_fixed_test.rs     # 고정 형태 어미 테스트 (24개)
│   ├── eomi_plain_test.rs     # 받침 유무 어미 테스트 (45개)
│   ├── postfix_test.rs        # 활용 파이프라인 통합 테스트 (76개)
│   └── yongeon_test.rs        # 용언 검색 통합 테스트 (7개)
└── src/
    ├── lib.rs                 # 크레이트 루트, lookup, conjugate, find_eomi_exact, postfix, postfix_word
    ├── bin/
    │   └── import.rs          # CLI: 사용자 CSV 검증 및 user_list.csv 생성
    ├── eomi/
    │   ├── mod.rs             # Eomi 열거형, AhEoForm 타입 (2-튜플), Eomi::matches()
    │   ├── ah_eo.rs           # 아/어 계열 어미 상수 13개
    │   ├── fixed.rs           # 고정 형태 어미 상수 16개
    │   └── plain.rs           # 받침 유무 어미 상수 17개
    ├── irregular/
    │   ├── mod.rs             # 불규칙 유형별 디스패치
    │   ├── yeo.rs             # 여불규칙
    │   ├── dieut.rs           # ㄷ불규칙
    │   ├── bieut.rs           # ㅂ불규칙
    │   ├── siot.rs            # ㅅ불규칙
    │   ├── hieut.rs           # ㅎ불규칙
    │   ├── rieul.rs           # ㄹ불규칙
    │   ├── reu.rs             # 르불규칙
    │   ├── u.rs               # 우불규칙
    │   ├── reo.rs             # 러불규칙
    │   └── eu.rs              # 으불규칙 (문서만, 규칙 활용으로 처리)
    ├── join.rs                # select → irregular → regular
    ├── merge.rs               # apply → irregular → regular
    ├── syllable.rs            # 한글 음절 분해/합성, starts_with_vowel(), combine_jamo()
    ├── types.rs               # YongeonType, IrregularType 열거형
    └── yongeon.rs             # Yongeon 구조체, moeum_joha()

```

## 파이프라인

```
select ──→ irregular::join ──→ Some이면 반환
           │
           └→ regular        ──→ moeum_joha() + 접합

apply  ──→ irregular::merge ──→ Some이면 반환
           │
           └→ regular        ──→ 모음 축약/탈락
```

## 구현 현황

### 어미

| 유형 | 파일 | 상수 |
|------|------|------|
| AhEo (아/어 계열) | `ah_eo.rs` | A, AYO, ASEO, ADO, AYA, ARA, ASS, ASS_EOYO, ASS_SEUMNIDA, AJIDA, ABODA, ADALLA, AJUDA (13개) |
| Fixed (고정 형태) | `fixed.rs` | GO, JI, NEUN, GE, JA, GESS_EOYO, GESS_SEUMNIDA, DA, JIMAN, GEONA, NEUNDE, DAGA, DOROK, DAMYEON, NA, NYA (16개) |
| Plain (받침 유무) | `plain.rs` | EUN, EUL, EUMYEON, EUNI, SEUMNIDA, EUMYEONSEO, EURYEOGO, EUSEYO, EUREO, EUSYEOSS, EUNIKKA, EULKKA, EULGE, EULLAE, EULSSUROK, EUPSIDA, NEUNDA (17개) |

`AhEoForm`은 2-튜플 `(양성, 음성)`입니다. 여불규칙의 여 형태는 음성 형태에서 ㅓ→ㅕ 변환으로 생성합니다.

### 규칙 활용

| 규칙 | 단계 | 구현 위치 |
|------|------|-----------|
| 모음조화 | join | `Yongeon::moeum_joha()` → `is_positive_harmony()` |
| ㅡ 어간 모음조화 | join | `Yongeon::is_positive_harmony()` |
| 모음 축약 6종 | merge | `merge.rs` contract_ah_eo() |
| ㅡ 탈락 | merge | `merge.rs` contract_ah_eo() |
| 종성 이전 | merge | `merge.rs` contract_ah_eo() |
| 자모 합성 | postfix_word | `syllable.rs` combine_jamo() |

### 불규칙 활용

| 유형 | 용언 수 | join | merge | 파일 |
|------|---------|------|-------|------|
| 여불규칙 | 610개 | form.1에서 ㅓ→ㅕ 생성 | ㅏ→ㅐ 축약 | `yeo.rs` |
| ㄷ불규칙 | 6개 | ㄷ→ㄹ 변환 (AhEo/Plain) | None | `dieut.rs` |
| ㅂ불규칙 | 66개 | ㅂ→우 + 특수 모음조화 (AhEo/Plain) | 우+어미 축약 | `bieut.rs` |
| ㅅ불규칙 | 9개 | ㅅ 탈락 (AhEo/Plain) | 축약 억제 | `siot.rs` |
| ㄹ불규칙 | 73개 | ㄴ/ㅂ/ㅅ 앞 ㄹ 탈락 (Plain/Fixed) | None | `rieul.rs` |
| 우불규칙 | 1개 | None | ㅜ→어미 첫 모음 대체 | `u.rs` |
| ㅎ불규칙 | 13개 | ㅎ 탈락 (AhEo/Plain) | ㅐ/ㅒ 축약 | `hieut.rs` |
| 러불규칙 | 3개 | 어→러 변환 | 축약 억제 | `reo.rs` |
| 르불규칙 | 36개 | ㄹ 삽입 + 초성 교체 (AhEo) | 축약 억제 | `reu.rs` |
| 으불규칙 | 24개 | (규칙 활용으로 처리) | (규칙 활용으로 처리) | `eu.rs` (문서만) |

### 기타

- `build.rs`: CSV → `load_yongeons()`, 소스 파싱 → `load_eomis()` 코드 생성, 등급 필터링, `user_list.csv` 자동 로드
- `build/validate.rs`: CSV 행 검증 로직 (헤더, 컬럼 수, 기본형 한글, 어간 일치, 품사, 활용 유형, dict_id 숫자, grade A/B/C)
- `src/bin/import.rs`: 사용자 CSV 검증 CLI (`cargo run --bin import`)
- `syllable.rs`: 한글 유니코드 분해/합성, `starts_with_vowel()`, `combine_jamo()`, 외부 의존성 없음
- `Yongeon::new()`: 빈 어간 assert 포함
- `Yongeon::moeum_joha()`: 모음조화 판별 통일 메서드
- `Eomi::matches()`: 문자열 매칭 메서드
- `find_eomi()`: 어미 문자열 검색
- `lookup()` / `conjugate()` / `find_eomi_exact()`: 편의 API (전역 데이터 사용)
- `conjugate_checked()`: 품사별 어미 제한 검사 (형용사 + 동사 전용 어미 → Err)
- 어미 상수 플랫 re-export: `use yongcat::*;`로 `AYO`, `GO`, `EUN` 등 직접 접근

## 테스트 현황

총 306개 테스트 통과

| 위치 | 테스트 수 | 내용 |
|------|-----------|------|
| `syllable.rs` | 13개 | 분해/합성, 라운드트립, 비한글, 받침/양성모음, 자모 합성 |
| `yongeon.rs` | 12개 | 생성, 어간 분석, 품사/활용 판별, Display |
| `join.rs` | 6개 | AhEo 모음조화, Plain 받침 유무, Fixed |
| `merge.rs` | 13개 | 모음 축약 6종, 과거 시제 종성 이전, 받침 통과 |
| `irregular/yeo.rs` | 7개 | join/merge × AhEo/Plain/Fixed |
| `irregular/dieut.rs` | 5개 | join × AhEo/Past/Plain모음/Plain자음/Fixed |
| `irregular/bieut.rs` | 10개 | join/merge × 양성/음성/다음절/과거/Plain모음/Plain자음 |
| `irregular/siot.rs` | 9개 | join/merge × 축약 억제/Plain모음/Plain자음 |
| `irregular/hieut.rs` | 11개 | join/merge × 양성/음성/과거/Plain모음/Plain자음/Fixed |
| `irregular/rieul.rs` | 9개 | join × AhEo/Plain5종/Fixed2종/다음절 |
| `irregular/reu.rs` | 11개 | join/merge × 양성/음성/과거/다음절/형용사/Plain/Fixed |
| `irregular/u.rs` | 3개 | merge × AhEo/Past/Plain |
| `irregular/reo.rs` | 5개 | join/merge × AhEo/Past/Plain |
| `tests/eomi_ah_eo_test.rs` | 26개 | 규칙 8 + 불규칙 12 + 새 어미 6 |
| `tests/eomi_find_test.rs` | 12개 | load_eomis 개수, find_eomi 검색, find_eomi_exact |
| `tests/eomi_fixed_test.rs` | 24개 | 상수 × 받침 유무 + ㄹ불규칙 + 새 어미 |
| `tests/eomi_plain_test.rs` | 45개 | 상수 × 받침 유무 + 불규칙 + 새 어미 + 자모 합성×불규칙 |
| `tests/postfix_test.rs` | 76개 | 규칙/불규칙/ㅡ탈락/동음이의어/편의API/품사제한 통합 |
| `tests/yongeon_test.rs` | 7개 | find_yongeon, find_eogan |
| doctest | 2개 | Yongeon::new 예시, conjugate_checked 예시 |

## 다른 프로젝트에서 사용

라이브러리로서 필요한 조건을 갖추고 있으며, 다른 프로젝트에서 바로 사용할 수 있습니다.

- 306개 테스트 전체 통과, clippy 경고 0개
- 외부 의존성 없음 (순수 Rust)
- `cargo publish --dry-run` 통과 (44개 파일, 273.4KB)
- MIT 라이선스

### git 의존성으로 사용

```toml
[dependencies]
yongcat = { git = "https://github.com/tossicat/yongcat" }
```

### crates.io에서 사용 (배포 후)

```toml
[dependencies]
yongcat = "0.8"
```

## 미구현 사항

- 등급 필터 테스트 부재

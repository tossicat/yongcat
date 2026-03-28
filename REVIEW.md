# 작업 리뷰

## 프로젝트 구조

```
yongcat/
├── Cargo.toml
├── build.rs
├── .gitattributes             # data/*.csv LF 강제
├── about_data.md              # 데이터 설명 문서
├── README.md                  # 프로젝트 소개, 사용법, 모듈 구조
├── QUICKSTART.md              # 시작 가이드
├── REVIEW.md                  # 이 파일
├── data/
│   └── yong_list.csv          # 용언 1,721개 (base_form, dict_id, eogan, pos, conjugation, usage, grade)
├── tests/
│   ├── yongeon_test.rs        # find_yongeon, find_eogan 통합 테스트
│   └── postfix_test.rs        # postfix, postfix_word 통합 테스트
└── src/
    ├── lib.rs                 # 크레이트 루트 + load_yongeons, find_yongeon, find_eogan, postfix, postfix_word
    ├── eomi.rs                # 어미 데이터 (AhEoForm, AH_EO_GROUP, EomiGroup)
    ├── join.rs                # 모음조화 판별 + 어간-어미 접합 (1·2단계)
    ├── merge.rs               # 음운 축약/탈락 처리 (3단계)
    ├── syllable.rs            # 한글 음절 분해/합성
    ├── types.rs               # YongeonType, IrregularType 열거형
    └── yongeon.rs             # Yongeon 구조체
```

## 완료된 작업

### 1. `src/types.rs` — 타입 정의

- `YongeonType`: 동사(Verb), 형용사(Adjective)
- `IrregularType`: 규칙(Regular) 포함 11종 (ㄷ, ㅂ, ㅎ, ㄹ, ㅅ, 르, 우, 여, 러, 으 불규칙)
- 각각 `Display` 트레이트 구현 (한글 출력)

### 2. `src/syllable.rs` — 한글 음절 처리

- 한글 유니코드 공식으로 직접 구현 (외부 의존성 없음)
- `Syllable` 구조체: `onset`(초성), `vowel`(중성), `coda`(종성, `Option<char>`)
- `decompose(&str) -> Vec<Syllable>`: 문자열 → 음절 분해
- `compose(&[Syllable]) -> String`: 음절 → 문자열 합성
- 비한글 문자는 `decompose` 시 건너뜀
- doc 주석 추가 완료

### 3. `src/yongeon.rs` — 용언 구조체

- `Yongeon<'a>`: `base_form`, `dict_id`는 `&'a str`로 빌림
- `new(base_form, dict_id, eogan, yongeon_type, irregular_type)`: CSV의 eogan 컬럼을 직접 받아 음절 분해
- 어간 분석 메서드: `last_syllable()`, `has_coda()`, `is_positive_vowel()`
- 품사/활용 판별: `is_verb()`, `is_adjective()`, `is_regular()`, `is_irregular()`
- 모듈 문서 및 구조체 문서 보강 완료

### 4. `src/eomi.rs` — 어미 데이터

- `AhEoForm`: 아/어 계열 어미 튜플 타입 (양성모음, 음성모음, "하다"용)
- `AH_EO_GROUP`: 7개 아/어 계열 어미 상수 (해라체, 해요체, 연결, 명령, 과거 시제)
- `EomiGroup` 열거형: `AhEo`, `Plain`, `Fixed` 세 가지 어미 그룹 분류
- 모듈 문서 추가 완료

### 5. `src/join.rs` — 어간-어미 결합 (1·2단계)

- `select(yongeon, group) -> String`: 어미 그룹에서 적절한 어미를 선택하여 어간과 접합
- `AhEo` 분기: 모음조화 판별 ("하다" → 3번째, 양성모음 → 1번째, 그 외 → 2번째)
- `Plain` 분기: 받침 유무로 선택
- `Fixed` 분기: 고정 형태 그대로 접합

### 6. `src/merge.rs` — 음운 축약/탈락 (3단계)

- `apply(yongeon, joined, group) -> String`: join 결과에 음운 규칙 적용
- 모음 축약 규칙: ㅏ+ㅏ→ㅏ, ㅗ+ㅏ→ㅘ, ㅜ+ㅓ→ㅝ, ㅣ+ㅓ→ㅕ, ㅓ+ㅓ→ㅓ
- ㅡ 탈락: ㅡ+ㅓ→ㅓ (크다→커요)
- "하다" 축약: 하+여→해 (하다→해요)
- 종성 이전: 과거 시제 어미(았/었/였)의 ㅆ을 어간으로 이전 (가+았→갔)
- 받침 있는 어간은 축약 없이 통과 (먹+어요→먹어요)

### 7. `src/lib.rs` — 크레이트 루트

- 모듈 선언: `eomi`, `join`, `merge`, `syllable`, `types`, `yongeon`
- pub re-export: `Yongeon`, `YongeonType`, `IrregularType`, `EomiGroup`
- `include!`로 build.rs가 생성한 `load_yongeons()` 포함
- `find_yongeon()`: 기본형으로 용언 검색 (동음이의어 복수 반환)
- `find_eogan()`: 어간으로 용언 검색 (동음이의어 복수 반환)
- `postfix(yongeons, word, group)`: 단어 문자열로 동음이의어 전체 활용형 생성, `Vec<(&Yongeon, String)>` 반환
- `postfix_word(yongeon, group)`: 단일 용언 활용형 생성, `String` 반환
- 모듈 문서 추가 완료

### 8. `build.rs` — 빌드 스크립트

- `data/yong_list.csv`를 읽어 `$OUT_DIR/yong_data.rs`에 `load_yongeons()` 함수 생성
- CSV 7컬럼 대응 (base_form, dict_id, eogan, pos, conjugation, usage, grade)
- conjugation은 약칭 사용 (규, 여, ㄹ, ㅂ, 르, 으, ㅎ, ㄷ, ㅅ, 러, 우)
- CSV 경로를 `CSV_PATH` 상수로 관리
- CSV 변경 시 자동 재실행 (`cargo:rerun-if-changed`)
- 잘못된 품사/활용 유형이 있으면 빌드 실패 (panic)

### 9. 데이터

- `data/yong_list.csv`: 7컬럼 (base_form, dict_id, eogan, pos, conjugation, usage, grade), LF 줄바꿈
- `.gitattributes`: `data/*.csv`에 LF 줄바꿈 자동 유지
- `about_data.md`: 원본 출처, 변환 과정, 유니코드 정규화, 컬럼 설명, 활용 유형별 개수, 고유 키 정의

### 10. 문서

- `README.md`: 프로젝트 소개, 사용법 예시, 어미 그룹 표, 모듈 구조, 등급별 컴파일
- `QUICKSTART.md`: 빌드 스크립트 설명, 용언 검색/활용 예시, 반환값 필드 표, 어미 목록 표

### 11. 기타

- `.gitignore`: `.DS_Store`, `docs/` 추가

## 미구현 사항

- 불규칙 활용 처리 (ㄷ, ㅂ, ㅅ, ㅎ, ㄹ, 르, 우 등) — merge 모듈에 추가 필요
- ㅡ 탈락 시 앞 음절 모음조화 (바쁘다→바빠요) — join 모듈 수정 필요
- `Plain`, `Fixed` 그룹의 merge 처리 — 현재는 축약 없이 통과

## 코드 리뷰 (2026-03-28)

### 우선순위 높음

#### 1. 불규칙 활용 미구현 — merge.rs

받침이 있는 불규칙 용언은 현재 축약 없이 그대로 반환됩니다.

```rust
// merge.rs:36 — 받침 있으면 무조건 통과
if yongeon.has_coda() {
    return joined.to_string();
}
```

예: 걷다(ㄷ불규칙) + 어요 → "걷어요" 반환 (정답: "걸어요")

영향 범위: CSV 1,721개 중 불규칙+받침 용언 전체 (ㄷ, ㅂ, ㅅ, ㅎ 불규칙 등)

#### 2. ㅡ 탈락 시 앞 음절 모음조화 — join.rs

ㅡ로 끝나는 다음절 어간은 앞 음절 모음으로 모음조화를 판별해야 합니다.

```
바쁘다: 앞 음절 "바"(ㅏ, 양성) → "아요" 선택 → 바빠요 (정답)
현재:   "쁘"(ㅡ, 음성) → "어요" 선택 → 바뻐요 (오답)
```

단음절(크다, 쓰다 등)은 정상 동작합니다.

#### 3. 문서 주석 스타일 불일치

`syllable.rs`와 `types.rs`의 문서가 반말체("~한다", "~이다")로 작성되어 있습니다. 프로젝트 규칙은 높임말(합니다체)입니다.

| 파일 | 현재 | 수정 필요 |
|------|------|-----------|
| `syllable.rs` | "~한다", "~이다" | "~합니다", "~입니다" |
| `types.rs` | doc comment 없음 (모듈 문서) | 모듈 문서 추가 |
| `yongeon.rs` | 메서드 doc이 반말체 | 높임말로 통일 |
| `lib.rs` 기존 함수 | "~한다" | "~합니다" |

#### 4. 불규칙 활용 테스트 부재

테스트가 모두 규칙 활용 용언(가다, 먹다, 하다 등)만 사용합니다. 불규칙 구현 시 다음 테스트 추가 필요:

- ㄷ불규칙: 걷다 → 걸어요
- ㅂ불규칙: 돕다 → 도와요, 가깝다 → 가까워요
- ㅅ불규칙: 짓다 → 지어요
- ㅎ불규칙: 노랗다 → 노래요
- ㄹ불규칙: 살다 (ㄹ탈락 케이스)
- 르불규칙: 모르다 → 몰라요
- 우불규칙: 푸다 → 퍼요

### 우선순위 중간

#### 5. `join::select()`과 `merge::apply()`의 pub 가시성

이 함수들은 내부 파이프라인 단계이지만 `pub`으로 노출되어 있습니다. 외부 사용자가 직접 호출할 필요가 없다면 `pub(crate)`로 변경하거나, 고급 사용자용이면 문서에 명시해야 합니다.

#### 6. `yongeon.rs` — 빈 어간 방어 없음

```rust
pub fn new(..., eogan: &str, ...) -> Self {
    // eogan이 빈 문자열이면 last_syllable()에서 panic
}
```

`new()`에 `assert!(!eogan.is_empty())`를 추가하면 panic 위치가 명확해집니다.

#### 7. build.rs — Clippy 경고

```rust
// 현재: expect 안에서 format! 호출 (불필요한 할당)
.expect(&format!("{}를 열 수 없습니다", CSV_PATH));

// 개선: unwrap_or_else 사용
.unwrap_or_else(|_| panic!("{}를 열 수 없습니다", CSV_PATH));
```

### 우선순위 낮음

#### 8. 등급 필터 테스트 부재

`--features grade-a` 등의 필터링이 실제로 올바르게 동작하는지 검증하는 테스트가 없습니다.

#### 9. QUICKSTART.md에 join/merge 미기재

`join::select()`과 `merge::apply()`가 pub이면 QUICKSTART.md에도 설명이 필요합니다. pub(crate)로 변경하면 이 항목은 불필요합니다.

#### 10. "하다" 판별 로직 분산

`join.rs`와 `merge.rs` 양쪽에서 `yongeon.base_form.ends_with("하다")`로 판별합니다. `Yongeon`에 `is_hada()` 메서드를 추가하면 로직이 한 곳에 모입니다.

## 테스트 현황

- 총 62개 테스트 통과 (유닛 40 + 통합 21 + doc 1)
- `syllable`: 분해/합성, 라운드트립, 비한글 처리, 받침/양성모음 판별 (9개)
- `yongeon`: 생성, 어간 분석, 품사/활용 판별, Display, 다양한 불규칙 유형 (12개)
- `join`: AhEo 모음조화 (양성/음성/하다), Plain 받침 유무, Fixed (6개)
- `merge`: 모음 축약 6종, 과거 시제 종성 이전, 받침 통과 (13개)
- `yongeon_test`: 동사/형용사 검색, 동음이의어, 어간 검색, 존재하지 않는 단어 (7개)
- `postfix_test`: 해요체 4종, 과거 시제 3종, 다양한 어미 3종, 동음이의어 처리 4종 (14개)
- doc-test: `Yongeon::new` 예시 (1개)

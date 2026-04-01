# 작업 리뷰

## 프로젝트 구조

```
yongcat/
├── Cargo.toml
├── build.rs
├── .gitattributes             # data/*.csv LF 강제
├── about_data.md              # 데이터 설명 문서
├── README.md                  # 프로젝트 소개, 사용법, 활용 규칙 현황, 한계
├── QUICKSTART.md              # 시작 가이드
├── RULES.md                   # 활용 규칙 구현 가이드
├── REVIEW.md                  # 이 파일
├── data/
│   └── yong_list.csv          # 용언 1,721개
├── tests/
│   ├── eomi_ah_eo_test.rs     # 아/어 계열 어미 테스트 (12개)
│   ├── eomi_fixed_test.rs     # 고정 형태 어미 테스트 (10개)
│   ├── eomi_plain_test.rs     # 받침 유무 어미 테스트 (10개)
│   ├── postfix_test.rs        # 활용 파이프라인 통합 테스트 (34개)
│   └── yongeon_test.rs        # 용언 검색 통합 테스트 (7개)
└── src/
    ├── lib.rs                 # 크레이트 루트, find_yongeon, find_eogan, postfix, postfix_word
    ├── eomi/
    │   ├── mod.rs             # Eomi 열거형, AhEoForm 타입 (2-튜플)
    │   ├── ah_eo.rs           # 아/어 계열 어미 상수 7개
    │   ├── fixed.rs           # 고정 형태 어미 상수 5개
    │   └── plain.rs           # 받침 유무 어미 상수 5개
    ├── irregular/
    │   ├── mod.rs             # 불규칙 유형별 디스패치
    │   ├── yeo.rs             # 여불규칙
    │   ├── dieut.rs           # ㄷ불규칙
    │   ├── bieut.rs           # ㅂ불규칙
    │   └── siot.rs            # ㅅ불규칙
    ├── join.rs                # select → irregular → regular
    ├── merge.rs               # apply → irregular → regular
    ├── syllable.rs            # 한글 음절 분해/합성
    ├── types.rs               # YongeonType, IrregularType 열거형
    └── yongeon.rs             # Yongeon 구조체
```

## 파이프라인

```
select ──→ irregular::join ──→ Some이면 반환
           │
           └→ regular        ──→ 모음조화 + 접합

apply  ──→ irregular::merge ──→ Some이면 반환
           │
           └→ regular        ──→ 모음 축약/탈락
```

## 구현 현황

### 어미

| 유형 | 파일 | 상수 |
|------|------|------|
| AhEo (아/어 계열) | `ah_eo.rs` | A, AYO, ASEO, ADO, AYA, ARA, ASS (7개) |
| Fixed (고정 형태) | `fixed.rs` | GO, JI, NEUN, GE, JA (5개) |
| Plain (받침 유무) | `plain.rs` | EUN, EUL, EUMYEON, EUNI, SEUMNIDA (5개) |

`AhEoForm`은 2-튜플 `(양성, 음성)`입니다. 여불규칙의 여 형태는 음성 형태에서 ㅓ→ㅕ 변환으로 생성합니다.

### 규칙 활용

| 규칙 | 단계 | 구현 위치 |
|------|------|-----------|
| 모음조화 | join | `join.rs` regular(), `Yongeon::is_positive_harmony()` |
| ㅡ 어간 모음조화 | join | `Yongeon::is_positive_harmony()` |
| 모음 축약 6종 | merge | `merge.rs` contract_ah_eo() |
| ㅡ 탈락 | merge | `merge.rs` contract_ah_eo() |
| 종성 이전 | merge | `merge.rs` contract_ah_eo() |

### 불규칙 활용

| 유형 | 용언 수 | join | merge | 파일 |
|------|---------|------|-------|------|
| 여불규칙 | 610개 | form.1에서 ㅓ→ㅕ 생성 | ㅏ→ㅐ 축약 | `yeo.rs` |
| ㄷ불규칙 | 6개 | ㄷ→ㄹ 변환 | None | `dieut.rs` |
| ㅂ불규칙 | 66개 | ㅂ→우 변환 + 특수 모음조화 | 우+어미 축약 | `bieut.rs` |
| ㅅ불규칙 | 9개 | ㅅ 탈락 | 축약 억제 | `siot.rs` |

### 기타

- `build.rs`: CSV → `load_yongeons()` 코드 생성, 등급 필터링
- `syllable.rs`: 한글 유니코드 분해/합성, 외부 의존성 없음
- `Yongeon::new()`: 빈 어간 assert 포함

## 테스트 현황

총 142개 테스트 통과

| 위치 | 테스트 수 | 내용 |
|------|-----------|------|
| `syllable.rs` | 9개 | 분해/합성, 라운드트립, 비한글, 받침/양성모음 |
| `yongeon.rs` | 12개 | 생성, 어간 분석, 품사/활용 판별, Display |
| `join.rs` | 6개 | AhEo 모음조화, Plain 받침 유무, Fixed |
| `merge.rs` | 13개 | 모음 축약 6종, 과거 시제 종성 이전, 받침 통과 |
| `irregular/yeo.rs` | 7개 | join/merge × AhEo/Plain/Fixed |
| `irregular/dieut.rs` | 4개 | join × AhEo/Past/Plain/Fixed |
| `irregular/bieut.rs` | 9개 | join/merge × 양성/음성/다음절/과거/Plain |
| `irregular/siot.rs` | 8개 | join/merge × 축약 억제 |
| `tests/eomi_ah_eo_test.rs` | 12개 | 규칙 8 + 불규칙 4 |
| `tests/eomi_fixed_test.rs` | 10개 | 상수 5 × 받침 유무 |
| `tests/eomi_plain_test.rs` | 10개 | 상수 5 × 받침 유무 |
| `tests/postfix_test.rs` | 34개 | 규칙/불규칙/ㅡ탈락/동음이의어 통합 |
| `tests/yongeon_test.rs` | 7개 | find_yongeon, find_eogan |
| doctest | 1개 | Yongeon::new 예시 |

## 미구현 사항

### 불규칙 활용 (6종)

| 유형 | 용언 수 | 난이도 |
|------|---------|--------|
| ㅎ불규칙 | 13개 | 높음 — 특수 축약 |
| ㄹ불규칙 | 73개 | 중간 — Plain/Fixed 불규칙 처리 필요 |
| 르불규칙 | 36개 | 높음 — ㄹ 삽입 + 어간 변환 |
| 으불규칙 | 24개 | 중간 |
| 러불규칙 | 3개 | 낮음 |
| 우불규칙 | 1개 | 낮음 |

### Plain/Fixed 어미의 불규칙 처리

구현된 불규칙(ㄷ, ㅂ, ㅅ)도 AhEo 어미에서만 동작합니다. Plain/Fixed 어미에서의 불규칙 처리는 미구현입니다.

```
걷다 (ㄷ) + 으면 → 걸으면  (현재: 걷으면 ✗)
돕다 (ㅂ) + 으면 → 도우면  (현재: 돕으면 ✗)
살다 (ㄹ) + 는   → 사는    (현재: 살는 ✗)
```

### 기타

- 품사별 어미 제한 없음 (동사 전용 어미를 형용사에 적용해도 오류 없이 생성)
- 등급 필터 테스트 부재

# 시작 가이드

## build.rs

`build.rs`은 Cargo가 컴파일 전에 자동 실행하는 빌드 스크립트입니다.

**용언 데이터 생성:**
1. `data/yong_list.csv`를 읽는다
2. 각 행을 `Yongeon::new(...)` 호출로 변환한다
3. `$OUT_DIR/yong_data.rs`에 `load_yongeons()` 함수를 생성한다

**어미 레지스트리 생성:**
1. `src/eomi/ah_eo.rs`, `fixed.rs`, `plain.rs`를 텍스트로 읽는다
2. `pub const XXX: Eomi` 패턴을 찾아 상수 이름을 추출한다
3. `$OUT_DIR/eomi_data.rs`에 `load_eomis()` 함수를 생성한다

소스에서 `include!`로 포함하며, CSV나 어미 소스 파일이 변경되면 자동 재실행한다.

## 어미 검색

### load_eomis — 전체 어미 목록

```rust
let eomis = yongcat::load_eomis();
// 26개 어미의 (이름, &Eomi) 목록
```

### find_eomi — 문자열로 어미 검색

```rust
let eomis = yongcat::load_eomis();
let results = yongcat::find_eomi(&eomis, "어요");
// "어요"를 포함하는 어미를 반환 (AYO, ASS_EOYO 등)
for (name, eomi) in &results {
    println!("{}", name);
}
```

## 용언 검색

### find_yongeon — 기본형으로 검색

```rust
let yongeons = yongcat::load_yongeons();
let results = yongcat::find_yongeon(&yongeons, "먹다");
// "먹다"에 해당하는 Yongeon 구조체들을 반환 (동음이의어가 있으면 여러 개)
```

### find_eogan — 어간으로 검색

```rust
let yongeons = yongcat::load_yongeons();
let results = yongcat::find_eogan(&yongeons, "먹");
// "먹"이 어간인 Yongeon 구조체들을 반환
```

두 함수 모두 일치하는 용언이 없으면 빈 `Vec`을 반환한다.

### 반환값

두 함수 모두 `Yongeon` 구조체의 참조를 반환하며, 다음 필드에 접근할 수 있다.

| 필드 | 타입 | 설명 |
|------|------|------|
| `base_form` | `&str` | 사전 기본형 (예: `"먹다"`) |
| `dict_id` | `&str` | 표제어 번호 (동음이의어 구분, 없으면 빈 문자열) |
| `eogan` | `Vec<Syllable>` | 어간의 음절 분해 |
| `yongeon_type` | `YongeonType` | `Verb`(동사) 또는 `Adjective`(형용사) |
| `irregular_type` | `IrregularType` | 활용 유형 (규칙 포함 11종) |

CSV의 `usage`와 `grade`는 구조체에 포함되지 않는다.

## 용언 활용

### postfix — 단어 문자열로 활용형 생성

```rust
use yongcat::eomi::ah_eo;

let yongeons = yongcat::load_yongeons();
let results = yongcat::postfix(&yongeons, "가다", &ah_eo::AYO);
// [(&Yongeon, "가요")] — 동음이의어가 있으면 각각의 활용형을 반환
```

동음이의어는 `dict_id`로 구별할 수 있다.

```rust
for (yongeon, conjugated) in &results {
    println!("{} (dict_id: {}) → {}", yongeon.base_form, yongeon.dict_id, conjugated);
}
```

### postfix_word — 단일 용언으로 활용형 생성

```rust
use yongcat::eomi::ah_eo;

let yongeons = yongcat::load_yongeons();
let meok = &yongcat::find_yongeon(&yongeons, "먹다")[0];
let result = yongcat::postfix_word(meok, &ah_eo::AYO);
// "먹어요"
```

### ah_eo 어미 목록 (아/어 계열)

| 상수 | 어미 | 용도 | 예시 (가다/먹다) |
|------|------|------|------------------|
| `A` | 아/어 | 종결 (반말) | 가, 먹어 |
| `AYO` | 아요/어요 | 종결 (해요체) | 가요, 먹어요 |
| `ASEO` | 아서/어서 | 연결 (이유) | 가서, 먹어서 |
| `ADO` | 아도/어도 | 연결 (양보) | 가도, 먹어도 |
| `AYA` | 아야/어야 | 연결 (조건) | 가야, 먹어야 |
| `ARA` | 아라/어라 | 명령 | 가라, 먹어라 |
| `ASS` | 았/었 | 과거 시제 | 갔, 먹었 |
| `ASS_EOYO` | 았어요/었어요 | 과거 해요체 | 갔어요, 먹었어요 |
| `ASS_SEUMNIDA` | 았습니다/었습니다 | 과거 합쇼체 | 갔습니다, 먹었습니다 |

### fixed 어미 목록 (고정 형태)

| 상수 | 어미 | 용도 | 예시 (먹다/가다) |
|------|------|------|------------------|
| `GO` | 고 | 연결 (나열) | 먹고, 가고 |
| `JI` | 지 | 부정 | 먹지, 가지 |
| `NEUN` | 는 | 관형사형 (현재) | 먹는, 가는 |
| `GE` | 게 | 결과 | 먹게, 가게 |
| `JA` | 자 | 청유 | 먹자, 가자 |
| `GESS_EOYO` | 겠어요 | 추측 해요체 | 먹겠어요, 가겠어요 |
| `GESS_SEUMNIDA` | 겠습니다 | 추측 합쇼체 | 먹겠습니다, 가겠습니다 |

### plain 어미 목록 (받침 유무)

| 상수 | 어미 | 용도 | 예시 (먹다/가다) |
|------|------|------|------------------|
| `EUN` | 은/ㄴ | 관형사형 (과거) | 먹은, 간 |
| `EUL` | 을/ㄹ | 관형사형 (미래) | 먹을, 갈 |
| `EUMYEON` | 으면/면 | 조건 | 먹으면, 가면 |
| `EUNI` | 으니/니 | 이유 | 먹으니, 가니 |
| `SEUMNIDA` | 습니다/ㅂ니다 | 종결 (합쇼체) | 먹습니다, 갑니다 |
| `EUMYEONSEO` | 으면서/면서 | 동시 | 먹으면서, 가면서 |
| `EURYEOGO` | 으려고/려고 | 의도 | 먹으려고, 가려고 |
| `EUSEYO` | 으세요/세요 | 높임 명령 | 먹으세요, 가세요 |
| `EUREO` | 으러/러 | 목적 (이동) | 먹으러, 가러 |
| `EUSYEOSS` | 으셨/셨 | 높임 과거 | 먹으셨, 가셨 |

### 불규칙 활용 예시

```rust
use yongcat::eomi::ah_eo;

let yongeons = yongcat::load_yongeons();

// 여불규칙: 하다 → 해요
let ha = &yongcat::find_yongeon(&yongeons, "하다")[0];
assert_eq!(yongcat::postfix_word(ha, &ah_eo::AYO), "해요");

// ㄷ불규칙: 걷다 → 걸어요
let geot = &yongcat::find_yongeon(&yongeons, "걷다")[0];
assert_eq!(yongcat::postfix_word(geot, &ah_eo::AYO), "걸어요");

// ㅂ불규칙: 돕다 → 도와요
let dop = &yongcat::find_yongeon(&yongeons, "돕다")[0];
assert_eq!(yongcat::postfix_word(dop, &ah_eo::AYO), "도와요");

// ㅅ불규칙: 짓다 → 지어요 (축약 억제)
let jit = &yongcat::find_yongeon(&yongeons, "짓다")[0];
assert_eq!(yongcat::postfix_word(jit, &ah_eo::AYO), "지어요");

// ㄹ불규칙: 살다 + 는 → 사는 (ㄹ 탈락)
use yongcat::eomi::fixed;
let sal = &yongcat::find_yongeon(&yongeons, "살다")[0];
assert_eq!(yongcat::postfix_word(sal, &fixed::NEUN), "사는");

// 우불규칙: 푸다 → 퍼요
let pu = &yongcat::find_yongeon(&yongeons, "푸다")[0];
assert_eq!(yongcat::postfix_word(pu, &ah_eo::AYO), "퍼요");

// 러불규칙: 이르다 → 이르러요
let ireu = &yongcat::find_yongeon(&yongeons, "이르다")[0];
assert_eq!(yongcat::postfix_word(ireu, &ah_eo::AYO), "이르러요");

// ㅎ불규칙: 그렇다 → 그래요
let geureot = &yongcat::find_yongeon(&yongeons, "그렇다")[0];
assert_eq!(yongcat::postfix_word(geureot, &ah_eo::AYO), "그래요");

// 르불규칙: 모르다 → 몰라요
let moreu = &yongcat::find_yongeon(&yongeons, "모르다")[0];
assert_eq!(yongcat::postfix_word(moreu, &ah_eo::AYO), "몰라요");
```

## 등급별 컴파일

Cargo feature로 특정 학습 등급의 용언만 포함할 수 있다. feature를 지정하지 않으면 전체(1,721개)가 포함된다.

```bash
cargo build --features grade-a            # A등급만 (230개)
cargo build --features "grade-a,grade-b"  # A+B등급 (863개)
cargo build --features "grade-a,grade-b,grade-c"  # 전체 (1,721개)
```

`Cargo.toml` 의존성에서도 지정 가능하다.

```toml
[dependencies]
yongcat = { path = "../yongcat", features = ["grade-a"] }
```

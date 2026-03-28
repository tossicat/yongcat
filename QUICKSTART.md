# 시작 가이드

## build.rs

`build.rs`은 Cargo가 컴파일 전에 자동 실행하는 빌드 스크립트입니다.

1. `data/yong_list.csv`를 읽는다 (`cargo:rerun-if-changed`로 CSV 변경 시 자동 재실행)
2. 각 행을 `Yongeon::new(...)` 호출로 변환한다
3. `$OUT_DIR/yong_data.rs`에 `load_yongeons()` 함수를 생성한다

사용할 때는 소스에서 `include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));`로 포함한다.

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
use yongcat::eomi::{EomiGroup, AH_EO_GROUP};

let yongeons = yongcat::load_yongeons();
let results = yongcat::postfix(&yongeons, "가다", &EomiGroup::AhEo(AH_EO_GROUP[1]));
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
use yongcat::eomi::{EomiGroup, AH_EO_GROUP};

let yongeons = yongcat::load_yongeons();
let meok = &yongcat::find_yongeon(&yongeons, "먹다")[0];
let result = yongcat::postfix_word(meok, &EomiGroup::AhEo(AH_EO_GROUP[1]));
// "먹어요"
```

### AH_EO_GROUP 어미 목록

| 인덱스 | 어미 | 용도 | 예시 (가다/먹다) |
|--------|------|------|------------------|
| 0 | 아/어/여 | 종결 (해라체) | 가, 먹어 |
| 1 | 아요/어요/여요 | 종결 (해요체) | 가요, 먹어요 |
| 2 | 아서/어서/여서 | 연결 (이유) | 가서, 먹어서 |
| 3 | 아도/어도/여도 | 연결 (양보) | 가도, 먹어도 |
| 4 | 아야/어야/여야 | 연결 (조건) | 가야, 먹어야 |
| 5 | 아라/어라/여라 | 명령 | 가라, 먹어라 |
| 6 | 았/었/였 | 과거 시제 | 갔, 먹었 |

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

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

# 시작 가이드

## build.rs

Cargo가 컴파일 전에 자동 실행하는 빌드 스크립트이다.

1. `data/yong_list.csv`를 읽는다 (`cargo:rerun-if-changed`로 CSV 변경 시 자동 재실행)
2. 각 행을 `Yongeon::new(...)` 호출로 변환한다
3. `$OUT_DIR/yong_data.rs`에 `load_yongeons()` 함수를 생성한다

사용할 때는 소스에서 `include!(concat!(env!("OUT_DIR"), "/yong_data.rs"));`로 포함한다.

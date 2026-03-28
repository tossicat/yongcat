# yongcat

한국어 용언(동사/형용사)의 활용형을 생성하는 Rust 라이브러리입니다.

## 특징

- 표준국어대사전 기반 용언 1,721개 지원
- 어미 그룹별 활용형 자동 생성 (아/어 계열 모음조화 처리)
- 동음이의어 구분 (`dict_id`)
- 등급별 컴파일 (학습 등급 A/B/C 필터링)

## 사용법

```rust
use yongcat::eomi::{EomiGroup, AH_EO_GROUP};

let yongeons = yongcat::load_yongeons();

// 단어 문자열로 활용형 생성 (동음이의어 전체 처리)
let results = yongcat::postfix(&yongeons, "가다", &EomiGroup::AhEo(AH_EO_GROUP[1]));
for (yongeon, conjugated) in &results {
    println!("{} → {}", yongeon.base_form, conjugated);
    // 가다 → 가요
}

// 단일 용언으로 활용형 생성
let meok = &yongcat::find_yongeon(&yongeons, "먹다")[0];
let result = yongcat::postfix_word(meok, &EomiGroup::AhEo(AH_EO_GROUP[6]));
// "먹었"
```

## 어미 그룹

### AH_EO_GROUP (아/어 계열)

| 인덱스 | 어미 | 용도 | 예시 (가다/먹다) |
|--------|------|------|------------------|
| 0 | 아/어/여 | 종결 (해라체) | 가, 먹어 |
| 1 | 아요/어요/여요 | 종결 (해요체) | 가요, 먹어요 |
| 2 | 아서/어서/여서 | 연결 (이유) | 가서, 먹어서 |
| 3 | 아도/어도/여도 | 연결 (양보) | 가도, 먹어도 |
| 4 | 아야/어야/여야 | 연결 (조건) | 가야, 먹어야 |
| 5 | 아라/어라/여라 | 명령 | 가라, 먹어라 |
| 6 | 았/었/였 | 과거 시제 | 갔, 먹었 |

## 모듈 구조

| 모듈 | 역할 |
|------|------|
| `eomi` | 어미 데이터 및 `EomiGroup` 열거형 |
| `join` | 모음조화 판별 + 어간-어미 접합 (1·2단계) |
| `merge` | 음운 축약/탈락 처리 (3단계) |
| `syllable` | 한글 음절 분해/합성 |
| `yongeon` | `Yongeon` 구조체 |
| `types` | `YongeonType`, `IrregularType` 열거형 |

## 등급별 컴파일

Cargo feature로 특정 학습 등급의 용언만 포함할 수 있습니다.

```bash
cargo build --features grade-a            # A등급만 (230개)
cargo build --features "grade-a,grade-b"  # A+B등급 (863개)
cargo build                               # 전체 (1,721개)
```

## 라이선스

MIT

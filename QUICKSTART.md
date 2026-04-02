# 시작 가이드

## 빠른 시작

`use yongcat::*;`로 모든 어미 상수와 편의 함수를 가져올 수 있습니다.

```rust
use yongcat::*;

// 용언 검색
let verb = lookup("먹다");

// 활용형 생성
conjugate(verb, &AYO);           // "먹어요"
conjugate(verb, &ASS);           // "먹었"
conjugate(verb, &EUMYEON);       // "먹으면"
conjugate(verb, &GO);            // "먹고"
```

### 게임 전투 로그 예시

```rust
use yongcat::*;

let ha = lookup("하다");

// 동적으로 스킬 이름을 조합
let skill = "공격";
println!("철수가 몬스터를 {}{}.", skill, conjugate(ha, &ASS_SEUMNIDA));
// → "철수가 몬스터를 공격했습니다."

println!("철수가 몬스터를 {}{}!", skill, conjugate(ha, &A));
// → "철수가 몬스터를 공격해!"

// 통째로 활용할 수도 있음
let verb = lookup("공격하다");
println!("철수가 몬스터를 {}.", conjugate(verb, &ASS_SEUMNIDA));
// → "철수가 몬스터를 공격했습니다."

// 비-하다 용언은 통째로 활용
let verb = lookup("때리다");
println!("철수가 몬스터를 {}.", conjugate(verb, &ASS));
// → "철수가 몬스터를 때렸."

// 다양한 어미로 상황 묘사
let verb = lookup("쓰러지다");
println!("몬스터가 {}.", conjugate(verb, &ASS_SEUMNIDA));
// → "몬스터가 쓰러졌습니다."

let verb = lookup("얻다");
println!("경험치를 {}!", conjugate(verb, &ASS_EOYO));
// → "경험치를 얻었어요!"
```

## 어미 문자열 검색

`find_eomi_exact()`로 어미 형태 문자열을 넘기면 해당 어미를 찾아줍니다. `lookup()`, `conjugate()`와 조합하면 문자열만으로 활용형을 생성할 수 있습니다.

```rust
use yongcat::*;

// "세요" → EUSEYO(으세요/세요) 를 찾아서 활용
let verb = lookup("쉬다");
let eomi = find_eomi_exact("세요").unwrap();
conjugate(verb, eomi)  // → "쉬세요"

// "었" → ASS(았/었) 를 찾아서 활용
let verb = lookup("먹다");
let eomi = find_eomi_exact("었").unwrap();
conjugate(verb, eomi)  // → "먹었"
```

템플릿 시스템에 활용할 수 있습니다. 예를 들어 `"{쉬다,세요}"`라는 템플릿을 파싱하여 용언과 어미를 분리한 뒤, `lookup()` + `find_eomi_exact()` + `conjugate()`로 활용형을 생성하면 됩니다.

```rust
use yongcat::*;

// 템플릿 파서가 "{쉬다,세요}"를 분리했다고 가정
let word = "쉬다";
let ending = "세요";

let verb = lookup(word);
let eomi = find_eomi_exact(ending).unwrap();
let result = conjugate(verb, eomi);
// → "쉬세요"

// 게임 대화 시스템 예시
let npc_line = format!("여기서 {}.", result);
// → "여기서 쉬세요."
```

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
// 42개 어미의 (이름, &Eomi) 목록
```

### find_eomi — 문자열로 어미 검색

```rust
let eomis = yongcat::load_eomis();
let results = yongcat::find_eomi(&eomis, "어요");
// "어요"와 정확히 일치하는 어미를 반환 (AYO)
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
| `AJIDA` | 아지다/어지다 | 피동·상태변화 | 가지다, 먹어지다 |
| `ABODA` | 아보다/어보다 | 시행 | 가보다, 먹어보다 |
| `ADALLA` | 아달라/어달라 | 요청 | 가달라, 먹어달라 |

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
| `DA` | 다 | 종결 (평서) | 먹다, 가다 |
| `JIMAN` | 지만 | 연결 (대조) | 먹지만, 가지만 |
| `GEONA` | 거나 | 연결 (선택) | 먹거나, 가거나 |
| `NEUNDE` | 는데 | 연결 (배경·대조) | 먹는데, 가는데 |
| `DAGA` | 다가 | 연결 (전환) | 먹다가, 가다가 |
| `DOROK` | 도록 | 연결 (목적·정도) | 먹도록, 가도록 |
| `DAMYEON` | 다면 | 연결 (가정) | 먹다면, 가다면 |

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
| `EUNIKKA` | 으니까/니까 | 이유 (강조) | 먹으니까, 가니까 |
| `EULKKA` | 을까/ㄹ까 | 종결 (의문·제안) | 먹을까, 갈까 |
| `EULGE` | 을게/ㄹ게 | 종결 (약속·의지) | 먹을게, 갈게 |
| `EULLAE` | 을래/ㄹ래 | 종결 (의향, 반말) | 먹을래, 갈래 |
| `EULSSUROK` | 을수록/ㄹ수록 | 연결 (점진) | 먹을수록, 갈수록 |
| `EUPSIDA` | 읍시다/ㅂ시다 | 종결 (격식 청유) | 먹읍시다, 갑시다 |

### 불규칙 활용 예시

```rust
use yongcat::*;

conjugate(lookup("하다"), &AYO);       // 여불규칙: "해요"
conjugate(lookup("걷다"), &AYO);       // ㄷ불규칙: "걸어요"
conjugate(lookup("돕다"), &AYO);       // ㅂ불규칙: "도와요"
conjugate(lookup("짓다"), &AYO);       // ㅅ불규칙: "지어요"
conjugate(lookup("살다"), &NEUN);      // ㄹ불규칙: "사는"
conjugate(lookup("푸다"), &AYO);       // 우불규칙: "퍼요"
conjugate(lookup("이르다"), &AYO);     // 러불규칙: "이르러요"
conjugate(lookup("그렇다"), &AYO);     // ㅎ불규칙: "그래요"
conjugate(lookup("모르다"), &AYO);     // 르불규칙: "몰라요"
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

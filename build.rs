use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CSV_PATH: &str = "data/yong_list.csv";
const OUT_FILE: &str = "yong_data.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", CSV_PATH);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(OUT_FILE);

    let file = fs::File::open(CSV_PATH).expect(&format!("{}를 열 수 없습니다", CSV_PATH));
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim_end_matches('\r').to_string();

        // 헤더 건너뛰기
        if i == 0 {
            continue;
        }

        let fields: Vec<&str> = line.splitn(5, ',').collect();
        if fields.len() != 5 {
            panic!("{}번째 줄: 컬럼 수가 5가 아닙니다: {}", i + 1, line);
        }

        let base_form = fields[0];
        // fields[1] = homonym_id (코드 생성에서는 사용하지 않음)
        let eogan = fields[2];
        let pos = match fields[3] {
            "동사" => "YongeonType::Verb",
            "형용사" => "YongeonType::Adjective",
            other => panic!("{}번째 줄: 알 수 없는 품사: {}", i + 1, other),
        };
        let conjugation = match fields[4] {
            "규칙" => "IrregularType::Regular",
            "ㄷ불규칙" => "IrregularType::Dieut",
            "ㅂ불규칙" => "IrregularType::Bieut",
            "ㅎ불규칙" => "IrregularType::Hieut",
            "ㄹ불규칙" => "IrregularType::Rieul",
            "ㅅ불규칙" => "IrregularType::Siot",
            "르불규칙" => "IrregularType::Reu",
            "우불규칙" => "IrregularType::U",
            "여불규칙" => "IrregularType::Yeo",
            "러불규칙" => "IrregularType::Reo",
            "으불규칙" => "IrregularType::Eu",
            other => panic!("{}번째 줄: 알 수 없는 활용 유형: {}", i + 1, other),
        };

        entries.push(format!(
            "    Yongeon::new(\"{}\", \"{}\", {}, {}),",
            base_form, eogan, pos, conjugation
        ));
    }

    let code = format!(
        "pub fn load_yongeons() -> Vec<Yongeon<'static>> {{\n    vec![\n{}\n    ]\n}}\n",
        entries.join("\n")
    );

    fs::write(&dest_path, code).unwrap();
}

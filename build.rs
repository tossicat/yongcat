use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CSV_PATH: &str = "data/yong_list.csv";
const OUT_FILE: &str = "yong_data.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", CSV_PATH);

    let grade_a = env::var("CARGO_FEATURE_GRADE_A").is_ok();
    let grade_b = env::var("CARGO_FEATURE_GRADE_B").is_ok();
    let grade_c = env::var("CARGO_FEATURE_GRADE_C").is_ok();
    let filter_grade = grade_a || grade_b || grade_c;

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

        let fields: Vec<&str> = line.splitn(7, ',').collect();
        if fields.len() != 7 {
            panic!("{}번째 줄: 컬럼 수가 7이 아닙니다: {}", i + 1, line);
        }

        // grade 필터링: feature가 하나라도 지정되면 해당 등급만 포함
        let grade = fields[6];
        if filter_grade {
            match grade {
                "A" if !grade_a => continue,
                "B" if !grade_b => continue,
                "C" if !grade_c => continue,
                _ => {}
            }
        }

        let base_form = fields[0];
        let dict_id = fields[1];
        let eogan = fields[2];
        let pos = match fields[3] {
            "동사" => "YongeonType::Verb",
            "형용사" => "YongeonType::Adjective",
            other => panic!("{}번째 줄: 알 수 없는 품사: {}", i + 1, other),
        };
        let conjugation = match fields[4] {
            "규" => "IrregularType::Regular",
            "ㄷ" => "IrregularType::Dieut",
            "ㅂ" => "IrregularType::Bieut",
            "ㅎ" => "IrregularType::Hieut",
            "ㄹ" => "IrregularType::Rieul",
            "ㅅ" => "IrregularType::Siot",
            "르" => "IrregularType::Reu",
            "우" => "IrregularType::U",
            "여" => "IrregularType::Yeo",
            "러" => "IrregularType::Reo",
            "으" => "IrregularType::Eu",
            other => panic!("{}번째 줄: 알 수 없는 활용 유형: {}", i + 1, other),
        };

        entries.push(format!(
            "    Yongeon::new(\"{}\", \"{}\", \"{}\", {}, {}),",
            base_form, dict_id, eogan, pos, conjugation
        ));
    }

    let code = format!(
        "pub fn load_yongeons() -> Vec<Yongeon<'static>> {{\n    vec![\n{}\n    ]\n}}\n",
        entries.join("\n")
    );

    fs::write(&dest_path, code).unwrap();
}

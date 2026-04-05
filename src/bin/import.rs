//! ## import
//! `data/` 폴더의 사용자 CSV 파일을 검증하고, 통과한 행을 `data/user_list.csv`로 생성합니다.
//!
//! ```bash
//! cargo run --bin import
//! ```

#[path = "../../build/validate.rs"]
mod validate;

use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use validate::{RowResult, RowData};

const DATA_DIR: &str = "data";
const BASE_CSV: &str = "yong_list.csv";
const OUTPUT_CSV: &str = "user_list.csv";

/// yong_list.csv에서 base_form을 수집합니다.
fn load_existing() -> HashSet<String> {
    let path = format!("{}/{}", DATA_DIR, BASE_CSV);
    let mut set = HashSet::new();

    let file = match fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("경고: {}를 열 수 없습니다", path);
            return set;
        }
    };

    for (i, line) in BufReader::new(file).lines().enumerate() {
        if i == 0 { continue; } // 헤더
        let line = line.unwrap();
        let fields: Vec<&str> = line.splitn(7, ',').collect();
        if !fields.is_empty() {
            set.insert(fields[0].trim().to_string());
        }
    }

    set
}

/// data/ 폴더에서 검증 대상 CSV 파일 목록을 반환합니다.
fn find_user_csv_files() -> Vec<String> {
    let mut files = Vec::new();

    let entries = match fs::read_dir(DATA_DIR) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("오류: {} 폴더를 열 수 없습니다", DATA_DIR);
            return files;
        }
    };

    for entry in entries {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".csv") && name != BASE_CSV && name != OUTPUT_CSV {
            files.push(name);
        }
    }

    files.sort();
    files
}

/// CSV 파일 하나를 검증합니다.
fn validate_file(
    filename: &str,
    existing: &HashSet<String>,
    seen: &mut HashSet<String>,
) -> (Vec<RowData>, usize, usize, usize) {
    let path = format!("{}/{}", DATA_DIR, filename);
    let file = fs::File::open(&path).unwrap_or_else(|_| panic!("{}를 열 수 없습니다", path));
    let reader = BufReader::new(file);

    let mut passed = Vec::new();
    let mut errors = 0usize;
    let mut duplicates = 0usize;
    let mut empty = 0usize;

    println!("\n[검증] {} ", filename);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // 헤더 건너뛰기 (첫 행이 base_form으로 시작하면)
        if i == 0 && line.starts_with("base_form") {
            continue;
        }

        let line_num = i + 1;

        match validate::validate_row(&line, line_num) {
            RowResult::Ok(data) => {
                if existing.contains(&data.base_form) {
                    println!("  ! {}행: 중복 — \"{}\" (yong_list.csv에 존재)", line_num, data.base_form);
                    duplicates += 1;
                } else if seen.contains(&data.base_form) {
                    println!("  ! {}행: 중복 — \"{}\" (다른 사용자 파일에 존재)", line_num, data.base_form);
                    duplicates += 1;
                } else {
                    println!("  ✓ {}행: {} ({}, {})", line_num, data.base_form, data.pos, conjugation_name(&data.conjugation));
                    seen.insert(data.base_form.clone());
                    passed.push(data);
                }
            }
            RowResult::Error { line, message } => {
                println!("  ✗ {}행: 오류 — {}", line, message);
                errors += 1;
            }
            RowResult::Empty => {
                empty += 1;
            }
        }
    }

    let _ = empty; // 빈 행은 조용히 건너뜀
    (passed, errors, duplicates, empty)
}

/// 활용 유형 코드를 읽기 쉬운 이름으로 변환합니다.
fn conjugation_name(code: &str) -> &str {
    match code {
        "규" => "규칙",
        "ㄷ" => "ㄷ불규칙",
        "ㅂ" => "ㅂ불규칙",
        "ㅎ" => "ㅎ불규칙",
        "ㄹ" => "ㄹ불규칙",
        "ㅅ" => "ㅅ불규칙",
        "르" => "르불규칙",
        "우" => "우불규칙",
        "여" => "여불규칙",
        "러" => "러불규칙",
        "으" => "으불규칙",
        _ => code,
    }
}

/// 통과한 행을 data/user_list.csv로 저장합니다.
fn write_user_list(rows: &[RowData]) {
    let path = format!("{}/{}", DATA_DIR, OUTPUT_CSV);
    let mut content = String::from("base_form,dict_id,eogan,pos,conjugation,usage,grade\n");
    for row in rows {
        content.push_str(&row.to_csv_line());
        content.push('\n');
    }
    fs::write(&path, &content).unwrap_or_else(|_| panic!("{}를 쓸 수 없습니다", path));
}

fn main() {
    let files = find_user_csv_files();

    if files.is_empty() {
        println!("data/ 폴더에 사용자 CSV 파일이 없습니다.");
        println!("({}, {} 제외한 *.csv 파일을 추가하세요)", BASE_CSV, OUTPUT_CSV);
        return;
    }

    println!("=== yongcat 용언 검증 ===");

    let existing = load_existing();
    let mut seen = HashSet::new();
    let mut all_passed = Vec::new();
    let mut total_errors = 0usize;
    let mut total_duplicates = 0usize;
    let mut _total_empty = 0usize;
    let mut total_rows = 0usize;

    for filename in &files {
        let (passed, errors, duplicates, empty) = validate_file(filename, &existing, &mut seen);
        let file_rows = passed.len() + errors + duplicates + empty;
        total_rows += file_rows;
        total_errors += errors;
        total_duplicates += duplicates;
        _total_empty += empty;
        all_passed.extend(passed);
    }

    println!("\n---");
    println!(
        "결과: {}행 중 {}행 통과, {}행 오류, {}행 중복",
        total_rows, all_passed.len(), total_errors, total_duplicates,
    );

    if all_passed.is_empty() {
        println!("통과한 행이 없어 {} 파일을 생성하지 않습니다.", OUTPUT_CSV);
    } else {
        write_user_list(&all_passed);
        println!("→ data/{} 생성 완료 ({}행)", OUTPUT_CSV, all_passed.len());
    }
}

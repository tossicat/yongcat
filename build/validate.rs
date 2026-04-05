/// CSV 한 행의 검증 결과입니다.
pub enum RowResult {
    /// 검증 통과
    Ok(RowData),
    /// 검증 오류 (빌드에 포함하지 않음)
    Error { line: usize, message: String },
    /// 빈 행 (건너뜀)
    Empty,
}

/// 검증을 통과한 CSV 행 데이터입니다.
pub struct RowData {
    pub base_form: String,
    pub dict_id: String,
    pub eogan: String,
    pub pos: String,
    pub conjugation: String,
    pub usage: String,
    pub grade: String,
}

const VALID_POS: &[&str] = &["동사", "형용사"];
const VALID_CONJUGATIONS: &[&str] = &[
    "규", "ㄷ", "ㅂ", "ㅎ", "ㄹ", "ㅅ", "르", "우", "여", "러", "으",
];

/// CSV 한 행을 검증합니다.
///
/// 헤더 행은 호출자가 건너뛰어야 합니다.
/// `line_num`은 파일 내 행 번호(1-based)입니다.
pub fn validate_row(line_str: &str, line_num: usize) -> RowResult {
    let trimmed = line_str.trim_end_matches('\r').trim();
    if trimmed.is_empty() {
        return RowResult::Empty;
    }

    let fields: Vec<&str> = trimmed.splitn(7, ',').collect();

    if fields.len() < 5 {
        return RowResult::Error {
            line: line_num,
            message: format!("컬럼 최소 5개 필요, {}개 발견", fields.len()),
        };
    }

    let base_form = fields[0].trim();
    let dict_id = fields[1].trim();
    let eogan = fields[2].trim();
    let pos = fields[3].trim();
    let conjugation = fields[4].trim();
    let usage = if fields.len() > 5 { fields[5].trim() } else { "" };
    let grade = if fields.len() > 6 { fields[6].trim() } else { "" };

    if base_form.is_empty() {
        return RowResult::Error {
            line: line_num,
            message: "기본형이 비어 있습니다".to_string(),
        };
    }

    if !base_form.ends_with("다") {
        return RowResult::Error {
            line: line_num,
            message: format!("기본형이 \"다\"로 끝나지 않음: \"{}\"", base_form),
        };
    }

    if eogan.is_empty() {
        return RowResult::Error {
            line: line_num,
            message: "어간이 비어 있습니다".to_string(),
        };
    }

    let expected_eogan = &base_form[..base_form.len() - '다'.len_utf8()];
    if eogan != expected_eogan {
        return RowResult::Error {
            line: line_num,
            message: format!(
                "어간이 기본형과 불일치: \"{}\" ≠ \"{}\"",
                eogan, expected_eogan
            ),
        };
    }

    if !VALID_POS.contains(&pos) {
        return RowResult::Error {
            line: line_num,
            message: format!("유효하지 않은 품사: \"{}\" (동사/형용사)", pos),
        };
    }

    if !VALID_CONJUGATIONS.contains(&conjugation) {
        return RowResult::Error {
            line: line_num,
            message: format!("유효하지 않은 활용 유형: \"{}\"", conjugation),
        };
    }

    RowResult::Ok(RowData {
        base_form: base_form.to_string(),
        dict_id: dict_id.to_string(),
        eogan: eogan.to_string(),
        pos: pos.to_string(),
        conjugation: conjugation.to_string(),
        usage: usage.to_string(),
        grade: grade.to_string(),
    })
}

impl RowData {
    /// CSV 행 형식으로 변환합니다.
    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.base_form,
            self.dict_id,
            self.eogan,
            self.pos,
            self.conjugation,
            self.usage,
            self.grade,
        )
    }
}

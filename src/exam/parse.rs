use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use indexmap::map::IndexMap;
use serde::Deserialize;

use crate::error::{ParseError, ParseErrorKind, WithPath};
use crate::exam::{Exam, Student};

#[derive(Deserialize)]
struct ExamFile {
    details: Option<IndexMap<String, f32>>,

    // Using IndexMap instead of HashMap to preserve the students order of the
    // original file.
    students: IndexMap<String, f32>,
}

// Files should follow the required format, see docs for more info.
pub fn parse_exam_file(path: &Path) -> Result<Exam, ParseError> {
    let file_content = fs::read_to_string(path).with_path(path)?;
    let file_extension = path.extension().and_then(OsStr::to_str);

    let exam_file: ExamFile = match file_extension {
        Some("toml") => toml::from_str(&file_content).with_path(path)?,
        Some("json") => serde_json::from_str(&file_content).with_path(path)?,
        None => return Err(ParseError::new(ParseErrorKind::MissingFormat, path)),
        _ => return Err(ParseError::new(ParseErrorKind::UnsupportedFormat, path)),
    };

    let students: Vec<Student> = exam_file
        .students
        .into_iter()
        .map(|(name, grade)| Student::new(name, grade))
        .collect();

    let _max_grade = exam_file
        .details
        .and_then(|details| details.get("max_grade").copied())
        .unwrap_or(10.0);

    Ok(Exam::new(students))
}

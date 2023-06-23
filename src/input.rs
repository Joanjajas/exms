use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use indexmap::map::IndexMap;
use serde::Deserialize;

use crate::error::{ParseError, ParseErrorKind, WithPath};
use crate::exam::Student;

/// Struct used to deserialize TOML and JSON files into a `Vec<Student>`.
#[derive(Deserialize)]
struct ExamFile {
    // We use an `IndexMap` to preserve the students order of the original file
    students: IndexMap<String, f32>,
}

/// Parses a file into a `Vec<Student>` based on the file extension. For the
/// moment only JSON and TOML files are supported.
pub fn parse_file(path: &Path) -> Result<Vec<Student>, ParseError> {
    let file_content = fs::read_to_string(path).with_path(path)?;
    let file_extension = path.extension().and_then(OsStr::to_str);

    let exam_file: ExamFile = match file_extension {
        Some("toml") => toml::from_str(&file_content).with_path(path)?,
        Some("json") => serde_json::from_str(&file_content).with_path(path)?,
        None => return Err(ParseError::new(ParseErrorKind::MissingFormat, path)),
        _ => return Err(ParseError::new(ParseErrorKind::UnsupportedFormat, path)),
    };

    let students = exam_file
        .students
        .into_iter()
        .map(|(name, grade)| Student::new(name, grade))
        .collect();

    Ok(students)
}

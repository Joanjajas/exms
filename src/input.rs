use crate::error::{Error, ErrorKind};
use crate::exam::Student;
use indexmap::map::IndexMap;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct ExamFile {
    // We use an `IndexMap` to preserve the students order of the original file
    students: IndexMap<String, f32>,
}

/// Parses a file into a `Vec<Student>` based on the file extension. For the
/// moment only JSON and TOML files are supported.
pub fn parse_file(path: &Path) -> Result<Vec<Student>, Error> {
    // Read file contents
    let file_content =
        fs::read_to_string(path).map_err(|err| Error::new(ErrorKind::Io(err), path))?;

    // Get file extension as a &str
    let file_extension = path.extension().and_then(OsStr::to_str);

    // Deserialize file content into an `ExamFile` based on the file extension
    let exam_file: ExamFile = match file_extension {
        Some("toml") => toml::from_str(&file_content)
            .map_err(|err| Error::new(ErrorKind::ParseToml(err), path))?,

        Some("json") => serde_json::from_str(&file_content)
            .map_err(|err| Error::new(ErrorKind::ParseJson(err), path))?,

        None => return Err(Error::new(ErrorKind::MissingFormat, path)),

        _ => return Err(Error::new(ErrorKind::UnsupportedFormat, path)),
    };

    // Transform the `ExamFile` object into a 'Vec<Student>'
    let students = exam_file
        .students
        .into_iter()
        .map(|(name, grade)| Student::new(name, grade))
        .collect();

    Ok(students)
}

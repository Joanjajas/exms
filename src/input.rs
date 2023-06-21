use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use indexmap::map::IndexMap;
use serde::Deserialize;

use crate::error::{Error, ErrorKind};
use crate::exam::Student;

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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    fn students() -> Vec<Student> {
        return vec![
            Student::new("Joan Beltrán Peris", 4.65),
            Student::new("Jose Abad Martínez", 3.6),
            Student::new("David Jiménez Hidalgo", 7.94),
            Student::new("Jorge García Martínez", 5.03),
            Student::new("Adrián Gómez García", 1.96),
        ];
    }

    #[test]
    fn test_parse_file_toml() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.toml");
        let mut file = File::create(&file_path).unwrap();

        file.write_all(
            r#"
            [students]
            "Joan Beltrán Peris" = 4.65
            "Jose Abad Martínez" = 3.6
            "David Jiménez Hidalgo" = 7.94
            "Jorge García Martínez" = 5.03
            "Adrián Gómez García" = 1.96
            "#
            .as_bytes(),
        )
        .unwrap();

        let parsed_students = parse_file(&file_path).unwrap();
        let students = students();

        assert_eq!(parsed_students, students)
    }

    #[test]
    fn test_parse_file_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).unwrap();

        file.write_all(
            r#"
            {
                "students": {
                    "Joan Beltrán Peris": 4.65,
                    "Jose Abad Martínez": 3.6,
                    "David Jiménez Hidalgo": 7.94,
                    "Jorge García Martínez": 5.03,
                    "Adrián Gómez García": 1.96
                }
            }
            "#
            .as_bytes(),
        )
        .unwrap();

        let parsed_students = parse_file(&file_path).unwrap();
        let students = students();

        assert_eq!(parsed_students, students);
    }

    #[test]
    fn test_parse_file_missing_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test");

        let result = parse_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_file_unsupported_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        let result = parse_file(&file_path);
        assert!(result.is_err());
    }
}

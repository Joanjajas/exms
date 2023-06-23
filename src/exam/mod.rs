use std::cmp::Ordering;
use std::path::Path;

use colored::Colorize;
use prettytable::{format, row, Table};

use statistics::{calculate_percentiles, mean, passed_students};
pub use student::Student;

use crate::error::ParseError;
use crate::input::parse_file;

pub mod statistics;
mod student;

pub struct Exam {
    pub students: Vec<Student>,
}

impl Exam {
    /// Creates a new `Exam` from a vector of type `Student`.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let exam = Exam::from_student_vec(students);
    /// ```
    pub fn from_student_vec(mut students: Vec<Student>) -> Self {
        // Calculate the percentile of each student
        calculate_percentiles(&mut students);

        // Create a new exam from the vector of students
        Self { students }
    }

    /// Creates a new `Exam` from a given file.
    /// The file formats suppported for the moment are only JSON and TOML files.
    /// The file should have a student object with all the students and their
    /// grades as key/value pairs. For more information about wich format
    /// a file should follow, please see [exms](crate).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::error::Error;
    /// use std::path::Path;
    ///
    /// use exms::exam::Exam;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file_path = Path::new("students.json");
    ///     let exam = Exam::from_file(&file_path)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        // Create a new vector of students from the file
        let mut students = parse_file(path.as_ref())?;

        // Calculate the percentiles of the students
        calculate_percentiles(&mut students);

        // Create a new exam from the vector of students
        Ok(Exam { students })
    }

    /// Sorts the exam students based on their grade in descending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::from_student_vec(students);
    /// exam.sort_by_grade();
    ///
    /// assert_eq!(exam.students[0].grade, 7.94);
    /// assert_eq!(exam.students[1].grade, 4.6);
    /// assert_eq!(exam.students[2].grade, 3.6);
    /// ```
    pub fn sort_by_grade(&mut self) {
        // Sort students by name so that students with the same grade are sorted
        // alphabetically
        Exam::sort_by_alphabetic_order(self);

        // Sort students by grade
        self.students
            .sort_by(|a, b| b.grade.partial_cmp(&a.grade).unwrap_or(Ordering::Equal))
    }

    /// Sorts the exam students based on their name alphabetically.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::from_student_vec(students);
    /// exam.sort_by_alphabetic_order();
    ///
    /// assert_eq!(exam.students[0].name, "David Jiménez Hidalgo");
    /// assert_eq!(exam.students[1].name, "Joan Beltrán Peris");
    /// assert_eq!(exam.students[2].name, "Jose Abad Martínez");
    /// ```
    // TODO: make it work with accents
    pub fn sort_by_alphabetic_order(&mut self) {
        self.students.sort_by_key(|s| s.name.to_lowercase())
    }

    /// Filters the exam students yielding only the students which name contains
    /// the given query
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::from_student_vec(students);
    /// exam.filter_by_name(&["joan", "jorge", "jim"]);
    ///
    /// assert_eq!(exam.students.len(), 2);
    /// assert_eq!(exam.students[0].name, "Joan Beltrán Peris");
    /// assert_eq!(exam.students[1].name, "David Jiménez Hidalgo");

    /// ```
    pub fn filter_by_name<S: AsRef<str>>(&mut self, query: &[S]) {
        self.students.retain(|student| {
            query.iter().any(|name| {
                let name = name.as_ref().to_lowercase();
                student.name.to_lowercase().contains(&name)
            })
        });
    }

    /// Filters the exam students yielding only the students that are in the
    /// given file. The file format should be the same as the one used in
    /// [from_file](Exam::from_file).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::error::Error;
    /// use std::path::Path;
    ///
    /// use exms::exam::Exam;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file_path = Path::new("students.json");
    ///     let mut exam = Exam::from_file(&file_path)?;
    ///
    ///     let file_path = Path::new("students2.json");
    ///     exam.filter_by_file(&file_path)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn filter_by_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ParseError> {
        let students = parse_file(path.as_ref())?;
        self.students.retain(|student| {
            students
                .iter()
                .any(|s| s.name.to_lowercase() == student.name.to_lowercase())
        });

        Ok(())
    }

    /// Print the exam students in a well formatted table.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::from_student_vec(students);
    /// exam.print_students();
    /// ```
    pub fn print_students(&mut self) {
        let mut table = Table::new();
        table.set_titles(row!["Name", "Grade", "Percentile"]);

        for student in &self.students {
            let grade = student.grade;
            let grade_str = if grade >= 5.0 {
                grade.to_string().green()
            } else {
                grade.to_string().red()
            };
            table.add_row(row![student.name, grade_str, student.percentile]);
        }

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.printstd()
    }

    /// Print statistical information about the exam in a well formatted table,
    /// like the mean, total students, the exam pass percentage etc...
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = vec![
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::from_student_vec(students);
    /// exam.print_statistics("Exam statistics");
    /// ```
    pub fn print_statistics(&self, title: &str) {
        let mean = mean(&self.students);
        let total_students = self.students.len() as u32;
        let passed_students = passed_students(&self.students);
        let failed_students = total_students - passed_students;
        let pass_percentage = if total_students > 0 {
            (passed_students as f32 / total_students as f32) * 100.0
        } else {
            0.
        };

        let mut title_table = Table::new();
        title_table.add_row(row![Fc->title]);

        let mut table = Table::new();
        table.add_row(row!["Total Students", total_students]);
        table.add_row(row!["Passed Students", passed_students]);
        table.add_row(row!["Failed Students", failed_students]);
        table.add_row(row!["Pass Percentage", format!("{}%", pass_percentage)]);
        table.add_row(row!["Mean", mean]);

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        title_table.set_format(*format::consts::FORMAT_BOX_CHARS);

        title_table.printstd();
        table.printstd();
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    fn students() -> Vec<Student> {
        return vec![
            Student::with_percentile("Joan Beltrán Peris", 4.65, 50.0),
            Student::with_percentile("Jose Abad Martínez", 3.6, 25.0),
            Student::with_percentile("David Jiménez Hidalgo", 7.94, 100.0),
            Student::with_percentile("Jorge García Martínez", 5.03, 75.0),
            Student::with_percentile("Adrián Gómez García", 1.96, 0.0),
        ];
    }

    #[test]
    fn test_from_student_vec() {
        let students = students();
        let exam = Exam::from_student_vec(students.clone());

        assert_eq!(exam.students, students);
    }

    #[test]
    fn test_from_file() {
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

        let exam = Exam::from_file(&file_path).unwrap();
        let students = students();

        assert_eq!(exam.students, students);
    }

    #[test]
    fn test_sort_by_grade() {
        let students = students();
        let mut exam = Exam::from_student_vec(students);

        exam.sort_by_grade();

        assert_eq!(exam.students[0].grade, 7.94);
        assert_eq!(exam.students[1].grade, 5.03);
        assert_eq!(exam.students[2].grade, 4.65);
        assert_eq!(exam.students[3].grade, 3.6);
        assert_eq!(exam.students[4].grade, 1.96);
    }

    #[test]
    fn test_sort_by_alphabetic_order() {
        let students = students();
        let mut exam = Exam::from_student_vec(students);

        exam.sort_by_alphabetic_order();

        assert_eq!(exam.students[0].name, "Adrián Gómez García");
        assert_eq!(exam.students[1].name, "David Jiménez Hidalgo");
        assert_eq!(exam.students[2].name, "Joan Beltrán Peris");
        assert_eq!(exam.students[3].name, "Jorge García Martínez");
        assert_eq!(exam.students[4].name, "Jose Abad Martínez");
    }

    #[test]
    fn test_filter_by_name() {
        let students = students();
        let mut exam = Exam::from_student_vec(students);

        exam.filter_by_name(&["joan", "abad", "jim"]);

        assert_eq!(exam.students.len(), 3);
        assert_eq!(exam.students[0].name, "Joan Beltrán Peris");
        assert_eq!(exam.students[1].name, "Jose Abad Martínez");
        assert_eq!(exam.students[2].name, "David Jiménez Hidalgo");
    }

    #[test]
    fn test_filter_by_file() {
        let students = students();
        let mut exam = Exam::from_student_vec(students.clone());

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("filter.toml");
        let mut filter_file = File::create(&file_path).unwrap();

        filter_file
            .write_all(
                r#"
                [students]
                "Joan Beltrán Peris" = 4.65
                "Adrián Gómez García" = 1.96
                "#
                .as_bytes(),
            ).unwrap();

        exam.filter_by_file(&file_path).unwrap();

        assert_eq!(exam.students.len(), 2);
        assert_eq!(
            exam.students[0],
            Student::with_percentile("Joan Beltrán Peris", 4.65, 50.0)
        );
        assert_eq!(
            exam.students[1],
            Student::with_percentile("Adrián Gómez García", 1.96, 0.0)
        );
    }
}

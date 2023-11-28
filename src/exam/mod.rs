mod parse;
mod plot;
mod statistics;
mod student;

use std::cmp::Ordering;
use std::path::Path;

use colored::Colorize;
use prettytable::{format, row, Table};
use unidecode::unidecode;

use crate::error::ParseError;
use parse::parse_exam_file;
use statistics::ExamStatistics;
use student::AttachStatistics;
pub use student::Student;

/// This type represents and exam.
pub struct Exam {
    /// The students of the exam
    pub students: Vec<Student>,

    name: Option<String>,
    max_grade: f32,
}

impl Exam {
    /// Creates a new `Exam` from a given set of students.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let exam = Exam::new(students);
    /// ```
    pub fn new<T: Into<Vec<Student>>>(students: T) -> Self {
        let mut students = students.into();
        students.attach_statistics();

        Self {
            students,
            max_grade: 10.0,
            name: None,
        }
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
    /// use std::path::Path;
    ///
    /// use exms::error::ParseError;
    /// use exms::exam::Exam;
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let file_path = Path::new("students.json");
    ///     let exam = Exam::from_file(&file_path)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let mut exam = parse_exam_file(path.as_ref())?;
        exam.students.attach_statistics();

        Ok(exam)
    }

    /// Sets the maximum achievable grade in the exam.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.set_max_grade(6.0);
    /// ```
    pub fn set_max_grade(&mut self, max_grade: f32) {
        self.max_grade = max_grade
    }

    /// Sets the name of the exam.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.set_name("Econometrics");
    /// ```
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into())
    }

    /// Sorts the exam students based on their grade in descending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
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
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.sort_by_alphabetic_order();
    ///
    /// assert_eq!(exam.students[0].name, "David Jiménez Hidalgo");
    /// assert_eq!(exam.students[1].name, "Joan Beltrán Peris");
    /// assert_eq!(exam.students[2].name, "Jose Abad Martínez");
    /// ```
    pub fn sort_by_alphabetic_order(&mut self) {
        self.students
            .sort_by_key(|s| unidecode(&s.name.to_lowercase()))
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
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.filter_by_name(&["joan", "jorge", "jim"]);
    ///
    /// assert_eq!(exam.students.len(), 2);
    /// assert_eq!(exam.students[0].name, "Joan Beltrán Peris");
    /// assert_eq!(exam.students[1].name, "David Jiménez Hidalgo");
    /// ```
    pub fn filter_by_name<S: AsRef<str>>(&mut self, query: &[S]) {
        self.students.retain(|student| {
            query.iter().any(|name| {
                student
                    .name
                    .to_lowercase()
                    .contains(&name.as_ref().to_lowercase())
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
    /// use std::path::Path;
    ///
    /// use exms::error::ParseError;
    /// use exms::exam::Exam;
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let file_path = Path::new("students.json");
    ///     let mut exam = Exam::from_file(&file_path)?;
    ///
    ///     let file_path = Path::new("students2.json");
    ///     exam.filter_by_file(&[file_path])?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn filter_by_file<P: AsRef<Path>>(&mut self, file_paths: &[P]) -> Result<(), ParseError> {
        for path in file_paths {
            let exam = parse_exam_file(path.as_ref())?;
            let students = exam.students;

            self.students.retain(|student| {
                students
                    .iter()
                    .any(|s| s.name.to_lowercase() == student.name.to_lowercase())
            });
        }

        Ok(())
    }

    /// Print the exam students in a well formatted table with some statistical
    /// information about each student, like the percentile, the rank, etc...
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.students();
    /// ```
    pub fn students(&self) {
        let mut table = Table::new();
        table.set_titles(row![c->"Name", c->"Grade", c->"Percentile", c->"Rank"]);

        for student in &self.students {
            let colored_grade = if student.grade >= self.max_grade / 2.0 {
                student.grade.to_string().green()
            } else {
                student.grade.to_string().red()
            };

            let highest_rank = self
                .students
                .iter()
                .map(|s| s.rank.unwrap_or(0))
                .max()
                .unwrap_or(0);

            table.add_row(row![
                student.name,
                c->colored_grade,
                c->student.percentile.unwrap_or(0.),
                c->format!("[{}/{}]", student.rank.unwrap_or(0), highest_rank)
            ]);
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
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.summary();
    /// ```
    pub fn summary(&self) {
        let statistics = ExamStatistics::new(&self.students, self.max_grade);

        if let Some(exam_name) = &self.name {
            let mut table_title = Table::new();
            table_title.add_row(row![Fc->exam_name]);

            table_title.set_format(*format::consts::FORMAT_BOX_CHARS);
            table_title.printstd();
        }

        let mut table = Table::new();
        table.add_row(row!["Total Students", statistics.total_students]);
        table.add_row(row!["Passed Students", statistics.passed_students]);
        table.add_row(row!["Failed Students", statistics.failed_students]);
        table.add_row(row![
            "Pass Percentage",
            format!("{}%", statistics.pass_percentage)
        ]);
        table.add_row(row!["Mean", statistics.mean]);
        table.add_row(row!["Median", statistics.median]);
        table.add_row(row!["Standard Deviation", statistics.std_deviation]);
        table.add_row(row!["Max Grade", statistics.max_grade]);
        table.add_row(row!["Min Grade", statistics.min_grade]);

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.printstd();
    }

    /// Print a histogram of the exam grades.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Exam;
    /// use exms::exam::Student;
    ///
    /// let students = &[
    ///     Student::new("Joan Beltrán Peris", 4.6),
    ///     Student::new("Jose Abad Martínez", 3.6),
    ///     Student::new("David Jiménez Hidalgo", 7.94),
    /// ];
    ///
    /// let mut exam = Exam::new(students);
    /// exam.histogram();
    /// ```
    pub fn histogram(&self) {
        plot::histogram(&self.students, self.max_grade)
    }
}

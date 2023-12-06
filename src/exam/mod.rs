mod parse;
mod plot;
mod statistics;
mod student;

use std::cmp::Ordering;
use std::path::Path;

use unidecode::unidecode;

use crate::error::ParseError;
use parse::parse_exam_file;
use statistics::ExamStatistics;
pub use student::Student;

/// This type represents and exam.
#[derive(Debug, Clone)]
pub struct Exam {
    title: Option<String>,
    max_grade: f32,
    students: Vec<Student>,
    statistics: ExamStatistics,
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
    pub fn new(students: impl Into<Vec<Student>>) -> Self {
        let mut students = students.into();
        let statistics = ExamStatistics::new(&mut students, 10.0);

        Self {
            title: None,
            max_grade: 10.0,
            students,
            statistics,
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
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        parse_exam_file(path.as_ref())
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
        self.max_grade = max_grade;
        self.statistics = ExamStatistics::new(&mut self.students, max_grade);
    }

    /// Sets the title of the exam.
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
    /// exam.set_title("Econometrics");
    /// ```
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into())
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
        Self::sort_by_alphabetic_order(self);

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
        self.statistics.students(&self.students)
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
        self.statistics.summary(&self.title)
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
    pub fn histogram(&self, step: Option<f64>) {
        plot::histogram(&self.students, self.max_grade, step)
    }
}

pub mod statistics;
mod student;

pub use student::Student;

use crate::error::Error;
use crate::input::parse_file;
use colored::Colorize;
use prettytable::{format, row, Table};
use statistics::{calculate_percentiles, mean, passed_students};
use std::cmp::Ordering;
use std::path::Path;

pub struct Exam {
    students: Vec<Student>,
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
        // Calculate the percentiles of the students
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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
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
    /// ```
    pub fn sort_by_grade(&mut self) {
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
    /// ```
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
    /// exam.filter_by_name(&["joan", "jorge"]);
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
    pub fn filter_by_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
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
            (passed_students as f32 / total_students as f32) * 100.
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

use crate::exam::statistics;

/// Struct representing a student.
#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub name: String,
    pub grade: f32,
    pub rank: Option<u32>,
    pub percentile: Option<f32>,
}

impl Student {
    /// Creates a new student from a given name and grade.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::student::Student;
    ///
    /// let student = Student::new("Joan Beltrán Peris", 9.5);
    ///
    /// assert_eq!(student.name, "Joan Beltrán Peris");
    /// assert_eq!(student.grade, 9.5);
    /// ```
    pub fn new<T: Into<String>>(name: T, grade: f32) -> Student {
        Student {
            name: name.into(),
            grade,
            rank: None,
            percentile: None,
        }
    }
}

pub trait AttachStatistics: Into<Vec<Student>> {
    /// Calculates the rank and percentile of each student.
    fn attach_statistics(&mut self);
}

impl AttachStatistics for Vec<Student> {
    fn attach_statistics(&mut self) {
        statistics::rank(self);
        statistics::percentile(self);
    }
}

/// Struct representing a student.
#[derive(Debug, Clone)]
pub struct Student {
    /// Name of the student.
    pub name: String,

    /// Grade of the student.
    pub grade: f32,

    pub(crate) rank: Option<u32>,
    pub(crate) percentile: Option<f32>,
}

impl Student {
    /// Creates a new student from a given name and grade.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Student;
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

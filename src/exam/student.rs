#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub name: String,
    pub grade: f32,
    pub percentile: f32,
}

impl Student {
    /// Create a new `Student` with the given name and grade.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Student;
    ///
    /// let student = Student::new("Joan Beltrán Peris", 4.6);
    ///
    /// assert_eq!(student.name, "Joan Beltrán Peris");
    /// assert_eq!(student.grade, 4.6);
    /// ```
    pub fn new<N: Into<String>>(name: N, grade: f32) -> Self {
        Student {
            name: name.into(),
            grade,
            percentile: 0.,
        }
    }
}

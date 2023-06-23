/// Struct that represents an exam student.
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
    /// assert_eq!(student.percentile, 0.);
    /// ```
    pub fn new<N: Into<String>>(name: N, grade: f32) -> Self {
        Student {
            name: name.into(),
            grade,
            percentile: 0.0,
        }
    }

    /// Create a new `Student` with the given name, grade and percentile.
    ///
    /// # Examples
    ///
    /// ```
    /// use exms::exam::Student;
    ///
    /// let student = Student::with_percentile("Joan Beltrán Peris", 4.6, 50.);
    ///
    /// assert_eq!(student.name, "Joan Beltrán Peris");
    /// assert_eq!(student.grade, 4.6);
    /// assert_eq!(student.percentile, 50.);
    /// ```
    pub fn with_percentile<N: Into<String>>(name: N, grade: f32, percentile: f32) -> Self {
        Student {
            name: name.into(),
            grade,
            percentile,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_new() {
        let student = Student::new("Joan", 9.86);

        assert_eq!(student.name, "Joan");
        assert_eq!(student.grade, 9.86);
        assert_eq!(student.percentile, 0.0);
    }

    #[test]
    fn test_student_with_percentile() {
        let student = Student::with_percentile("Joan", 9.86, 50.0);

        assert_eq!(student.name, "Joan");
        assert_eq!(student.grade, 9.86);
        assert_eq!(student.percentile, 50.0);
    }
}

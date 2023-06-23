use std::cmp::Ordering;

use crate::exam::Student;

/// Calculate the mean grade of a given set of students.
pub fn mean(students: &[Student]) -> f32 {
    let (sum, count) = students
        .iter()
        .fold((0.0, 0.0), |(sum, count), s| (sum + s.grade, count + 1.0));

    if count == 0.0 {
        0.0
    } else {
        sum / count
    }
}

/// Calculates the number of students that passed an exam.
pub fn passed_students(students: &[Student]) -> u32 {
    students.iter().filter(|s| s.grade >= 5.0).count() as u32
}

/// Calculates the percentile of each student grade in a given set of students.
pub fn calculate_percentiles(students: &mut [Student]) {
    let total_students = students.len();

    // Create a vector of indices that represent the original order of the students
    let mut indices: Vec<usize> = (0..total_students).collect();
    let max_grade = students
        .iter()
        .map(|s| s.grade)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(0.0);

    // Sort the indices based on the grades of the students
    indices.sort_by(|&a, &b| {
        students[a]
            .grade
            .partial_cmp(&students[b].grade)
            .unwrap_or(Ordering::Equal)
    });

    // Calculate percentiles for the sorted vector of indices
    let mut last_grade = None;
    let mut last_percentile = None;

    for (index, &student_index) in indices.iter().enumerate() {
        let grade = students[student_index].grade;
        let percentile = match last_grade {
            // If the current grade is the same as the last grade, use the last percentile
            Some(last_grade) if grade == last_grade => last_percentile.unwrap_or(0.0),
            // Otherwise, calculate the percentile based on the current index and the total number
            // of students
            _ => {
                if grade == max_grade {
                    100.0
                } else {
                    index as f32 / (total_students - 1) as f32 * 100.0
                }
            }
        };
        // Set the percentile field for the current student
        students[student_index].percentile = percentile;
        // Update the last grade and percentile seen
        last_grade = Some(grade);
        last_percentile = Some(percentile);
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    fn students() -> Vec<Student> {
        vec![
            Student::new("Joan Beltrán Peris", 4.65),
            Student::new("Jose Abad Martínez", 3.6),
            Student::new("David Jiménez Hidalgo", 7.94),
            Student::new("Jorge García Martínez", 5.03),
            Student::new("Adrián Gómez García", 1.96),
        ]
    }

    #[test]
    fn test_mean() {
        let students = students();
        let mean = mean(&students);

        assert_approx_eq!(mean, 4.636)
    }

    #[test]
    fn test_passed_students() {
        let students = students();
        let passed_students = passed_students(&students);

        assert_eq!(passed_students, 2)
    }

    #[test]
    fn test_percentile() {
        let mut students = students();
        calculate_percentiles(&mut students);

        assert_eq!(students[0].percentile, 50.0);
        assert_eq!(students[1].percentile, 25.0);
        assert_eq!(students[2].percentile, 100.0);
        assert_eq!(students[3].percentile, 75.0);
        assert_eq!(students[4].percentile, 0.0);
    }
}

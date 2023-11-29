use std::cmp::Ordering;

use crate::exam::Student;

pub struct ExamStatistics {
    pub total_students: u32,
    pub passed_students: u32,
    pub failed_students: u32,
    pub pass_percentage: f32,
    pub mean: f32,
    pub median: f32,
    pub std_deviation: f32,
    pub max_grade: f32,
    pub min_grade: f32,
}

impl ExamStatistics {
    pub fn new(students: &[Student], max_grade: f32) -> Self {
        let total_students = students.len() as u32;
        let passed_students = passed_students(students, max_grade);
        let failed_students = total_students - passed_students;
        let pass_percentage = passed_students as f32 / total_students as f32 * 100.0;
        let mean = mean(students);
        let median = median(students);
        let std_deviation = std_deviation(students, mean);
        let max_grade = max_student_grade(students);
        let min_grade = min_student_grade(students);

        Self {
            total_students,
            passed_students,
            failed_students,
            pass_percentage,
            mean,
            median,
            std_deviation,
            max_grade,
            min_grade,
        }
    }
}

fn mean(students: &[Student]) -> f32 {
    let total_students = students.len();
    let grades_sum: f32 = students.iter().map(|s| s.grade).sum();

    if total_students == 0 {
        return 0.0;
    }

    grades_sum / total_students as f32
}

fn median(students: &[Student]) -> f32 {
    let total_students = students.len();

    if total_students == 0 {
        return 0.0;
    }

    let mut grades: Vec<f32> = students.iter().map(|s| s.grade).collect();
    grades.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    if total_students % 2 == 0 {
        let middle = total_students / 2;
        (grades[middle - 1] + grades[middle]) / 2.0
    } else {
        let middle = total_students / 2;
        grades[middle]
    }
}

fn passed_students(students: &[Student], max_grade: f32) -> u32 {
    return students
        .iter()
        .filter(|s| s.grade >= max_grade / 2.0)
        .count() as u32;
}

fn std_deviation(students: &[Student], mean: f32) -> f32 {
    let total_students = students.len();
    let mut sum = 0.0;

    for student in students {
        sum += (student.grade - mean).powi(2);
    }

    if total_students == 0 {
        return 0.0;
    }

    (sum / total_students as f32).sqrt()
}

fn max_student_grade(students: &[Student]) -> f32 {
    return students
        .iter()
        .map(|s| s.grade)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(0.0);
}

fn min_student_grade(students: &[Student]) -> f32 {
    students
        .iter()
        .map(|s| s.grade)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(0.0)
}

pub fn rank(students: &mut [Student]) {
    // Create a vector of indices that represent the original order of the students
    let total_students = students.len();
    let mut indices: Vec<usize> = (0..total_students).collect();

    // Sort the indices based on the grades of the students
    indices.sort_by(|&a, &b| {
        students[b]
            .grade
            .partial_cmp(&students[a].grade)
            .unwrap_or(Ordering::Equal)
    });

    let mut last_grade = None;
    let mut last_rank = None;
    let mut rank = 0;

    for &student_index in &indices {
        let grade = students[student_index].grade;
        let rank = match last_grade {
            // If the current grade is the same as the last grade, use the last rank
            Some(last_grade) if grade == last_grade => last_rank.unwrap_or(0),
            // Otherwise, calculate the rank based on the current index and the total number
            // of students
            _ => {
                rank += 1;
                rank
            }
        };
        // Set the rank field for the current student
        students[student_index].rank = Some(rank as u32);
        // Update the last grade and rank seen
        last_grade = Some(grade);
        last_rank = Some(rank);
    }
}

/// Calculates the percentile of each student given a set of students.
pub fn percentile(students: &mut [Student]) {
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
        students[student_index].percentile = Some(percentile);
        // Update the last grade and percentile seen
        last_grade = Some(grade);
        last_percentile = Some(percentile);
    }
}

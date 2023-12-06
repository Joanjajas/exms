use std::cmp::Ordering;

use colored::Colorize;
use prettytable::{format, row, Table};

use crate::exam::Student;

#[derive(Debug, Clone)]
pub struct ExamStatistics {
    pub total_students: u32,
    pub passed_students: u32,
    pub failed_students: u32,
    pub pass_rate: f32,
    pub mean: f32,
    pub median: f32,
    pub std_dev: f32,
    pub max_grade: f32,
    pub highest_grade: f32,
    pub lowest_grade: f32,
    pub highest_rank: u32,
}

impl ExamStatistics {
    pub fn new(students: &mut [Student], max_grade: f32) -> Self {
        attach_rank(students);
        attach_percentile(students);

        let total_students = students.len() as u32;
        let passed_students = passed_students(students, max_grade);
        let failed_students = total_students - passed_students;
        let pass_rate = passed_students as f32 / total_students as f32 * 100.0;
        let mean = mean(students);
        let median = median(students);
        let std_dev = std_deviation(students, mean);
        let highest_grade = max_student_grade(students);
        let lowest_grade = min_student_grade(students);
        let highest_rank = highest_rank(students);

        Self {
            total_students,
            passed_students,
            failed_students,
            pass_rate,
            mean,
            median,
            std_dev,
            max_grade,
            highest_grade,
            lowest_grade,
            highest_rank,
        }
    }

    pub fn summary(&self, title: &Option<String>) {
        if let Some(exam_title) = title {
            let mut table_title = Table::new();
            table_title.add_row(row![Fc->exam_title]);

            table_title.set_format(*format::consts::FORMAT_BOX_CHARS);
            table_title.printstd();
        }

        let mut table = Table::new();
        table.add_row(row!["Total Students", self.total_students]);
        table.add_row(row!["Passed Students", self.passed_students]);
        table.add_row(row!["Failed Students", self.failed_students]);
        table.add_row(row!["Pass Rate", format!("{}%", self.pass_rate)]);
        table.add_row(row!["Mean", self.mean]);
        table.add_row(row!["Median", self.median]);
        table.add_row(row!["Standard Deviation", self.std_dev]);
        table.add_row(row!["Max Grade", self.highest_grade]);
        table.add_row(row!["Min Grade", self.lowest_grade]);

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.printstd();
    }

    pub fn students(&self, students: &[Student]) {
        let mut table = Table::new();
        table.set_titles(row![c->"Name", c->"Grade", c->"Percentile", c->"Rank"]);

        for student in students {
            let colored_grade = if student.grade >= self.max_grade / 2.0 {
                student.grade.to_string().green()
            } else {
                student.grade.to_string().red()
            };

            table.add_row(row![
                student.name,
                c->colored_grade,
                c->student.percentile.unwrap_or(0.),
                c->format!("[{}/{}]", student.rank.unwrap_or(0), self.highest_rank)
            ]);
        }

        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.printstd()
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

fn highest_rank(students: &[Student]) -> u32 {
    students
        .iter()
        .map(|s| s.rank.unwrap_or(0))
        .max()
        .unwrap_or(0)
}

fn attach_rank(students: &mut [Student]) {
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
            Some(last_grade) if grade == last_grade => last_rank.unwrap_or(0),
            _ => {
                rank += 1;
                rank
            }
        };
        students[student_index].rank = Some(rank as u32);
        last_grade = Some(grade);
        last_rank = Some(rank);
    }
}

fn attach_percentile(students: &mut [Student]) {
    let total_students = students.len();

    // Create a vector of indices that represent the original order of the students
    let mut indices: Vec<usize> = (0..total_students).collect();

    let max_grade = students
        .iter()
        .map(|s| s.grade)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap_or(0.0);

    indices.sort_by(|&a, &b| {
        students[a]
            .grade
            .partial_cmp(&students[b].grade)
            .unwrap_or(Ordering::Equal)
    });

    let mut last_grade = None;
    let mut last_percentile = None;

    for (index, &student_index) in indices.iter().enumerate() {
        let grade = students[student_index].grade;
        let percentile = match last_grade {
            Some(last_grade) if grade == last_grade => last_percentile.unwrap_or(0.0),
            _ => {
                if grade == max_grade {
                    100.0
                } else {
                    index as f32 / (total_students - 1) as f32 * 100.0
                }
            }
        };
        students[student_index].percentile = Some(percentile);
        last_grade = Some(grade);
        last_percentile = Some(percentile);
    }
}

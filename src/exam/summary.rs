use colored::Colorize;
use prettytable::{format, row, Table};

use super::Student;
use crate::exam::statistics;

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
    pub min_grade: f32,
    pub highest_rank: u32,
}

impl ExamStatistics {
    pub fn new(students: &mut [Student], max_grade: f32) -> Self {
        statistics::attach_rank(students);
        statistics::attach_percentile(students);

        let total_students = students.len() as u32;
        let passed_students = statistics::passed_students(students, max_grade);
        let failed_students = total_students - passed_students;
        let pass_rate = passed_students as f32 / total_students as f32 * 100.0;
        let mean = statistics::mean(students);
        let median = statistics::median(students);
        let std_dev = statistics::std_deviation(students, mean);
        let max_grade = statistics::max_student_grade(students);
        let min_grade = statistics::min_student_grade(students);
        let highest_rank = statistics::highest_rank(students);

        Self {
            total_students,
            passed_students,
            failed_students,
            pass_rate,
            mean,
            median,
            std_dev,
            max_grade,
            min_grade,
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
        table.add_row(row!["Max Grade", self.max_grade]);
        table.add_row(row!["Min Grade", self.min_grade]);

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

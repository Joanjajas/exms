use crate::exam::Student;
use textplots::{AxisBuilder, Chart, LabelBuilder, LabelFormat, LineStyle, Plot, Shape};

pub fn histogram(students: &[Student], max_grade: f32) {
    let data: Vec<(f32, f32)> = students
        .iter()
        .map(|s| {
            (
                1.0,
                if s.grade < 10.0 {
                    s.grade + 1.0
                } else {
                    s.grade
                },
            )
        })
        .collect();
    let hist = textplots::utils::histogram(&data, 0.0, max_grade + 1.0, (max_grade + 1.0) as usize);

    println!();

    Chart::new(180, 60, 0.0, max_grade)
        .x_axis_style(LineStyle::Solid)
        .y_axis_style(LineStyle::Solid)
        .x_label_format(LabelFormat::Custom(Box::new(|x| format!("{}", x))))
        .y_label_format(LabelFormat::Custom(Box::new(|x| format!("{}", x))))
        .lineplot(&Shape::Bars(&hist))
        .nice();
}

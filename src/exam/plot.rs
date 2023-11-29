use colored::Colorize;
use term_size::dimensions_stdout;
use termplot::{plot::Histogram, Domain, Plot, Size};

use crate::exam::Student;

pub fn histogram(students: &[Student], max_grade: f32) {
    let mut overflow = false;

    let grades: Vec<f64> = students
        .iter()
        .map(|s| {
            if s.grade > max_grade {
                overflow = true;
                (max_grade - 0.01) as f64
                // We subtract 0.01 to avoid the last grade to be in the next
                // bucket
            } else if s.grade == max_grade {
                (s.grade - 0.01) as f64
            } else {
                s.grade as f64
            }
        })
        .collect();

    let buckets_range = (0..max_grade.ceil() as usize)
        .map(|i| i as f64..(i + 1) as f64)
        .collect();

    let hist = Histogram::new(grades.clone(), buckets_range);

    let mut max_bucket_size = 0;
    let mut buckets = vec![0; max_grade.ceil() as usize];
    for grade in &grades {
        let bucket = grade.floor() as usize;
        buckets[bucket] += 1;
        if buckets[bucket] > max_bucket_size {
            max_bucket_size = buckets[bucket];
        }
    }

    let (term_width, term_height) = dimensions_stdout().unwrap_or((80, 24));

    let mut plot = Plot::default();
    plot.set_domain(Domain(0.0..max_grade as f64))
        .set_codomain(Domain(0.0..max_bucket_size as f64))
        .set_size(Size::new(term_width - (term_width / 2), term_height))
        .set_title("Grades Histogram")
        .set_x_label("X => [Grade Range]")
        .set_y_label("Y => [Number of Students]")
        .add_plot(Box::new(hist));

    println!("{plot}");

    if overflow {
        let warning = "Some grades were truncated to fit in the histogram as\
                       they were greater than the maximum grade.\n\
                       This does not affect to the other statistics.";
        println!("{}\n", warning.yellow());
    }
}

use exms::exam::Exam;

fn main() {
    let mut exam =
        Exam::from_file("/Users/joan/Downloads/not/econometría_(11762)/test1.toml").unwrap();

    exam.set_max_grade(30.0);
    exam.histogram();
    exam.sort_by_grade();
    exam.students();
    exam.summary();
}

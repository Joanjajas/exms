use exms::exam::Exam;

fn main() {
    let mut exam =
        Exam::from_file("/Users/joan/Downloads/not/econometría_(11762)/exampract1.toml").unwrap();

    exam.students();
    exam.histogram();
    exam.sort_by_grade();
    exam.students();
}

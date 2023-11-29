use exms::exam::Exam;

fn main() {
    let exam = Exam::from_file("/Users/joan/Downloads/not/econometría_(11762)/test1.toml").unwrap();

    exam.summary();
}

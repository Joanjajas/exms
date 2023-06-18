use criterion::{criterion_group, criterion_main, Criterion};
use exms::exam::Exam;

fn file_parsing_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("FileParsing");
    for i in [
        "benches/bench_files/bench.toml",
        "benches/bench_files/bench.json",
    ]
    .iter()
    {
        group.bench_with_input(*i, i, |b, i| b.iter(|| Exam::from_file(i)));
    }
    group.finish();
}

criterion_group!(benches, file_parsing_bench);
criterion_main!(benches);

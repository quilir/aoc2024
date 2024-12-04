use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    for mut day in aoc2024::all_days() {
        day.load_input();
        c.bench_function(format!("{:02}", day.number()).as_str(), |b| {
            b.iter(|| day.solve())
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

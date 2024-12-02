
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let days: Vec<Box<dyn aoc2024::Day>> = aoc2024::all_days();
    for mut day in days {
        day.load_data();
        c.bench_function(format!("{:02}", day.number()).as_str(), |b| b.iter(|| day.solve()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


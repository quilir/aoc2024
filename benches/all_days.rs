use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut all_days = aoc2024::all_days();
    for day in all_days.iter_mut() {
        day.load_input();
        c.bench_function(format!("{:02}", day.number()).as_str(), |b| {
            b.iter(|| day.solve())
        });
    }
    c.bench_function("all_days", |b| {
        b.iter(|| {
            all_days.iter().for_each(|d| {
                d.solve();
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};

use aoc2020::solutions::build_runner;

fn criterion_benchmark(c: &mut Criterion) {
    let runner = build_runner();
    for name in runner.list() {
        c.bench_function(name, |b| b.iter(|| runner.run(name)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

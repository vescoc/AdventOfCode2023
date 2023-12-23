use criterion::{criterion_group, criterion_main, Criterion};

use day23::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(part_1));
    c.bench_function("part 2", |b| b.iter(part_2));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);

criterion_main!(benches);

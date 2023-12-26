use std::hint::black_box;

use lazy_static::lazy_static;

use criterion::{criterion_group, criterion_main, Criterion};

use day25::solve_1;

lazy_static! {
    static ref EXAMPLE_1: &'static str = include_str!("../../example1");
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test part 1", |b| b.iter(|| black_box(solve_1(&EXAMPLE_1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

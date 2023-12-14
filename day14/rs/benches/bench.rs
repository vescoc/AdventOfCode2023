use lazy_static::lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day14::*;

lazy_static! {
    static ref EXAMPLE_1: &'static str = include_str!("../../example1");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(part_1));
    c.bench_function("part 2", |b| b.iter(part_2));
}

fn bench_simple(c: &mut Criterion) {
    let (tiles, ncols, nrows) = parse(&EXAMPLE_1).expect("invalid input");

    let tiles = tiles.to_vec();

    c.bench_function("simple cycle example 1", |b| {
        b.iter(|| black_box(simple::cycle(tiles.clone(), ncols, nrows)))
    });
    c.bench_function("simple load example 1", |b| {
        b.iter(|| black_box(simple::load(&tiles, ncols, nrows)))
    });
}

#[cfg(feature = "simd")]
fn bench_simd(c: &mut Criterion) {
    let (tiles, ncols, nrows) = parse(&EXAMPLE_1).expect("invalid input");

    let tiles = tiles.to_vec();

    c.bench_function("simd cycle example 1", |b| {
        b.iter(|| black_box(simd::cycle::<16>(tiles.clone(), ncols, nrows)))
    });
    c.bench_function("simd load example 1", |b| {
        b.iter(|| black_box(simd::load::<16>(&tiles, ncols, nrows)))
    });
}

#[cfg(feature = "simd")]
criterion_group!(benches, criterion_benchmark, bench_simple, bench_simd);

#[cfg(not(feature = "simd"))]
criterion_group!(benches, criterion_benchmark, bench_simple);

criterion_main!(benches);

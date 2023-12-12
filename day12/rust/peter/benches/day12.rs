use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mr_kaffee_2023_12::*;

pub fn sol_benchmark(c: &mut Criterion) {
    let data = InputT::from(read_input());
    c.bench_function("star_1", |b| b.iter(|| star_1(black_box(&data))));
    c.bench_function("star_2", |b| b.iter(|| star_2(black_box(&data))));
}

criterion_group!(benches, sol_benchmark);
criterion_main!(benches);

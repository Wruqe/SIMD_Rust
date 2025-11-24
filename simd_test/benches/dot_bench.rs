use criterion::{criterion_group, criterion_main, Criterion};
use simd_test::{dot_basic, dot_simd};

pub fn bench_dot(c: &mut Criterion) {
    let a: Vec<f32> = (0..4096).map(|x| x as f32).collect();
    let b: Vec<f32> = (0..4096).map(|x| (x * 2) as f32).collect();

    c.bench_function("dot_basic 4096 floats", |bench| {
        bench.iter(|| dot_basic(&a, &b))
    });

    c.bench_function("dot_simd 4096 floats", |bench| {
        bench.iter(|| dot_simd(&a, &b))
    });
}

criterion_group!(benches, bench_dot);
criterion_main!(benches);

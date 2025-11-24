# 🚀 SIMD Dot Product Benchmark

## Rust • AVX • Benchmarking with Criterion • SIMD vs Scalar

### This project benchmarks the performance difference between a standard scalar dot product and a SIMD-accelerated dot product using Rust’s experimental core::simd API.

#### The goal:

Show how modern CPUs can multiply & add 8 values at once using AVX vector lanes.
And yeah… it’s fast.

## What's Inside

### A clean scalar implementation

```Rust
pub fn dot_basic(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
```

### A SIMD-powered implementation (8-lane f32 vectors)

```Rust
use core::simd::f32x8;

pub fn dot_simd(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = f32x8::splat(0.0);

    for i in (0..a.len()).step_by(8) {
        let va = f32x8::from_slice(&a[i..]);
        let vb = f32x8::from_slice(&b[i..]);
        sum += va * vb;
    }

    sum.reduce_sum()
}
```

## What's the difference?

### SIMD = Single Intruction, Multiple Data. Which means you will be multiplying 8 numbers at once instead of 1. The CPU has vector registers that hold 8 lanes of floats.

## 🧠 Why SIMD is faster

### SIMD = Single Instruction, Multiple Data

Instead of:

```csharp
multiply 1 float → add 1 float → repeat
```

You do:

```csharp

multiply 8 floats → add 8 floats → repeat
```

## 📊 Benchmark Setup

- 4096 floats in each vector

- Criterion benchmarking

- Release mode

- AVX/AVX2 supported CPU

- MacOS Rust nightly (core::simd)

```Rust
c.bench_function("dot_basic 4096 floats", |bench| {
    bench.iter(|| dot_basic(&a, &b))
});

c.bench_function("dot_simd 4096 floats", |bench| {
    bench.iter(|| dot_simd(&a, &b))
});
```

## 🏆 Final Measurements

### (dot_basic) ≈ 1946 ns

### (dot_simd) ≈ 0.337 µs

### Summary

- The SIMD Speedup: is literally 6x faster, this is amazing for performance and really neat.
- If you are interested pull the code, and run cargo bench to see the results for yourself.

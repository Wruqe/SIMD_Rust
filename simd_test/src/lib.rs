#![feature(portable_simd)]
use core::simd::f32x8;
use std::simd::num::SimdFloat;
// were packing 8 f32 numbers into a single CPU register, to do 8 multiplications in one go.

// A normal CPU register holds 1 number
// A vector register holds multiple nymbers at the same time.

// AVX = Advanced Vector Extensions

// AVX register = 256 bits so it can hold 8 floats in one vector

pub fn dot_basic(a: &[f32], b: &[f32]) -> f32 {
    // Turn a into iterables
    // zip 2 iterators into a pair
    // zip basically calls a next() method and bounces between both iterables making pairs
    // map just takes each tuple and combines it into one val
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn dot_simd(a: &[f32], b: &[f32]) -> f32 {
    // creates a vector wih 8 0.0s
    let mut sum = f32x8::splat(0.0);

    // every iteration we proccess 8 floats at once
    // use a vector register with 8 lanes, then it loads 8 floats into the SIMD register
    for i in (0..a.len()).step_by(8) {
        let va = f32x8::from_slice(&a[i..]);
        let vb = f32x8::from_slice(&b[i..]);

        // there are literally 8 lanes inside of the cpu
        // this is how its definded on a CPU hardware level
        // CPU register now holds 8 nmbers to process in parrell
        sum += va * vb;
    }
    sum.reduce_sum()
}

// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#![feature(codeview_annotation)]
#![allow(clippy::needless_range_loop)]

use std::hint::{CodeViewAnnotationArgs, black_box, codeview_annotation};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

// Every expansion creates its own `Args` type; identical payloads still lower
// to the same uniqued LLVM metadata tuple.
macro_rules! annotate {
    ($($s:literal),* $(,)?) => {{
        struct Args;
        impl CodeViewAnnotationArgs for Args {
            const ARGS: &[&str] = &[$($s),*];
        }
        codeview_annotation::<Args>();
    }};
}

// ============================================================================
// Family 1: MIR inlining and downstream simplification
// ============================================================================

#[inline]
fn f1_decode_base(x: u32, shift: u32, mask: u32) -> u32 {
    (x >> shift) & mask
}

#[inline]
fn f1_decode_annot(x: u32, shift: u32, mask: u32) -> u32 {
    annotate!("f1.decode");
    (x >> shift) & mask
}

#[unsafe(no_mangle)]
pub fn f1_base(x: u32) -> u32 {
    f1_decode_base(x, 4, 0xF) + f1_decode_base(x, 8, 0xFF)
}

#[unsafe(no_mangle)]
pub fn f1_annot(x: u32) -> u32 {
    f1_decode_annot(x, 4, 0xF) + f1_decode_annot(x, 8, 0xFF)
}

// ============================================================================
// Family 2: LLVM inlining (multiple vs. sole call site)
// ============================================================================

#[inline]
fn f2_poly_base(x: u64, k: u64) -> u64 {
    let mut r = x;
    for _ in 0..k {
        r = r.wrapping_mul(31).wrapping_add(7);
    }
    r
}

#[inline]
fn f2_poly_annot(x: u64, k: u64) -> u64 {
    annotate!("f2.poly");
    let mut r = x;
    for _ in 0..k {
        r = r.wrapping_mul(31).wrapping_add(7);
    }
    r
}

#[inline]
fn f2_poly_annot_sole(x: u64, k: u64) -> u64 {
    annotate!("f2.poly.sole");
    let mut r = x;
    for _ in 0..k {
        r = r.wrapping_mul(31).wrapping_add(7);
    }
    r
}

#[unsafe(no_mangle)]
pub fn f2_base(x: u64) -> u64 {
    f2_poly_base(x, 3) ^ f2_poly_base(x, 5)
}

#[unsafe(no_mangle)]
pub fn f2_annot(x: u64) -> u64 {
    f2_poly_annot(x, 3) ^ f2_poly_annot(x, 5)
}

#[unsafe(no_mangle)]
pub fn f2_annot_sole(x: u64) -> u64 {
    f2_poly_annot_sole(x, 3)
}

// ============================================================================
// Family 3: Loop vectorization, elementwise
// ============================================================================

#[unsafe(no_mangle)]
pub fn f3_base(dst: &mut [f32], a: &[f32], b: &[f32]) {
    let n = dst.len().min(a.len()).min(b.len());
    let (dst, a, b) = (&mut dst[..n], &a[..n], &b[..n]);
    for i in 0..n {
        dst[i] = a[i] * 2.0 + b[i];
    }
}

#[unsafe(no_mangle)]
pub fn f3_annot_in_loop(dst: &mut [f32], a: &[f32], b: &[f32]) {
    let n = dst.len().min(a.len()).min(b.len());
    let (dst, a, b) = (&mut dst[..n], &a[..n], &b[..n]);
    for i in 0..n {
        annotate!("f3.body");
        dst[i] = a[i] * 2.0 + b[i];
    }
}

#[unsafe(no_mangle)]
pub fn f3_annot_before_loop(dst: &mut [f32], a: &[f32], b: &[f32]) {
    annotate!("f3.entry");
    let n = dst.len().min(a.len()).min(b.len());
    let (dst, a, b) = (&mut dst[..n], &a[..n], &b[..n]);
    for i in 0..n {
        dst[i] = a[i] * 2.0 + b[i];
    }
}

fn codeview_annotation_benchmarks(c: &mut Criterion) {
    for input in [0, 1, 0x1234_5678, u32::MAX] {
        assert_eq!(f1_base(input), f1_annot(input));
    }

    let mut group = c.benchmark_group("family1_mir_inlining");
    group.bench_function("base", |b| {
        b.iter(|| black_box(f1_base(black_box(0xFEDC_BA98))))
    });
    group.bench_function("annotated", |b| {
        b.iter(|| black_box(f1_annot(black_box(0xFEDC_BA98))))
    });
    group.finish();

    for input in [0, 1, 0x1234_5678_9ABC_DEF0, u64::MAX] {
        assert_eq!(f2_base(input), f2_annot(input));
        assert_eq!(f2_poly_base(input, 3), f2_annot_sole(input));
    }

    let mut group = c.benchmark_group("family2_llvm_inlining");
    group.bench_function("base_multiple", |b| {
        b.iter(|| black_box(f2_base(black_box(0xFEDC_BA98_7654_3210))))
    });
    group.bench_function("annotated_multiple", |b| {
        b.iter(|| black_box(f2_annot(black_box(0xFEDC_BA98_7654_3210))))
    });
    group.bench_function("annotated_sole", |b| {
        b.iter(|| black_box(f2_annot_sole(black_box(0xFEDC_BA98_7654_3210))))
    });
    group.finish();

    const F3_LEN: usize = 4096;
    let f3_a: Vec<_> = (0..F3_LEN).map(|i| (i % 17) as f32).collect();
    let f3_b: Vec<_> = (0..F3_LEN).map(|i| (i % 11) as f32).collect();
    let mut f3_expected = vec![0.0; F3_LEN];
    let mut f3_in_loop = vec![0.0; F3_LEN];
    let mut f3_before_loop = vec![0.0; F3_LEN];
    f3_base(&mut f3_expected, &f3_a, &f3_b);
    f3_annot_in_loop(&mut f3_in_loop, &f3_a, &f3_b);
    f3_annot_before_loop(&mut f3_before_loop, &f3_a, &f3_b);
    assert_eq!(f3_expected, f3_in_loop);
    assert_eq!(f3_expected, f3_before_loop);

    let mut group = c.benchmark_group("family3_loop_vectorization");
    group.throughput(Throughput::Elements(F3_LEN as u64));
    group.bench_function("base", |b| {
        b.iter(|| {
            f3_base(
                black_box(&mut f3_expected),
                black_box(&f3_a),
                black_box(&f3_b),
            );
            black_box(&f3_expected);
        })
    });
    group.bench_function("annotated_in_loop", |b| {
        b.iter(|| {
            f3_annot_in_loop(
                black_box(&mut f3_in_loop),
                black_box(&f3_a),
                black_box(&f3_b),
            );
            black_box(&f3_in_loop);
        })
    });
    group.bench_function("annotated_before_loop", |b| {
        b.iter(|| {
            f3_annot_before_loop(
                black_box(&mut f3_before_loop),
                black_box(&f3_a),
                black_box(&f3_b),
            );
            black_box(&f3_before_loop);
        })
    });
    group.finish();
}

criterion_group!(benches, codeview_annotation_benchmarks);
criterion_main!(benches);

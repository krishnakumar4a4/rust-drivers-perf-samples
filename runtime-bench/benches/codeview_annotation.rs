// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#![feature(codeview_annotation)]
#![feature(core_os)]
#![allow(clippy::needless_range_loop)]

use core::os::windows::{CodeViewAnnotationArgs, codeview_annotation};
use std::hint::black_box;

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

// ============================================================================
// Family 6: Unrolling (fixed trip count)
// ============================================================================

#[unsafe(no_mangle)]
pub fn f6_base(x: &[u32; 16]) -> u32 {
    let mut h = 0u32;
    for i in 0..16 {
        h = h.rotate_left(5) ^ x[i];
    }
    h
}

#[unsafe(no_mangle)]
pub fn f6_annot_in_loop(x: &[u32; 16]) -> u32 {
    let mut h = 0u32;
    for i in 0..16 {
        annotate!("f6.body");
        h = h.rotate_left(5) ^ x[i];
    }
    h
}

#[unsafe(no_mangle)]
pub fn f6_annot_preheader(x: &[u32; 16]) -> u32 {
    annotate!("f6.preheader");
    let mut h = 0u32;
    for i in 0..16 {
        h = h.rotate_left(5) ^ x[i];
    }
    h
}

// ============================================================================
// Family 7: Branch unswitching and vectorization
// ============================================================================

// Neither arm can be vectorized.
#[unsafe(no_mangle)]
pub fn f7_base(v: &mut [u64], mode: bool) -> u64 {
    let mut acc = 1u64;
    for x in v.iter_mut() {
        if mode {
            acc = acc
                .wrapping_mul(*x | 1)
                .rotate_left(13)
                .wrapping_add(0x9E37_79B9);
        } else {
            acc = (acc ^ x.rotate_left(7)).wrapping_mul(0xFF51_AFD7_ED55_8CCD) >> 3;
        }
        *x = acc;
    }
    acc
}

#[unsafe(no_mangle)]
pub fn f7_annot_rare_arm(v: &mut [u64], mode: bool) -> u64 {
    let mut acc = 1u64;
    for x in v.iter_mut() {
        if mode {
            acc = acc
                .wrapping_mul(*x | 1)
                .rotate_left(13)
                .wrapping_add(0x9E37_79B9);
        } else {
            annotate!("f7.rare");
            acc = (acc ^ x.rotate_left(7)).wrapping_mul(0xFF51_AFD7_ED55_8CCD) >> 3;
        }
        *x = acc;
    }
    acc
}

// Each arm is vectorizable on its own.
#[unsafe(no_mangle)]
pub fn f7v_base(dst: &mut [u32], a: &[u32], mode: bool) {
    let n = dst.len().min(a.len());
    let (dst, a) = (&mut dst[..n], &a[..n]);
    for i in 0..n {
        if mode {
            dst[i] = a[i].wrapping_mul(3).wrapping_add(1);
        } else {
            dst[i] = a[i].rotate_left(7) ^ 0x5555_5555;
        }
    }
}

#[unsafe(no_mangle)]
pub fn f7v_annot(dst: &mut [u32], a: &[u32], mode: bool) {
    let n = dst.len().min(a.len());
    let (dst, a) = (&mut dst[..n], &a[..n]);
    for i in 0..n {
        if mode {
            dst[i] = a[i].wrapping_mul(3).wrapping_add(1);
        } else {
            annotate!("f7v.rare");
            dst[i] = a[i].rotate_left(7) ^ 0x5555_5555;
        }
    }
}

// Move the mode test out of the loop and annotate before the rare loop.
#[unsafe(no_mangle)]
pub fn f7v_manual(dst: &mut [u32], a: &[u32], mode: bool) {
    let n = dst.len().min(a.len());
    let (dst, a) = (&mut dst[..n], &a[..n]);
    if mode {
        for i in 0..n {
            dst[i] = a[i].wrapping_mul(3).wrapping_add(1);
        }
    } else {
        annotate!("f7v.rare");
        for i in 0..n {
            dst[i] = a[i].rotate_left(7) ^ 0x5555_5555;
        }
    }
}

// ============================================================================
// Family 8: Loop idiom recognition
// ============================================================================

// Fill (memset idiom).
#[unsafe(no_mangle)]
pub fn f8_fill_base(d: &mut [u8], v: u8) {
    for x in d.iter_mut() {
        *x = v;
    }
}

#[unsafe(no_mangle)]
pub fn f8_fill_annot(d: &mut [u8], v: u8) {
    for x in d.iter_mut() {
        annotate!("f8.fill");
        *x = v;
    }
}

#[unsafe(no_mangle)]
pub fn f8_fill_annot_before(d: &mut [u8], v: u8) {
    annotate!("f8.fill.before");
    for x in d.iter_mut() {
        *x = v;
    }
}

// Copy (memcpy idiom).
#[unsafe(no_mangle)]
pub fn f8_copy_base(d: &mut [u8], s: &[u8]) {
    let n = d.len().min(s.len());
    for i in 0..n {
        d[i] = s[i];
    }
}

#[unsafe(no_mangle)]
pub fn f8_copy_annot(d: &mut [u8], s: &[u8]) {
    let n = d.len().min(s.len());
    for i in 0..n {
        annotate!("f8.copy");
        d[i] = s[i];
    }
}

// Count set bits (popcount idiom).
#[unsafe(no_mangle)]
pub fn f8_popcount_base(mut x: u64) -> u32 {
    let mut c = 0;
    while x != 0 {
        x &= x - 1;
        c += 1;
    }
    c
}

#[unsafe(no_mangle)]
pub fn f8_popcount_annot(mut x: u64) -> u32 {
    let mut c = 0;
    while x != 0 {
        annotate!("f8.popcount");
        x &= x - 1;
        c += 1;
    }
    c
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

    let f6_input: [u32; 16] =
        std::array::from_fn(|i| (i as u32).wrapping_mul(0x9E37_79B9).wrapping_add(7));
    let f6_expected = f6_base(&f6_input);
    assert_eq!(f6_expected, f6_annot_in_loop(&f6_input));
    assert_eq!(f6_expected, f6_annot_preheader(&f6_input));

    let mut group = c.benchmark_group("family6_fixed_trip_unrolling");
    group.throughput(Throughput::Elements(f6_input.len() as u64));
    group.bench_function("base", |b| {
        b.iter(|| black_box(f6_base(black_box(&f6_input))))
    });
    group.bench_function("annotated_in_loop", |b| {
        b.iter(|| black_box(f6_annot_in_loop(black_box(&f6_input))))
    });
    group.bench_function("annotated_preheader", |b| {
        b.iter(|| black_box(f6_annot_preheader(black_box(&f6_input))))
    });
    group.finish();

    const F7_LEN: usize = 4096;
    let f7_seed: Vec<_> = (0..F7_LEN)
        .map(|i| {
            (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(0xD1B5_4A32_D192_ED03)
        })
        .collect();
    let f7v_input: Vec<_> = (0..F7_LEN)
        .map(|i| (i as u32).wrapping_mul(0x9E37_79B9).wrapping_add(7))
        .collect();

    for mode in [true, false] {
        let mut f7_expected = f7_seed.clone();
        let mut f7_annotated = f7_seed.clone();
        let expected_acc = f7_base(&mut f7_expected, mode);
        let annotated_acc = f7_annot_rare_arm(&mut f7_annotated, mode);
        assert_eq!(expected_acc, annotated_acc);
        assert_eq!(f7_expected, f7_annotated);

        let mut f7v_expected = vec![0; F7_LEN];
        let mut f7v_annotated = vec![0; F7_LEN];
        let mut f7v_manual_result = vec![0; F7_LEN];
        f7v_base(&mut f7v_expected, &f7v_input, mode);
        f7v_annot(&mut f7v_annotated, &f7v_input, mode);
        f7v_manual(&mut f7v_manual_result, &f7v_input, mode);
        assert_eq!(f7v_expected, f7v_annotated);
        assert_eq!(f7v_expected, f7v_manual_result);
    }

    let mut group = c.benchmark_group("family7_branch_vectorization");
    group.throughput(Throughput::Elements(F7_LEN as u64));
    for (mode_name, mode) in [("common", true), ("rare", false)] {
        let mut f7_base_values = f7_seed.clone();
        let mut f7_annotated_values = f7_seed.clone();
        group.bench_function(format!("original/{mode_name}/base"), |b| {
            b.iter(|| black_box(f7_base(black_box(&mut f7_base_values), black_box(mode))))
        });
        group.bench_function(format!("original/{mode_name}/annotated"), |b| {
            b.iter(|| {
                black_box(f7_annot_rare_arm(
                    black_box(&mut f7_annotated_values),
                    black_box(mode),
                ))
            })
        });

        let mut f7v_base_values = vec![0; F7_LEN];
        let mut f7v_annotated_values = vec![0; F7_LEN];
        let mut f7v_manual_values = vec![0; F7_LEN];
        group.bench_function(format!("vectorizable/{mode_name}/base"), |b| {
            b.iter(|| {
                f7v_base(
                    black_box(&mut f7v_base_values),
                    black_box(&f7v_input),
                    black_box(mode),
                );
                black_box(&f7v_base_values);
            })
        });
        group.bench_function(format!("vectorizable/{mode_name}/annotated"), |b| {
            b.iter(|| {
                f7v_annot(
                    black_box(&mut f7v_annotated_values),
                    black_box(&f7v_input),
                    black_box(mode),
                );
                black_box(&f7v_annotated_values);
            })
        });
        group.bench_function(format!("vectorizable/{mode_name}/manual"), |b| {
            b.iter(|| {
                f7v_manual(
                    black_box(&mut f7v_manual_values),
                    black_box(&f7v_input),
                    black_box(mode),
                );
                black_box(&f7v_manual_values);
            })
        });
    }
    group.finish();

    const F8_LEN: usize = 4096;
    const F8_FILL_VALUE: u8 = 0xA5;
    const F8_POPCOUNT_INPUT: u64 = 0xF0F0_F0F0_F0F0_F0F0;
    let f8_source: Vec<_> = (0..F8_LEN)
        .map(|i| (i as u8).wrapping_mul(31).wrapping_add(7))
        .collect();
    let mut f8_fill_expected = vec![0; F8_LEN];
    let mut f8_fill_annotated = vec![0; F8_LEN];
    let mut f8_fill_before = vec![0; F8_LEN];
    f8_fill_base(&mut f8_fill_expected, F8_FILL_VALUE);
    f8_fill_annot(&mut f8_fill_annotated, F8_FILL_VALUE);
    f8_fill_annot_before(&mut f8_fill_before, F8_FILL_VALUE);
    assert_eq!(f8_fill_expected, f8_fill_annotated);
    assert_eq!(f8_fill_expected, f8_fill_before);

    let mut f8_copy_expected = vec![0; F8_LEN];
    let mut f8_copy_annotated = vec![0; F8_LEN];
    f8_copy_base(&mut f8_copy_expected, &f8_source);
    f8_copy_annot(&mut f8_copy_annotated, &f8_source);
    assert_eq!(f8_copy_expected, f8_copy_annotated);

    for input in [0, 1, F8_POPCOUNT_INPUT, u64::MAX] {
        assert_eq!(f8_popcount_base(input), input.count_ones());
        assert_eq!(f8_popcount_base(input), f8_popcount_annot(input));
    }

    let mut group = c.benchmark_group("family8_memory_idioms");
    group.throughput(Throughput::Bytes(F8_LEN as u64));
    group.bench_function("fill/base", |b| {
        b.iter(|| {
            f8_fill_base(black_box(&mut f8_fill_expected), black_box(F8_FILL_VALUE));
            black_box(&f8_fill_expected);
        })
    });
    group.bench_function("fill/annotated_in_loop", |b| {
        b.iter(|| {
            f8_fill_annot(black_box(&mut f8_fill_annotated), black_box(F8_FILL_VALUE));
            black_box(&f8_fill_annotated);
        })
    });
    group.bench_function("fill/annotated_before_loop", |b| {
        b.iter(|| {
            f8_fill_annot_before(black_box(&mut f8_fill_before), black_box(F8_FILL_VALUE));
            black_box(&f8_fill_before);
        })
    });
    group.bench_function("copy/base", |b| {
        b.iter(|| {
            f8_copy_base(black_box(&mut f8_copy_expected), black_box(&f8_source));
            black_box(&f8_copy_expected);
        })
    });
    group.bench_function("copy/annotated_in_loop", |b| {
        b.iter(|| {
            f8_copy_annot(black_box(&mut f8_copy_annotated), black_box(&f8_source));
            black_box(&f8_copy_annotated);
        })
    });
    group.finish();

    let mut group = c.benchmark_group("family8_popcount");
    group.bench_function("base", |b| {
        b.iter(|| black_box(f8_popcount_base(black_box(F8_POPCOUNT_INPUT))))
    });
    group.bench_function("annotated", |b| {
        b.iter(|| black_box(f8_popcount_annot(black_box(F8_POPCOUNT_INPUT))))
    });
    group.finish();
}

criterion_group!(benches, codeview_annotation_benchmarks);
criterion_main!(benches);

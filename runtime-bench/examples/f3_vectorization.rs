// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone copy of Family 3 from `benches/codeview_annotation.rs`, used to
//! inspect loop vectorization behaviour in the generated assembly.

#![feature(codeview_annotation)]
#![feature(core_os)]
#![allow(clippy::needless_range_loop)]

use core::os::windows::{CodeViewAnnotationArgs, codeview_annotation};
use std::hint::black_box;

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

fn main() {
    const LEN: usize = 4096;
    let a: Vec<_> = (0..LEN).map(|i| (i % 17) as f32).collect();
    let b: Vec<_> = (0..LEN).map(|i| (i % 11) as f32).collect();

    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so none of these functions can be inlined here.
    for (name, function) in [
        ("f3_base", f3_base as fn(&mut [f32], &[f32], &[f32])),
        ("f3_annot_in_loop", f3_annot_in_loop),
        ("f3_annot_before_loop", f3_annot_before_loop),
    ] {
        let function = black_box(function);
        let mut dst = vec![0.0; LEN];
        function(&mut dst, &a, &b);
        let sum: f32 = black_box(&dst).iter().sum();
        println!("{name}: sum = {sum}");
    }
}

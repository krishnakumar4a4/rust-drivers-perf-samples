// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone copy of Family 2 from `benches/codeview_annotation.rs`, used to
//! inspect LLVM inlining behaviour in the generated assembly.

#![feature(codeview_annotation)]
#![feature(core_os)]

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

fn main() {
    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so none of these functions can be inlined here.
    for (name, function) in [
        ("f2_base", f2_base as fn(u64) -> u64),
        ("f2_annot", f2_annot),
        ("f2_annot_sole", f2_annot_sole),
    ] {
        let function = black_box(function);
        let result = black_box(function(0xFEDC_BA98_7654_3210));
        println!("{name}: {result:#x}");
    }
}

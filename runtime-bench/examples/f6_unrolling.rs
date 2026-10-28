// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone extension of Family 6 from `benches/codeview_annotation.rs`, used
//! to inspect fixed-trip-count loop unrolling with direct and helper annotations.

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

fn annotate_body() {
    annotate!("f6.body");
}

#[inline(always)]
fn annotate_body_inline() {
    annotate!("f6.body");
}

#[inline(never)]
fn annotate_body_noinline() {
    annotate!("f6.body");
}

#[unsafe(no_mangle)]
pub fn f6_annot_in_loop_helper(x: &[u32; 16]) -> u32 {
    let mut h = 0u32;
    for i in 0..16 {
        annotate_body();
        h = h.rotate_left(5) ^ x[i];
    }
    h
}

#[unsafe(no_mangle)]
pub fn f6_annot_in_loop_helper_inline(x: &[u32; 16]) -> u32 {
    let mut h = 0u32;
    for i in 0..16 {
        annotate_body_inline();
        h = h.rotate_left(5) ^ x[i];
    }
    h
}

#[unsafe(no_mangle)]
pub fn f6_annot_in_loop_helper_noinline(x: &[u32; 16]) -> u32 {
    let mut h = 0u32;
    for i in 0..16 {
        annotate_body_noinline();
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

fn main() {
    let input: [u32; 16] =
        std::array::from_fn(|i| (i as u32).wrapping_mul(0x9E37_79B9).wrapping_add(7));
    let expected = input.iter().fold(0u32, |h, &x| h.rotate_left(5) ^ x);

    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so none of these functions can be inlined here.
    for (name, function) in [
        ("f6_base", f6_base as fn(&[u32; 16]) -> u32),
        ("f6_annot_in_loop", f6_annot_in_loop),
        ("f6_annot_in_loop_helper", f6_annot_in_loop_helper),
        (
            "f6_annot_in_loop_helper_inline",
            f6_annot_in_loop_helper_inline,
        ),
        (
            "f6_annot_in_loop_helper_noinline",
            f6_annot_in_loop_helper_noinline,
        ),
        ("f6_annot_preheader", f6_annot_preheader),
    ] {
        let function = black_box(function);
        let result = black_box(function(&input));
        assert_eq!(result, expected, "{name}");
        println!("{name}: {result:#x}");
    }
}

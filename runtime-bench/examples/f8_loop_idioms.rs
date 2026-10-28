// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone copy of Family 8 from `benches/codeview_annotation.rs`, used to
//! inspect loop idiom recognition (memset, memcpy, popcount) in the generated
//! assembly.

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

fn main() {
    const LEN: usize = 4096;
    const FILL_VALUE: u8 = 0xA5;
    const POPCOUNT_INPUT: u64 = 0xF0F0_F0F0_F0F0_F0F0;
    let source: Vec<_> = (0..LEN)
        .map(|i| (i as u8).wrapping_mul(31).wrapping_add(7))
        .collect();
    let checksum = |d: &[u8]| d.iter().fold(0u32, |h, &x| h.wrapping_mul(31).wrapping_add(u32::from(x)));

    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so none of these functions can be inlined here.
    for (name, function) in [
        ("f8_fill_base", f8_fill_base as fn(&mut [u8], u8)),
        ("f8_fill_annot", f8_fill_annot),
        ("f8_fill_annot_before", f8_fill_annot_before),
    ] {
        let function = black_box(function);
        let mut d = vec![0; LEN];
        function(&mut d, FILL_VALUE);
        println!("{name}: checksum = {:#x}", checksum(black_box(&d)));
    }

    for (name, function) in [
        ("f8_copy_base", f8_copy_base as fn(&mut [u8], &[u8])),
        ("f8_copy_annot", f8_copy_annot),
    ] {
        let function = black_box(function);
        let mut d = vec![0; LEN];
        function(&mut d, &source);
        println!("{name}: checksum = {:#x}", checksum(black_box(&d)));
    }

    for (name, function) in [
        ("f8_popcount_base", f8_popcount_base as fn(u64) -> u32),
        ("f8_popcount_annot", f8_popcount_annot),
    ] {
        let function = black_box(function);
        let result = black_box(function(POPCOUNT_INPUT));
        println!("{name}: {result}");
    }
}

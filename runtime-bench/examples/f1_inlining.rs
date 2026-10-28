// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone copy of Family 1 from `benches/codeview_annotation.rs`, used to
//! inspect inlining behaviour in the generated assembly.

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

fn main() {
    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so neither function can be inlined here.
    for (name, function) in [("f1_base", f1_base as fn(u32) -> u32), ("f1_annot", f1_annot)] {
        let function = black_box(function);
        let result = black_box(function(0xFEDC_BA98));
        println!("{name}: {result}");
    }
}

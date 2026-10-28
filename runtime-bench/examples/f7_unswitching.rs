// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Standalone copy of Family 7 from `benches/codeview_annotation.rs`, used to
//! inspect branch unswitching and vectorization in the generated assembly.

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

fn main() {
    const LEN: usize = 4096;
    let seed: Vec<_> = (0..LEN)
        .map(|i| {
            (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(0xD1B5_4A32_D192_ED03)
        })
        .collect();
    let input: Vec<_> = (0..LEN)
        .map(|i| (i as u32).wrapping_mul(0x9E37_79B9).wrapping_add(7))
        .collect();

    // Same indirection as `benches/codeview_annotation.rs`: hiding the pointer
    // keeps the call indirect, so none of these functions can be inlined here.
    for (mode_name, mode) in [("common", true), ("rare", false)] {
        for (name, function) in [
            ("f7_base", f7_base as fn(&mut [u64], bool) -> u64),
            ("f7_annot_rare_arm", f7_annot_rare_arm),
        ] {
            let function = black_box(function);
            let mut values = seed.clone();
            let acc = black_box(function(&mut values, mode));
            println!("{mode_name}/{name}: acc = {acc:#x}");
        }

        for (name, function) in [
            ("f7v_base", f7v_base as fn(&mut [u32], &[u32], bool)),
            ("f7v_annot", f7v_annot),
            ("f7v_manual", f7v_manual),
        ] {
            let function = black_box(function);
            let mut dst = vec![0; LEN];
            function(&mut dst, &input, mode);
            let checksum = black_box(&dst).iter().fold(0u32, |h, &x| h.wrapping_mul(31).wrapping_add(x));
            println!("{mode_name}/{name}: checksum = {checksum:#x}");
        }
    }
}

// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

fn main() {
    let target = std::env::var("TARGET").expect("Cargo must set TARGET");
    assert_eq!(
        target, "x86_64-pc-windows-msvc",
        "The mock WPP suite currently supports Windows x64 MSVC only"
    );
    println!("cargo:rerun-if-changed=native\\mocks.c");
    cc::Build::new()
        .file("native\\mocks.c")
        .warnings_into_errors(true)
        .compile("wpp_bench_mocks");
}

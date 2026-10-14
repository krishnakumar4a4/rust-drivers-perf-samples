// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#![feature(codeview_annotation)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

include!("..\\support.rs");

fn trace_benchmarks(c: &mut Criterion) {
    for (name, etw, ifr) in [
        ("disabled", false, false),
        ("etw", true, false),
        ("ifr", false, true),
        ("etw_and_ifr", true, true),
    ] {
        let session = Session::new(etw, ifr, false);
        let mut group = c.benchmark_group(name);
        group.bench_function("no_fields", |b| {
            let before = session.stats();
            b.iter(|| trace!(INFO, BENCHMARK, "no fields"));
            session.assert_activity_since(before, etw, ifr);
        });
        group.bench_function("u64", |b| {
            let before = session.stats();
            b.iter(|| trace!(INFO, BENCHMARK, "value={}", black_box(42_u64)));
            session.assert_activity_since(before, etw, ifr);
        });
        group.bench_function("mixed", |b| {
            let before = session.stats();
            b.iter(|| {
                trace!(
                    INFO,
                    BENCHMARK,
                    "number={}, flag={}, text={}",
                    black_box(42_u64),
                    black_box(true),
                    black_box(c"hello")
                )
            });
            session.assert_activity_since(before, etw, ifr);
        });
        group.bench_function("rust_str", |b| {
            let before = session.stats();
            b.iter(|| trace!(INFO, BENCHMARK, "text={}", black_box("hello")));
            session.assert_activity_since(before, etw, ifr);
        });
        group.finish();
        // Registration and teardown are deliberately outside every timed loop.
        let stats = session.stats();
        assert_eq!(stats.registrations, 1);
        assert_eq!(stats.set_information, 2);
    }
}

criterion_group!(benches, trace_benchmarks);
criterion_main!(benches);

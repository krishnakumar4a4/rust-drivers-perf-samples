# Mock WPP macro benchmarks

This standalone package benchmarks the real `wpp_control_guids!`-generated
`trace!` macro in a user-mode Criterion executable. Both ETW and IFR resolve to
the C symbols in [native/mocks.c](native/mocks.c), not to Windows kernel
libraries. No mock feature or backend is added to the production crates.

The nested workspace keeps Criterion, the native mock symbols, and their lockfile
separate from normal driver builds. Its `wpp` and `wpp-macros` dependencies come
from the `wdf-wpp-rust-native-frontend-impl-3` branch of
`krishnakumar4a4/windows-drivers-rs`. The local `wdf-shim` suppresses temporary
driver-only diagnostic prints emitted by that branch so they do not affect the
user-mode measurements. Do not link this package into a driver.

## Requirements and commands

- Windows x64 MSVC and the Visual C++ build tools (for `cc`).
- The latest nightly Rust toolchain (`rustup update nightly`), selected by the
  repository-root `rust-toolchain.toml`. The benchmarks enable
  `#![feature(codeview_annotation)]` and `#![feature(core_os)]` for the generic
  `core::os::windows::CodeViewAnnotationArgs` API. No custom compiler is needed.
- No WDK, loaded driver, ETW session, or administrator privileges are needed.

From the repository root in PowerShell:

```powershell
$manifest = ".\runtime-bench\Cargo.toml"
cargo +nightly test --manifest-path $manifest --test ffi
cargo +nightly bench --manifest-path $manifest --bench trace -- --test
cargo +nightly bench --manifest-path $manifest --bench trace
cargo +nightly bench --manifest-path $manifest --bench codeview_annotation --profile release -- --save-baseline release
cargo +nightly bench --manifest-path $manifest --bench codeview_annotation --profile dev -- --baseline release
```

The second command smoke-tests every benchmark without collecting statistics.
The final two commands measure the CodeView annotation scenario with optimized
release code and unoptimized debug code. The debug run compares its measurements
against the saved release baseline. The GitHub Actions workflow also uploads the
Criterion data as the `runtime-benchmark-results` artifact.
To collect a short, filtered measurement:

```powershell
cargo +nightly bench --manifest-path $manifest --bench trace -- 'etw/u64' --warm-up-time 1 --measurement-time 1 --sample-size 10
```

Criterion stores results under this package's `target\criterion` directory by
default. Keep the toolchain, build profile, machine and power settings consistent
when comparing results.

## What runs

[support.rs](support.rs) is included at the root of each test/benchmark executable
because the generated trace macro refers to `$crate` provider and IFR state.
It owns a serialized RAII fixture that:

1. Initializes the generated IFR module with opaque mock driver/registry
   stand-ins. The C start stub calls a Rust helper to populate the actual Rust
   control-block types, without duplicating their layout in C.
2. Calls the generated provider initialization. `EtwRegister` supplies a handle
   and invokes the real Rust enable callback; `EtwSetInformation` validates both
   ModernWpp configuration requests.
3. Selects enabled/disabled ETW through that callback and enabled/disabled IFR
   through the start mock.
4. Unregisters the provider and stops IFR before releasing the session lock.

Mocks support one registered provider and single-threaded trace emission at a
time. The fixture serializes test cases; this is not a concurrent ETW emulator.
Unexpected lifecycle/FFI inputs abort with a diagnostic rather than silently
succeeding. Unused IFR imports are not stubbed; newly reached imports must get
an explicit mock instead of accidentally binding to kernel code.

[tests/ffi.rs](tests/ffi.rs) captures descriptors and variadic pointer/length
pairs, checks payload bytes, decode GUIDs and event metadata, and verifies
registration, filtering, independent sink gates, repeated sessions and cleanup.
Capture is bounded to eight fields / 256 payload bytes and fails on overflow.

[benches/trace.rs](benches/trace.rs) measures zero fields, a scalar, mixed
allocation-free fields and a Rust string, each with neither sink, ETW only, IFR
only, and both sinks enabled. Registration, enablement, assertions and cleanup
are outside the timed loops. Rust strings currently include their conversion
to the WPP field representation; the C-string case avoids that allocation.

[benches/codeview_annotation.rs](benches/codeview_annotation.rs) compares
functions with and without `codeview_annotation`. Family 1 exercises MIR
inlining and downstream simplification. Family 2 exercises LLVM inlining with
multiple and sole call sites. Family 3 measures elementwise loop vectorization
with an annotation inside or before the loop and reports element throughput.
Family 6 measures fixed-trip-count loop unrolling with an annotation inside the
loop or in its preheader. Family 7 measures common and rare branch behavior for
a non-vectorizable loop, then compares automatic branch handling with manually
unswitched vectorizable loops. Family 8 exercises memset, memcpy, and popcount
loop patterns using the target's default CPU feature set. The benchmark keeps
the exported entry points visible for binary inspection. Each timed call goes
through a typed function pointer passed through `black_box` once before timing
to prevent devirtualization and inlining into Criterion's loop. Inputs and calls
are not wrapped in `black_box`; Criterion consumes each iteration's return value.
The indirect call also keeps writes to output buffers observable. Run it under
both the release and dev profiles to observe the annotation's impact with and
without optimization. Previous direct-call baselines include different call and
optimization-barrier overheads, so collect fresh baselines for comparisons.

## Inspecting fixed-trip-count unrolling

[examples/f6_unrolling.rs](examples/f6_unrolling.rs) extends the Family 6
benchmark with three helpers that emit the same `f6.body` annotation: an ordinary
function, an `#[inline(always)]` function, and an `#[inline(never)]` function.
Each helper is called once per iteration of the same 16-element loop. The
example checks every variant's result against a reference fold.

Generate optimized assembly and LLVM IR, then run the example from the
repository root:

```powershell
cargo +nightly rustc --manifest-path .\runtime-bench\Cargo.toml --release --example f6_unrolling -- --emit=asm,llvm-ir
& .\runtime-bench\target\release\examples\f6_unrolling.exe
```

The generated files are
`runtime-bench\target\release\examples\f6_unrolling.s` and
`runtime-bench\target\release\examples\f6_unrolling.ll`.
With rustc `1.101.0-nightly (1a2c4797e 2026-10-21)`, LLVM `23.1.4`, and the default
`x86_64-pc-windows-msvc` target CPU, the release assembly shows:

| Variant | Loop unrolled? | Assembly evidence |
| --- | --- | --- |
| `f6_base` | Fully | Fixed-offset loads through byte offset 60; no loop branch |
| `f6_annot_in_loop` | No | Indexed load, counter increment, compare with 16, backward `jne` |
| `f6_annot_in_loop_helper` | No | Ordinary helper is inlined; symbol aliases `f6_annot_in_loop` |
| `f6_annot_in_loop_helper_inline` | No | Forced-inline helper is inlined; symbol aliases `f6_annot_in_loop` |
| `f6_annot_in_loop_helper_noinline` | Fully | 16 helper calls and fixed-offset loads; no loop branch |
| `f6_annot_preheader` | Fully | Annotation before straight-line loads; no loop branch |

The IR declares `llvm.codeview.annotation` as `noduplicate`, preventing the
direct or inlined annotation from being duplicated by unrolling. With
`#[inline(never)]`, the annotation stays inside the helper; its callers can be
duplicated without duplicating the intrinsic itself. The helper contains an
annotation label followed by `retq`, and its IR retains `noinline`.

Thus, preventing helper inlining enables unrolling in this example, but is not
necessarily a performance improvement: it adds 16 real call/return pairs plus
register-save and stack-frame overhead. The annotation also describes an
address inside the helper rather than inside the loop function. These are
compiler-specific observations, not a guarantee for other loops or toolchains.

## Interpreting measurements

These are **generated Rust code plus mock sink** measurements, not kernel ETW
or IFR performance. The timed C sinks include argument checks, a capture-mode
branch and non-atomic counters, but do not copy payloads, allocate or print.
The native code is compiled separately without cross-language LTO, keeping the
FFI boundary opaque to Rust even when Rust LTO is enabled. This prevents Rust
from optimizing away payload/descriptor construction just because a mock sink
ignores it. Benchmark inputs also pass through `black_box`.

The disabled path still performs argument conversion before the sink gates in
the current macro; it is not necessarily just an enablement check. CodeView
annotation compilation and actual kernel work are not measured. The generated
WDF diagnostic prints have been removed; production FFI signatures and routing
remain unchanged.

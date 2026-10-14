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
- The Rust toolchain used by this checkout that implements
  `#![feature(codeview_annotation)]` and the generic
  `core::hint::CodeViewAnnotationArgs` API. An arbitrary stable or nightly
  compiler is not sufficient. The commands below use the locally linked
  `stage1` toolchain; substitute your annotation-capable toolchain name.
- No WDK, loaded driver, ETW session, or administrator privileges are needed.

From the repository root in PowerShell:

```powershell
$manifest = ".\runtime-bench\Cargo.toml"
cargo +stage1 test --manifest-path $manifest --test ffi
cargo +stage1 bench --manifest-path $manifest --bench trace -- --test
cargo +stage1 bench --manifest-path $manifest --bench trace
```

The second command smoke-tests every benchmark without collecting statistics.
To collect a short, filtered measurement:

```powershell
cargo +stage1 bench --manifest-path $manifest --bench trace -- 'etw/u64' --warm-up-time 1 --measurement-time 1 --sample-size 10
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

# KMDF 10-dummy-trace fixture

This crate is the tracing-free baseline for `sample-kmdf-with-traces`. It has
the same 10 scenario modules, emitter functions, locals, enums, and call
order, but every `trace!` statement is replaced by a `dummy_trace(...)` call
with exactly the same level, keyword, message, and three fields. Comparing the
two drivers isolates the overhead added by WPP tracing itself.

## How the dummy trace stays in optimized builds

`src\dummy_trace.rs` defines:

- `dummy_trace`, an `#[inline(always)]` call shim used at every former
  `trace!` site.
- `dummy_trace_sink`, the `#[inline(never)]` dummy trace function. Its body is
  empty; it only accepts the arguments.

The shim calls the sink through a function pointer read with
`ptr::read_volatile`, so the optimizer cannot see the call target. In release
(LTO) builds every call therefore stays, and every argument is still passed as
the Rust ABI requires.

Two simpler approaches do not work:

- `#[used]` is only accepted on `static` items; rustc rejects it on functions.
  A `#[used]` static that holds the function pointer keeps the symbol, not the
  calls.
- A direct call to an empty function keeps the `call` instruction at best
  (with `#[inline(never)]`/`#[optimize(none)]`), but LLVM dead-argument
  elimination replaces the unused arguments with `poison`, so no argument
  set-up is emitted.

The level and keyword constants mirror the WPP `TRACE_LEVEL_*` values and the
two traced providers' keyword bits. This crate does not use `wpp` directly and
registers no trace providers.

| Module | Calls | Field scenarios |
| --- | ---: | --- |
| `scenario_00_literals` | 1 | Direct `i8`, `u16`, and `bool` literals |
| `scenario_01_signed` | 1 | `i16`, `i32`, and `i64` function arguments |
| `scenario_02_unsigned` | 1 | `u8`, `u32`, and `u64` variables |
| `scenario_03_floats` | 1 | `f32`, `f64`, and `bool` variables |
| `scenario_04_strings` | 1 | `&str`, `&CStr`, and returned `&str` |
| `scenario_05_status` | 1 | `IRQL`, `NtStatus`, and returned `usize` |
| `scenario_06_enum_primary` | 1 | Explicit `u8`, sparse `u16`, and `u32` enums |
| `scenario_07_generic` | 1 | Three generic arguments |
| `scenario_08_enum_secondary` | 1 | Implicit and mixed-discriminant enums plus returned `i32` |
| `scenario_09_mixed` | 1 | `u8`, `i16`, and `usize` function arguments |

The scenario sources are reproducible with
`..\generate-trace-fixtures.ps1`. Build and package it with:

```powershell
cargo +stage1 wdk build
```

//! Empty stand-in for the WPP `trace!` macro.
//!
//! Every generated `dummy_trace(...)` call receives exactly the same level,
//! keyword, message, and three fields as the matching `trace!` statement in the
//! traced fixture of the same size. Comparing the two drivers therefore
//! isolates the cost of tracing itself from the cost of producing the traced
//! values and passing them to a call.
//!
//! Optimized (LTO) builds must keep every call *and* its argument set-up:
//!
//! - `#[used]` cannot help: rustc only accepts it on `static` items, and a
//!   `#[used]` static that points at the function keeps the symbol, not the
//!   calls.
//! - A direct call to an empty function is not enough either. Even when the
//!   call survives (`#[inline(never)]`, `#[optimize(none)]`), LLVM's
//!   dead-argument elimination sees that the parameters are unused and
//!   replaces them with `poison` at every direct call site, so no arguments
//!   are passed.
//! - [`dummy_trace`] therefore calls [`dummy_trace_sink`] through a function
//!   pointer loaded with a volatile read. The optimizer cannot know the call
//!   target, so it must emit the call and pass every argument exactly as the
//!   Rust ABI requires, while the sink's body stays empty.

use core::ptr;

// WPP levels (`TRACE_LEVEL_*` in evntrace.h).
pub(crate) const CRITICAL: u8 = 1;
pub(crate) const ERROR: u8 = 2;
pub(crate) const WARNING: u8 = 3;
pub(crate) const INFO: u8 = 4;
pub(crate) const VERBOSE: u8 = 5;

// `CoreProvider` keywords in the traced fixtures.
pub(crate) const LITERALS: u32 = 1 << 0;
pub(crate) const SIGNED: u32 = 1 << 1;
pub(crate) const UNSIGNED: u32 = 1 << 2;
pub(crate) const FLOATS: u32 = 1 << 3;
pub(crate) const STRINGS: u32 = 1 << 4;

// `DetailProvider` keywords in the traced fixtures.
pub(crate) const STATUS: u32 = 1 << 0;
pub(crate) const ENUM_PRIMARY: u32 = 1 << 1;
pub(crate) const GENERIC: u32 = 1 << 2;
pub(crate) const ENUM_SECONDARY: u32 = 1 << 3;
pub(crate) const MIXED: u32 = 1 << 4;

type DummyTraceSink<A, B, C> = fn(u8, u32, &'static str, A, B, C);

/// Replaces one `trace!(level, keyword, message, field_0, field_1, field_2)`.
#[inline(always)]
pub(crate) fn dummy_trace<A, B, C>(
    level: u8,
    keyword: u32,
    message: &'static str,
    field_0: A,
    field_1: B,
    field_2: C,
) {
    let sink: DummyTraceSink<A, B, C> = dummy_trace_sink;
    // SAFETY: `sink` is a live, properly aligned local.
    let sink = unsafe { ptr::read_volatile(&sink) };
    sink(level, keyword, message, field_0, field_1, field_2);
}

/// The dummy trace function: it only accepts its arguments.
#[inline(never)]
fn dummy_trace_sink<A, B, C>(
    _level: u8,
    _keyword: u32,
    _message: &'static str,
    _field_0: A,
    _field_1: B,
    _field_2: C,
) {
}

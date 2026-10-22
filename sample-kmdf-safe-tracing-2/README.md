# Direct tracing in KMDF entry routines

This fixture has **10 `trace!` calls directly in `driver_entry` and 10 directly in
`event_device_add`**, for 20 unique trace sites in [src/lib.rs](src/lib.rs).
There are no scenario modules, loops, or source-level trace-emitter helper functions.
The device-add routine is registered with `DriverConfig::new(event_device_add)`.
The WPP macros still generate their own CodeView annotation helpers.

Both routines exercise signed/unsigned integers, floats, booleans, Rust/C strings,
IRQL/status fields, enum fields, and mixed field types. The `GENERIC` keyword uses
concrete field types here, rather than moving the call into a generic helper.
Two providers and all five tracing levels follow the existing trace fixtures.

## Build and analyze

Use the same custom compiler and dependency versions as the earlier comparison.
Keep [Cargo.lock](Cargo.lock): the upstream branch has since migrated to a different
CodeView API and is incompatible with this older custom compiler.

From the repository root:

```powershell
$env:LIBCLANG_PATH = 'C:\work\tools\LLVM17\bin'
$env:BINDGEN_EXTRA_CLANG_ARGS = '-fms-compatibility'
Push-Location .\sample-kmdf-safe-tracing-2
try {
    cargo +stage1 wdk build --sample
    if ($LASTEXITCODE -ne 0) { throw 'Driver build failed' }
} finally {
    Pop-Location
}
python .\stack-analysis\analyze-stack.py --drivers sample-kmdf-safe-tracing-2 --output .\stack-analysis\safe-tracing-2\dev
python -m unittest discover -s .\stack-analysis -p test_analyze_stack.py -v
```

The resulting signed driver is
`target\debug\sample_kmdf_safe_tracing_2_package\sample_kmdf_safe_tracing_2.sys`.

## Measured stack frames

The dev/x64 build reserves **5,592 bytes** in the user `driver_entry` and **5,704 bytes**
in `event_device_add`, including two saved-register pushes in each. Generated annotation
helpers have separate 24-byte frames.

The normal release/LTO build reduces the callback frame to **760 bytes**. The user entry
implementation is inlined into the exported entry wrapper, whose combined frame is
**776 bytes**. There are still ten ETW and ten IFR call sites per routine, but the default
release PDB lacks the dev build's TMF decoding records.

See the [dev-versus-release report](../stack-analysis/safe-tracing-2/DEV-VS-RELEASE.md)
for per-profile results, compiler/map provenance, and release reproduction commands.

**The name "safe-tracing" is not a stack-safety guarantee.** These are individual fixed
frames, not runtime peak stack usage. Direct placement produces much larger routine frames
than the earlier samples with separate trace-site functions.

See the [full report](../stack-analysis/safe-tracing-2/REPORT.md),
[per-function CSV](../stack-analysis/safe-tracing-2/dev/functions.csv), and
[per-module CSV](../stack-analysis/safe-tracing-2/dev/modules.csv) for the binary evidence,
comparison, coverage, and limitations. No driver was loaded as part of this analysis.

# WPP stack-analysis samples

Rust and C UM/KM samples with matching provider.

- **[Rust](rust/):** `cargo +nightly rustc --manifest-path .\trace-stack-wpp\rust\Cargo.toml -- --emit=asm,llvm-ir`
- **[C UM](c-um/trace-stack-c-um.vcxproj):** Uses `tracewpp -um`; links real ETW APIs and `WppRecorderUM.lib`, producing an `.exe`. No mocks.
- **[C KM](c-km/trace-stack-c-km.vcxproj):** Uses `tracewpp -km` and `/kernel`; produces `.obj`/`.lib` without a driver link, signing, or mocks.

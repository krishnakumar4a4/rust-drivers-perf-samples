# KMDF 1000-trace fixture

This crate is the 100x WPP performance fixture. It repeats the baseline's ten
scenario families 100 times, producing 1,000 unique trace statements. Each
scenario group contains 10 shard modules with 10 statements each, for 100 leaf
trace modules overall. The first two scenario groups run from `driver_entry`;
the remaining eight run from the device-add callback.

## Composition

- Two providers: `CoreProvider` and `DetailProvider`.
- Five keywords per provider and all five WPP levels, selected with a
  deterministic rotation.
- Exactly three fields in every trace statement.
- Ten literal-only shard modules containing 100 statements (10% of the fixture).
- Nine hundred statements (90%) whose fields come from function arguments,
  local variables, enum values, generic arguments, or function return values.
- Every message and generated variable identifier includes its scenario and
  instance number, so all statements remain unique after scaling.
- Physical modules are capped at 10 trace statements. The CodeView annotation
  lowering uses stack space proportional to the number of trace sites in a
  source module, so sharding keeps driver-entry stack use safe.

| Scenario group | Leaf modules | Statements | Field scenarios |
| --- | ---: | ---: | --- |
| `scenario_00_literals` | 10 | 100 | Direct `i8`, `u16`, and `bool` literals |
| `scenario_01_signed` | 10 | 100 | `i16`, `i32`, and `i64` function arguments |
| `scenario_02_unsigned` | 10 | 100 | `u8`, `u32`, and `u64` variables |
| `scenario_03_floats` | 10 | 100 | `f32`, `f64`, and `bool` variables |
| `scenario_04_strings` | 10 | 100 | `&str`, `&CStr`, and returned `&str` |
| `scenario_05_status` | 10 | 100 | `IRQL`, `NtStatus`, and returned `usize` |
| `scenario_06_enum_primary` | 10 | 100 | Explicit `u8`, sparse `u16`, and `u32` enums |
| `scenario_07_generic` | 10 | 100 | Three generic `IntoWppField` arguments |
| `scenario_08_enum_secondary` | 10 | 100 | Implicit and mixed-discriminant enums plus returned `i32` |
| `scenario_09_mixed` | 10 | 100 | `u8`, `i16`, and `usize` function arguments |

The fixture sources are reproducible with
`..\generate-trace-fixtures.ps1`. Build and package it with:

```powershell
cargo +nightly wdk build
```

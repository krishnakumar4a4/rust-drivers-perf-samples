# KMDF 100-trace fixture

This crate is the 10x WPP performance fixture. It repeats the baseline's ten
scenario families ten times, producing 100 unique trace statements split evenly
across 10 modules (10 statements per module). The first two modules run from
`driver_entry`; the remaining eight run from the device-add callback.

## Composition

- Two providers: `CoreProvider` and `DetailProvider`.
- Five keywords per provider and all five WPP levels, selected with a
  deterministic rotation.
- Exactly three fields in every trace statement.
- One literal-only module containing 10 statements (10% of the fixture).
- Ninety statements (90%) whose fields come from function arguments, local
  variables, enum values, generic arguments, or function return values.
- Every message and generated variable identifier includes its scenario and
  instance number, so all statements remain unique after scaling.

| Module | Statements | Field scenarios |
| --- | ---: | --- |
| `scenario_00_literals` | 10 | Direct `i8`, `u16`, and `bool` literals |
| `scenario_01_signed` | 10 | `i16`, `i32`, and `i64` function arguments |
| `scenario_02_unsigned` | 10 | `u8`, `u32`, and `u64` variables |
| `scenario_03_floats` | 10 | `f32`, `f64`, and `bool` variables |
| `scenario_04_strings` | 10 | `&str`, `&CStr`, and returned `&str` |
| `scenario_05_status` | 10 | `IRQL`, `NtStatus`, and returned `usize` |
| `scenario_06_enum_primary` | 10 | Explicit `u8`, sparse `u16`, and `u32` enums |
| `scenario_07_generic` | 10 | Three generic `IntoWppField` arguments |
| `scenario_08_enum_secondary` | 10 | Implicit and mixed-discriminant enums plus returned `i32` |
| `scenario_09_mixed` | 10 | `u8`, `i16`, and `usize` function arguments |

The fixture sources are reproducible with
`..\generate-trace-fixtures.ps1`. Build and package it with:

```powershell
cargo +nightly wdk build
```

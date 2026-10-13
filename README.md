# Rust driver build-performance samples

KMDF sample drivers that differ only in how much WPP tracing they contain. They are used to
measure how tracing affects compile time and binary size.

| Folder | Contents |
|---|---|
| `sample-kmdf-no-tracing` | Baseline driver without tracing |
| `sample-kmdf-with-traces`, `-with-10x-traces`, `-with-100x-traces` | 1x / 10x / 100x WPP trace calls |
| `sample-kmdf-with-dummy-traces`, `-with-10x-dummy-traces`, `-with-100x-dummy-traces` | Same volume of calls without real tracing |

The build scripts work on every top-level folder that has a `Cargo.toml`.

## Prerequisites

- The `stage1` rustup toolchain (`rustc +stage1`). It doesn't ship a cargo, so `cargo +stage1`
  uses rustup's fallback cargo, which must be a recent nightly.
- The measureme `summarize` tool on `PATH`, used by `aggregate-self-profile.py`:
  `cargo install --git https://github.com/rust-lang/measureme summarize`
- `cargo-wdk` and the WDK, if you build with `-WdkBuild`.
- Python 3.8 or later, for `aggregate-self-profile.py`.

Each driver's `.cargo\config.toml` sets:

```toml
[unstable]
build-dir-new-layout = false
```

This keeps cargo's classic `target\<profile>\deps` layout, which is where `cargo wdk build` looks
for the linker map file.

## build-self-profile.ps1

Builds each driver with `rustc -Zself-profile` and records the build's wall-clock time.

For each driver it:

1. Cleans. By default it runs `cargo clean` and deletes `<driver>\self-profile`. With
   `-Incremental` it runs `cargo clean -p <driver>` only, for the profile being built.
2. Builds with `-Zself-profile=<driver>\self-profile`, added to the rustflags from the driver's
   `.cargo\config.toml`. The default command is `cargo +stage1 build`; with `-WdkBuild` it's
   `cargo +stage1 wdk build`. Builds use the dev profile, or the release profile with `-Release`.
3. Writes the build time to `<driver>\self-profile\build-time.json`.

The per-crate profiles (`*.mm_profdata`) stay in `<driver>\self-profile` for
`aggregate-self-profile.py`.

| Parameter | Description |
|---|---|
| `-Drivers a,b` | Build only these driver folders. Default: all of them. |
| `-WdkBuild` | Build, package and sign with `cargo +stage1 wdk build`. Default: compile and link with `cargo +stage1 build`. |
| `-Release` | Build with the release profile (`--profile release`; the drivers set `lto = true`). Output goes to `target\release`. Default: dev profile, `target\debug`. |
| `-Incremental` | Rebuild only the driver crate and reuse the dependencies from the last full build. Keeps the dependency profiles and replaces only the driver crate's profile. The last full build must have used the same profile (dev or `-Release`); otherwise the script stops. |
| `-MeasureUnprofiled` | Before the profiled build, time a separate clean build without `-Zself-profile`. Can't be combined with `-Incremental`. |
| `-Toolchain <name>` | rustup toolchain. Default: `stage1`. |
| `-CargoArgs @(...)` | Extra arguments for the cargo build command, e.g. `@('--target-arch','arm64')` with `-WdkBuild`. Use `-Release` rather than `--release`/`--profile`. |
| `-SelfProfileEvents <list>` | Value for `-Zself-profile-events`, e.g. `default,args`. |

```powershell
.\build-self-profile.ps1                                           # full build of every driver
.\build-self-profile.ps1 -WdkBuild                                 # same, with cargo wdk build
.\build-self-profile.ps1 -Release                                  # full release build of every driver
.\build-self-profile.ps1 -Drivers sample-kmdf-no-tracing           # one driver
.\build-self-profile.ps1 -Drivers sample-kmdf-no-tracing -Incremental   # driver crate only
.\build-self-profile.ps1 -MeasureUnprofiled                        # also time an unprofiled build
```

### build-time.json

Each successful build writes `<driver>\self-profile\build-time.json`:

- A full build creates a new file with `profile` (`dev` or `release`), `full_build_seconds`,
  `full_build_command` and `full_build_timestamp`. With `-MeasureUnprofiled` it also has
  `unprofiled_full_build_seconds`.
- `-Incremental` keeps the full-build values and adds or replaces `incremental_build_seconds`,
  `incremental_build_command` and `incremental_build_timestamp`.

The file holds only the latest run. To keep a history, use `run-build-experiment.ps1`, which
appends every run to a CSV file.

> **Note:** `-Incremental` assumes the previous full build used the same toolchain, command and
> flags. If it didn't, cargo rebuilds every dependency, and that time is recorded as a
> driver-only build. Do a full build first.

## run-build-experiment.ps1

Runs a fixed build-time experiment for each driver by calling `build-self-profile.ps1` four times
in a row:

1. Full build 1
2. Full build 2
3. Driver-only build 1 (`-Incremental`)
4. Driver-only build 2 (`-Incremental`)

It prints the four times in that order for each driver, and appends one row per driver to
`build-experiment-results.csv` as soon as that driver finishes. The CSV columns are `Timestamp`,
`Driver`, `Command`, `Profile`, `CargoArgs`, `FullBuild1_s`, `FullBuild2_s`, `DriverOnlyBuild1_s`
and `DriverOnlyBuild2_s`. If the CSV file already exists with different columns (for example
one written before the `Profile` column was added), the script stops; pass a new `-ResultsCsv`.

Drivers without dummy traces run first (no-tracing, then 1x, 10x and 100x traces), followed by
the `*-dummy-traces` drivers.

| Parameter | Description |
|---|---|
| `-Drivers a,b` | Run only for these driver folders. Default: all of them. |
| `-DriverSet All\|NonDummy\|Dummy` | Run all drivers, only the ones without dummy traces, or only the `*-dummy-traces` ones. Default: `All`. |
| `-WdkBuild` | Run the same experiment with `cargo +stage1 wdk build`. |
| `-Release` | Run the same experiment with the release profile. |
| `-Toolchain <name>` | rustup toolchain. Default: `stage1`. |
| `-CargoArgs @(...)` | Extra cargo arguments, passed through. |
| `-ResultsCsv <file>` | CSV file the results are appended to. Default: `build-experiment-results.csv`. |

```powershell
.\run-build-experiment.ps1                                         # cargo build, all drivers
.\run-build-experiment.ps1 -WdkBuild                               # cargo wdk build, all drivers
.\run-build-experiment.ps1 -Release -ResultsCsv build-experiment-release.csv   # release profile
.\run-build-experiment.ps1 -DriverSet NonDummy                     # only drivers without dummy traces
.\run-build-experiment.ps1 -Drivers sample-kmdf-no-tracing,sample-kmdf-with-traces
```

A full build takes about 4–5 minutes per driver, so one experiment takes about 10 minutes per
driver.

Example output:

```text
Build times (cargo +stage1 build)

Driver                 Full build 1     Full build 2     Driver-only build 1 Driver-only build 2
------                 ------------     ------------     ------------------- -------------------
sample-kmdf-no-tracing   278.9s (04:38)   261.8s (04:21)     9.3s (00:09)        8.6s (00:08)
```

For steadier numbers, stop other builds from running at the same time, for example
rust-analyzer's check-on-save.

## aggregate-self-profile.py

Summarizes the `-Zself-profile` data and `build-time.json` into tables with one column per
driver. It shows build wall-clock times, time per compiler phase (whole build and driver crate
only), time per crate group, the most expensive crates, and driver binary sizes.

```powershell
python aggregate-self-profile.py                                   # print the tables
python aggregate-self-profile.py --csv self-profile-summary.csv    # also write a CSV
python aggregate-self-profile.py --labels                          # show how compiler steps map to phases
python aggregate-self-profile.py --clean-profiles --clean-targets  # delete profile data and target\
```

The tables, except the wall-clock one, show compiler CPU time summed across all rustc processes.
Builds run many compilers in parallel, so these totals are larger than the wall-clock time.

The script prints the cargo profile of each driver's last build (from `build-time.json`) and
warns if the drivers were built with different profiles, since dev and release numbers aren't
comparable. The binary size is read from that profile's folder (`target\debug` or
`target\release`).

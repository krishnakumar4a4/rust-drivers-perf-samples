"""High-level aggregation of rustc -Zself-profile data.

For every driver folder (sample-kmdf-*) this script runs `summarize summarize --json`
on each .mm_profdata file in <driver>\\self-profile, caches the JSON output in
<driver>\\self-profile\\summaries, and rolls the per-query data up into:

  * compiler phases (frontend / type-check & analysis / MIR / codegen / LLVM / linking / ...)
  * crate groups (driver crate / WDK crates / third-party dependencies)
  * per-crate totals (top N)
  * artifact sizes for the driver crate (final .sys, object files, ...)

and prints a side-by-side comparison of all drivers, ordered by trace volume
(no-tracing, with-traces, 10x, 100x). Each dummy-trace baseline (same call sites and
arguments, empty callee instead of WPP) is placed directly before its traced driver.

Times are summed self-times (CPU time). LLVM work runs on multiple threads, so
the totals can be larger than the wall-clock build time. Wall-clock build times are
read from <driver>\\self-profile\\build-time.json, written by build-self-profile.ps1.

Usage:
    python aggregate-self-profile.py [--top N] [--csv out.csv] [--labels] [driver-dir ...]
    python aggregate-self-profile.py --clean-profiles [--clean-targets] [driver-dir ...]
"""

import argparse
import concurrent.futures
import csv
import json
import os
import re
import shutil
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DEFAULT_OUTPUT_DIR = "sample-kmdf-mm_profdata"  # build-self-profile.ps1 -OutputDir default

# Ordered: first matching rule wins. Patterns are regexes matched against the event label.
# Run with --labels to see which raw labels land in which phase (and what ends up in "Other").
PHASE_RULES = [
    ("Linking", r"^(run_linker|link_)"),
    ("Incremental / dep-graph", r"^(incr_comp_|copy_all_cgu_workproducts|serialize_|"
                                r"query_key_hash_verify|verify_query_key_hashes|drop_dep_graph|"
                                r"encode_query_results|save_dep_graph|finish_previous_session)"),
    ("Metadata (decode/encode)", r"^(metadata_|encode_|write_crate_metadata|generate_crate_metadata|"
                                 r"crate_metadata|crate_hash|used_crate|crate_extern_paths|"
                                 r"visible_parent_map|dependency_formats)"),
    ("LLVM (optimize + emit)", r"^(LLVM_|llvm_|codegen_module_optimize|codegen_module_perform_lto|"
                               r"codegen_copy_artifacts|codegen_run_passes|codegen_emit|"
                               r"finish_ongoing_codegen|join_worker_thread|"
                               r"target_machine_factory|thin_lto|lto_)"),
    ("Codegen (mono + LLVM IR gen)", r"^(codegen_|collect_and_partition_mono_items|monomorphization_|"
                                     r"partition_|cgu_partitioning|assert_symbols_are_distinct|symbol_name|"
                                     r"exported_|reachable_|upstream_|fn_abi_of|layout_of|"
                                     r"mir_shims|items_of_instance|size_estimate|"
                                     r"check_mono_item|resolve_instance|vtable_|first_method_vtable|"
                                     r"compute_debuginfo|sanitizer_settings|"
                                     r"unused_generic_params|is_reachable_non_generic|"
                                     r"cross_crate_inlinable|should_codegen)"),
    ("Parse, expand, resolve, lower", r"^(parse_crate|expand_|macro_expand|pre_AST_expansion|"
                                      r"AST_validation|configure_and_expand|crate_injection|"
                                      r"complete_gated_feature_checking|early_lint_checks|"
                                      r"finalize_|resolver_for_lowering|late_resolve|resolutions|"
                                      r"resolve_postprocess|resolve_check_unused|maybe_create_a_macro_crate|"
                                      r"registered_|index_ast|lower_to_hir|hir_|opt_hir_|"
                                      r"emit_ast_lowering|cfg_eval|derive_)"),
    ("MIR build, borrowck, const-eval", r"^(mir_|optimized_mir|promoted_mir|borrowck|thir_|"
                                        r"check_unsafety|check_match|check_liveness|"
                                        r"eval_|const_eval|trivial_const|try_destructure|valtree|"
                                        r"lit_to_const|has_ffi_unwind_calls|nested_bodies_within|"
                                        r"closure_|coroutine_|run_pass|MIR_)"),
    ("Lints & privacy", r"^(lint_|late_lint|module_lints|check_mod_privacy|check_mod_deathness|"
                        r"live_symbols|effective_visibilities|compute_effective_visibilities|"
                        r"privacy_|stability_|check_mod_unstable|lib_features|"
                        r"skippable_lints|shallow_lint|missing_docs|check_expectations)"),
    ("Type check & trait solving", r"^(typeck|type_|check_|evaluate_|predicates_|explicit_|inferred_|"
                                   r"impl_|trait_|specializ|coherent_|coherence|adt_|variances_|"
                                   r"crate_variances|generics_of|fn_sig|param_env|typing_env|"
                                   r"normalize|try_normalize|erase_|dropck|needs_drop|is_copy|"
                                   r"is_sized|is_freeze|is_unpin|has_significant_drop|associated_|"
                                   r"opaque_|wf_|assumed_wf|implied_outlives|super_predicates|"
                                   r"item_|const_param|constness|defaultness|region_|"
                                   r"resolve_bound_vars|late_bound|object_lifetime|named_variable|"
                                   r"in_scope_traits|lookup_|all_local_trait_impls|local_trait_impls|"
                                   r"implementations_of_trait|crate_inherent_impls|inherent_impls|"
                                   r"incoherent_impls|orphan_check|enforce_impl|method_autoderef|"
                                   r"clauses_of|dyn_compatibility|compare_impl|collect_mod_item|"
                                   r"own_existential|representability|params_in_repr|"
                                   r"upvars|adjust_|reveal_|instantiate_|inhabited_|anon_const)"),
    ("Session setup & misc", r"^(free_global_ctxt|drop_ast|drop_compiler|prepare_outputs|"
                             r"write_dep_info|create_global_ctxt|setup_global_ctxt|output_filenames|"
                             r"analysis|misc_checking|self_profile_alloc_query_strings)"),
]
_COMPILED = [(name, re.compile(pat)) for name, pat in PHASE_RULES]
OTHER = "Other queries"
PHASE_ORDER = [name for name, _ in PHASE_RULES] + [OTHER]

WDK_CRATE_PREFIXES = ("wdk", "wdf", "wpp")


def classify_label(label: str) -> str:
    for name, rx in _COMPILED:
        if rx.match(label):
            return name
    return OTHER


def crate_group(crate: str, driver_crate: str) -> str:
    if crate == driver_crate:
        return "Driver crate"
    if crate.startswith(WDK_CRATE_PREFIXES):
        return "WDK crates (wdk*/wdf*/wpp*)"
    if crate.startswith("build_script_"):
        return "Build scripts"
    return "Third-party deps"


def dur(d) -> float:
    return d["secs"] + d["nanos"] / 1e9


def summarize_file(profdata: Path, summarize_exe: str) -> dict:
    out_dir = profdata.parent / "summaries"
    out_dir.mkdir(exist_ok=True)
    cached = out_dir / (profdata.stem + ".json")
    if cached.exists() and cached.stat().st_mtime >= profdata.stat().st_mtime:
        return json.loads(cached.read_text())
    prefix = profdata.with_suffix("")
    proc = subprocess.run([summarize_exe, "summarize", "--json", str(prefix)],
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    produced = prefix.with_suffix(".json")
    if proc.returncode != 0 or not produced.exists():
        # Usually a truncated profile from an interrupted build.
        return None
    os.replace(produced, cached)
    return json.loads(cached.read_text())


def crate_name_of(profdata: Path) -> str:
    # <crate_name>-<pid>.mm_profdata
    return profdata.stem.rsplit("-", 1)[0]


def trace_order(driver_dir: Path):
    """Sort key: no-tracing, then 1x, 10x, 100x trace volume, then anything else.

    Within a trace volume the dummy-trace baseline sorts directly before the traced driver.
    """
    name = driver_dir.name
    if "no-tracing" in name:
        multiplier = 0
    elif m := re.search(r"(\d+)x-(?:dummy-)?traces", name):
        multiplier = int(m.group(1))
    elif "with-traces" in name or "with-dummy-traces" in name:
        multiplier = 1
    else:
        multiplier = float("inf")
    return (multiplier, "dummy-traces" not in name, name)


def column_label(driver_name: str) -> str:
    """Table column header: no_tracing, dummy_traces, with_traces, 10x_dummy_traces, 10x_traces, ..."""
    if "no-tracing" in driver_name:
        return "no_tracing"
    if m := re.search(r"(\d+)x-(dummy-)?traces", driver_name):
        return f"{m.group(1)}x_{'dummy_' if m.group(2) else ''}traces"
    if "with-dummy-traces" in driver_name:
        return "dummy_traces"
    if "with-traces" in driver_name:
        return "with_traces"
    return driver_name.replace("sample-kmdf-", "")

def discover_drivers(explicit):
    if explicit:
        dirs = [Path(d).resolve() for d in explicit]
        missing = [d for d in dirs if not (d / "Cargo.toml").is_file()]
        if missing:
            raise SystemExit(f"Not a driver folder (no Cargo.toml): {', '.join(map(str, missing))}")
    else:
        dirs = [p for p in ROOT.iterdir() if p.is_dir() and (p / "Cargo.toml").is_file()]
    return sorted(dirs, key=trace_order)


def package_crate_name(driver_dir: Path) -> str:
    in_package = False
    for line in (driver_dir / "Cargo.toml").read_text().splitlines():
        if m := re.match(r"^\s*\[(.+)\]\s*$", line):
            in_package = m.group(1).strip() == "package"
        elif in_package and (m := re.match(r'^\s*name\s*=\s*"([^"]+)"', line)):
            return m.group(1).replace("-", "_")
    return driver_dir.name.replace("-", "_")


def clean_profiles(driver_dirs, output_dir: Path):
    for d in driver_dirs:
        prof_dir = d / "self-profile"
        if prof_dir.exists():
            print(f"Removing {prof_dir}", flush=True)
            shutil.rmtree(prof_dir)
        if output_dir.is_dir():
            crate = package_crate_name(d)
            # Only the files build-self-profile.ps1 generates; leave anything else in the folder alone.
            for f in [*output_dir.glob(f"{crate}-*.mm_profdata"), output_dir / f"{crate}.json"]:
                if f.is_file():
                    print(f"Removing {f}", flush=True)
                    f.unlink()


def clean_targets(driver_dirs, toolchain: str):
    for d in driver_dirs:
        print(f"cargo +{toolchain} clean  ({d.name})", flush=True)
        subprocess.run(["cargo", f"+{toolchain}", "clean"], cwd=d, check=True)


# Keys written by build-self-profile.ps1 -> row label.
WALL_CLOCK_ROWS = [
    ("full_build_seconds", "Full build, profiled"),
    ("unprofiled_full_build_seconds", "Full build, unprofiled"),
    ("incremental_build_seconds", "Driver-only rebuild, profiled"),
]


def read_build_times(prof_dir: Path) -> dict:
    f = prof_dir / "build-time.json"
    if not f.is_file():
        return {}
    data = json.loads(f.read_text(encoding="utf-8-sig"))
    return {k: float(data[k]) for k, _ in WALL_CLOCK_ROWS if data.get(k) is not None}


def analyze_driver(driver_dir: Path, summarize_exe: str, jobs: int, label_totals: dict):
    prof_dir = driver_dir / "self-profile"
    # `___` is cargo's rustc target-info probe (one per cargo invocation), not a crate compile.
    files = sorted(f for f in prof_dir.glob("*.mm_profdata") if crate_name_of(f) != "___")
    if not files:
        print(f"Skipping {driver_dir.name}: no .mm_profdata files in {prof_dir}", file=sys.stderr)
        return None
    driver_crate = package_crate_name(driver_dir)

    with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
        results = list(pool.map(lambda f: (f, summarize_file(f, summarize_exe)), files))
    unreadable = [f.name for f, data in results if data is None]
    if unreadable:
        print(f"Warning: {driver_dir.name}: skipped {len(unreadable)} unreadable profile(s) "
              f"(interrupted build?): {', '.join(unreadable)}", file=sys.stderr)
    results = [(f, data) for f, data in results if data is not None]

    phases = defaultdict(float)
    driver_phases = defaultdict(float)
    groups = defaultdict(float)
    crates = defaultdict(float)
    driver_artifacts = {}
    for f, data in results:
        crate = crate_name_of(f)
        total = 0.0
        for q in data["query_data"]:
            st = dur(q["self_time"])
            ph = classify_label(q["label"])
            phases[ph] += st
            if crate == driver_crate:
                driver_phases[ph] += st
            label_totals[q["label"]] += st
            total += st
        crates[crate] += total
        groups[crate_group(crate, driver_crate)] += total
        if crate == driver_crate:
            driver_artifacts = {a["label"]: a["value"] for a in data.get("artifact_sizes", [])}

    sys_file = next(driver_dir.glob(f"target/**/{driver_crate}.sys"), None)
    return {
        "name": driver_dir.name,
        "files": len(files),
        "total": sum(phases.values()),
        "phases": phases,
        "driver_phases": driver_phases,
        "groups": groups,
        "crates": crates,
        "artifacts": driver_artifacts,
        "sys_size": sys_file.stat().st_size if sys_file else None,
        "wall_clock": read_build_times(prof_dir),
    }


def fmt_s(x: float) -> str:
    return f"{x:8.2f}s"


def fmt_wall(x: float) -> str:
    m, s = divmod(x, 60)
    return f"{x:7.1f}s ({int(m)}m{s:04.1f}s)"


def print_table(title, row_keys, drivers, getter, fmt=fmt_s, with_pct=True):
    names = [column_label(d["name"]) for d in drivers]
    w0 = max(len(title), *(len(k) for k in row_keys)) + 2
    colw = max(18, *(len(n) + 2 for n in names))
    print()
    print(title.ljust(w0) + "".join(n.rjust(colw) for n in names))
    print("-" * (w0 + colw * len(names)))
    for k in row_keys:
        cells = []
        for d in drivers:
            v = getter(d, k)
            if v is None:
                cells.append("-".rjust(colw))
                continue
            s = fmt(v)
            if with_pct:
                tot = getter(d, None)
                if tot:
                    s += f" ({100 * v / tot:4.1f}%)"
            cells.append(s.rjust(colw))
        print(k.ljust(w0) + "".join(cells))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("drivers", nargs="*", help="driver folders (default: sample-kmdf-*)")
    ap.add_argument("--top", type=int, default=10, help="number of most expensive crates to list")
    ap.add_argument("--csv", help="also write a long-format CSV with all aggregates")
    ap.add_argument("--labels", action="store_true",
                    help="print the most expensive raw labels with their assigned phase (to tune PHASE_RULES)")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--summarize", default="summarize", help="path to measureme summarize")
    ap.add_argument("--clean-profiles", action="store_true",
                    help="delete <driver>\\self-profile (profiles, cached summaries, build-time.json) and the "
                         "driver's .mm_profdata/.json in --output-dir, then exit")
    ap.add_argument("--clean-targets", action="store_true",
                    help="run `cargo clean` in each driver folder (removes target\\ artifacts), then exit")
    ap.add_argument("--output-dir", default=DEFAULT_OUTPUT_DIR,
                    help=f"build-self-profile.ps1 output folder cleaned by --clean-profiles (default: {DEFAULT_OUTPUT_DIR})")
    ap.add_argument("--toolchain", default="stage1", help="rustup toolchain for --clean-targets (default: stage1)")
    args = ap.parse_args()

    dirs = discover_drivers(args.drivers)

    if args.clean_profiles or args.clean_targets:
        if args.clean_profiles:
            out = Path(args.output_dir)
            clean_profiles(dirs, out if out.is_absolute() else ROOT / out)
        if args.clean_targets:
            clean_targets(dirs, args.toolchain)
        return

    label_totals = defaultdict(float)
    drivers = []
    for d in dirs:
        print(f"Analyzing {d.name} ...", file=sys.stderr)
        result = analyze_driver(d, args.summarize, args.jobs, label_totals)
        if result:
            drivers.append(result)
    if not drivers:
        raise SystemExit("No profile data found. Run build-self-profile.ps1 first.")

    wall_rows = [(k, label) for k, label in WALL_CLOCK_ROWS if any(k in d["wall_clock"] for d in drivers)]
    if wall_rows:
        label_to_key = {label: k for k, label in wall_rows}
        print_table("Build wall-clock time (cargo wdk build)", list(label_to_key), drivers,
                    lambda d, k: None if k is None else d["wall_clock"].get(label_to_key[k]),
                    fmt=fmt_wall, with_pct=False)
    else:
        print("\nNo build-time.json found; run build-self-profile.ps1 to record wall-clock build times.")

    print("\nAll times below are summed self-time (CPU time) reported by rustc -Zself-profile.")

    print_table("Compiler phase (whole build)", PHASE_ORDER, drivers,
                lambda d, k: d["total"] if k is None else d["phases"].get(k, 0.0))
    print_table("Crate group", sorted({g for d in drivers for g in d["groups"]}), drivers,
                lambda d, k: d["total"] if k is None else d["groups"].get(k))
    print_table("Compiler phase (driver crate only)", PHASE_ORDER, drivers,
                lambda d, k: sum(d["driver_phases"].values()) if k is None else d["driver_phases"].get(k, 0.0))

    agg = defaultdict(float)
    for d in drivers:
        for c, t in d["crates"].items():
            agg[c] = max(agg[c], t)
    top = sorted(agg, key=agg.get, reverse=True)[: args.top]
    print_table(f"Top {args.top} crates by compile time", top, drivers,
                lambda d, k: d["total"] if k is None else d["crates"].get(k), with_pct=False)

    art_keys = sorted({k for d in drivers for k in d["artifacts"]})
    rows = art_keys + ["(final .sys on disk)"]
    print_table("Driver crate artifact sizes (bytes)", rows, drivers,
                lambda d, k: None if k is None else (d["sys_size"] if k == "(final .sys on disk)" else d["artifacts"].get(k)),
                fmt=lambda v: f"{v:,}", with_pct=False)

    print()
    for d in drivers:
        line = f"{d['name']}: {d['files']} rustc invocations, {d['total']:.2f}s total compiler CPU time"
        wall = d["wall_clock"].get("full_build_seconds")
        if wall:
            line += f", {wall:.1f}s wall-clock (full profiled build, {d['total'] / wall:.1f}x parallelism)"
        print(line)

    if args.labels:
        print("\nMost expensive raw labels (all drivers):")
        for lbl, t in sorted(label_totals.items(), key=lambda kv: kv[1], reverse=True)[:60]:
            print(f"  {t:9.2f}s  {classify_label(lbl):32}  {lbl}")
        other = [(l, t) for l, t in label_totals.items() if classify_label(l) == OTHER]
        if other:
            print(f"\nLabels in '{OTHER}' ({len(other)}, {sum(t for _, t in other):.2f}s):")
            for lbl, t in sorted(other, key=lambda kv: kv[1], reverse=True)[:40]:
                print(f"  {t:9.3f}s  {lbl}")

    if args.csv:
        with open(args.csv, "w", newline="") as fh:
            w = csv.writer(fh)
            w.writerow(["driver", "dimension", "key", "value"])
            for d in drivers:
                for k, v in d["wall_clock"].items():
                    w.writerow([d["name"], "wall_clock_seconds", k, f"{v:.3f}"])
                for k, v in d["phases"].items():
                    w.writerow([d["name"], "phase_seconds", k, f"{v:.6f}"])
                for k, v in d["driver_phases"].items():
                    w.writerow([d["name"], "driver_crate_phase_seconds", k, f"{v:.6f}"])
                for k, v in d["groups"].items():
                    w.writerow([d["name"], "crate_group_seconds", k, f"{v:.6f}"])
                for k, v in d["crates"].items():
                    w.writerow([d["name"], "crate_seconds", k, f"{v:.6f}"])
                for k, v in d["artifacts"].items():
                    w.writerow([d["name"], "driver_artifact_bytes", k, v])
                if d["sys_size"] is not None:
                    w.writerow([d["name"], "driver_artifact_bytes", "sys_file", d["sys_size"]])
        print(f"\nWrote {args.csv}")


if __name__ == "__main__":
    main()

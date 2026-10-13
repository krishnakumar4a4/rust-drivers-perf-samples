<#
.SYNOPSIS
    Builds every driver crate with rustc -Zself-profile and collects the driver crate's
    profile data (.mm_profdata + crox Chrome trace) into a single top-level folder.

.DESCRIPTION
    For each driver folder (a top-level folder containing a Cargo.toml):
      1. Cleans the previous build (full `cargo clean` by default, or only the driver
         crate with -Incremental) and the matching profile data in <driver>\self-profile.
      2. Runs `cargo +<toolchain> build` (or `cargo +<toolchain> wdk build` with -WdkBuild) with
         `-Zself-profile=<driver>\self-profile` added to the rustflags from the driver's
         .cargo\config.toml.
      3. Runs `crox` on the driver crate's .mm_profdata and writes the Chrome trace as
         <OutputDir>\<crate_name>.json.
      4. Copies the driver crate's .mm_profdata to <OutputDir>.
      5. Records the wall-clock time of the build(s) in <driver>\self-profile\build-time.json.

    With -MeasureUnprofiled, a separate clean build without -Zself-profile is timed first,
    so the real build time can be compared with the profiled one (profiling adds overhead).

    The full set of per-crate profiles and build-time.json stay in <driver>\self-profile
    for aggregate-self-profile.py.

.EXAMPLE
    .\build-self-profile.ps1

.EXAMPLE
    .\build-self-profile.ps1 -Drivers sample-kmdf-no-tracing,sample-kmdf-with-traces -Incremental

.EXAMPLE
    .\build-self-profile.ps1 -SkipBuild    # only re-run crox / copy on existing profile data

.EXAMPLE
    .\build-self-profile.ps1 -MeasureUnprofiled    # also time a clean build without -Zself-profile

.EXAMPLE
    .\build-self-profile.ps1 -WdkBuild    # build + package/sign with `cargo wdk build`
#>
[CmdletBinding()]
param(
    # Driver folder names to build (default: every top-level folder with a Cargo.toml).
    [string[]] $Drivers,

    # rustup toolchain used for the build.
    [string] $Toolchain = "stage1",

    # Folder (relative to this script or absolute) that receives the .mm_profdata and .json files.
    [string] $OutputDir = "sample-kmdf-mm_profdata",

    # Only rebuild the driver crate (cargo clean -p <crate>) and replace just its profile in
    # <driver>\self-profile, keeping the dependency profiles from the last full build.
    [switch] $Incremental,

    # Skip cleaning and building; just run crox and copy existing driver crate profile data.
    [switch] $SkipBuild,

    # Before the profiled build, time a separate clean build without -Zself-profile.
    # Not supported with -Incremental (changing RUSTFLAGS would rebuild every dependency).
    [switch] $MeasureUnprofiled,

    # Build with `cargo +<toolchain> wdk build` (compile + package/sign) instead of the default
    # `cargo +<toolchain> build` (compile + link only). cargo-wdk reads the linker map from
    # target\<profile>\deps, so the driver's .cargo\config.toml keeps the classic build-dir layout
    # ([unstable] build-dir-new-layout = false).
    [switch] $WdkBuild,

    # Optional value for -Zself-profile-events (e.g. "default,args").
    [string] $SelfProfileEvents,

    # Optional crox --minimum-duration filter in microseconds.
    [int] $CroxMinimumDuration = 0,

    # Extra arguments passed to `cargo build` (e.g. --profile release), or to `cargo wdk build`
    # with -WdkBuild (e.g. --profile release, --target-arch arm64).
    [string[]] $CargoArgs = @()
)

$ErrorActionPreference = "Stop"

function Get-CrateName([string] $manifest) {
    $inPackage = $false
    foreach ($line in Get-Content $manifest) {
        if ($line -match '^\s*\[(.+)\]\s*$') { $inPackage = $Matches[1].Trim() -eq "package"; continue }
        if ($inPackage -and $line -match '^\s*name\s*=\s*"([^"]+)"') { return $Matches[1] }
    }
    throw "Could not find [package] name in $manifest"
}

# Returns the build.rustflags array from <driver>\.cargo\config.toml, because RUSTFLAGS overrides it.
function Get-ConfigRustflags([string] $driverDir) {
    $config = Join-Path $driverDir ".cargo\config.toml"
    if (-not (Test-Path $config)) { return @() }
    $text = Get-Content $config -Raw
    if ($text -notmatch '(?s)\[build\][^\[]*?rustflags\s*=\s*\[(.*?)\]') { return @() }
    return [regex]::Matches($Matches[1], '"([^"]*)"') | ForEach-Object { $_.Groups[1].Value }
}

# The cargo subcommand for the build: `cargo build` by default, `cargo wdk build` with -WdkBuild.
function Get-BuildCommand { if ($WdkBuild) { "wdk build" } else { "build" } }

# Times one driver build (run from the driver folder).
function Invoke-TimedBuild([string] $crate) {
    $sw = [Diagnostics.Stopwatch]::StartNew()
    if ($WdkBuild) {
        & cargo "+$Toolchain" wdk build @CargoArgs | Out-Host
    } else {
        & cargo "+$Toolchain" build @CargoArgs | Out-Host
    }
    $exit = $LASTEXITCODE
    $sw.Stop()
    if ($exit -ne 0) { throw "cargo $(Get-BuildCommand) failed for $crate (exit $exit)" }
    return $sw.Elapsed
}

# After `cargo clean`, Windows can leave the deleted target\ folder "delete pending" while another process
# (rust-analyzer, Defender, the search indexer) still has a handle open; creating it again then fails with
# "Access is denied". Wait (outside the timed build) until target\ can be created.
function Wait-TargetDir([string] $driverDir) {
    $target = Join-Path $driverDir "target"
    for ($i = 1; $i -le 60; $i++) {
        try { New-Item -ItemType Directory -Force $target -ErrorAction Stop | Out-Null; return }
        catch {
            if ($i -eq 1) { Write-Host "Waiting for $target to become available ..." -ForegroundColor Yellow }
            Start-Sleep -Seconds 2
        }
    }
    throw "Could not create $target (still locked after 2 minutes)"
}

# Records build wall-clock times in <driver>\self-profile\build-time.json. A full build replaces the
# file; a driver-only (-Incremental) rebuild keeps the last full build's numbers and adds its own.
function Save-BuildTime([string] $profileDir, [string] $driverName, [string] $crate, [bool] $incremental,
                        [TimeSpan] $profiled, $unprofiled) {
    $file = Join-Path $profileDir "build-time.json"
    $data = [ordered]@{}
    if ($incremental -and (Test-Path $file)) {
        (Get-Content $file -Raw | ConvertFrom-Json).PSObject.Properties | ForEach-Object { $data[$_.Name] = $_.Value }
    }
    $data["driver"] = $driverName
    $data["crate"] = $crate
    $data["toolchain"] = $Toolchain
    $data["cargo_args"] = @($CargoArgs)
    $now = (Get-Date).ToString("o")
    $command = "cargo +$Toolchain $(Get-BuildCommand)"
    if ($incremental) {
        $data["incremental_build_seconds"] = [math]::Round($profiled.TotalSeconds, 3)
        $data["incremental_build_command"] = $command
        $data["incremental_build_timestamp"] = $now
    } else {
        $data["full_build_seconds"] = [math]::Round($profiled.TotalSeconds, 3)
        $data["full_build_command"] = $command
        $data["full_build_timestamp"] = $now
        if ($unprofiled) { $data["unprofiled_full_build_seconds"] = [math]::Round($unprofiled.TotalSeconds, 3) }
    }
    $data | ConvertTo-Json | Set-Content -Encoding utf8 $file
}

if ($MeasureUnprofiled -and $Incremental) { throw "-MeasureUnprofiled cannot be combined with -Incremental." }
if ($MeasureUnprofiled -and $SkipBuild) { throw "-MeasureUnprofiled cannot be combined with -SkipBuild." }

foreach ($tool in @("cargo", "crox")) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) { throw "'$tool' was not found on PATH." }
}

if (-not [System.IO.Path]::IsPathRooted($OutputDir)) { $OutputDir = Join-Path $PSScriptRoot $OutputDir }
New-Item -ItemType Directory -Force $OutputDir | Out-Null

$driverDirs = Get-ChildItem -Directory $PSScriptRoot | Where-Object { Test-Path (Join-Path $_.FullName "Cargo.toml") }
if ($Drivers) {
    $unknown = $Drivers | Where-Object { $_ -notin $driverDirs.Name }
    if ($unknown) { throw "Unknown driver folder(s): $($unknown -join ', ')" }
    $driverDirs = $driverDirs | Where-Object { $_.Name -in $Drivers }
}

$results = @()
$savedRustflags = $env:RUSTFLAGS
try {
    foreach ($driver in $driverDirs) {
        $crate = Get-CrateName (Join-Path $driver.FullName "Cargo.toml")
        $crateFile = $crate.Replace("-", "_")
        $profileDir = Join-Path $driver.FullName "self-profile"
        Write-Host "`n===== $($driver.Name) ($crateFile) =====" -ForegroundColor Cyan

        $buildTime = $null
        $unprofiledTime = $null
        if (-not $SkipBuild) {
            Push-Location $driver.FullName
            try {
                if ($Incremental) {
                    & cargo "+$Toolchain" clean -p $crate
                } else {
                    & cargo "+$Toolchain" clean
                    Remove-Item -Recurse -Force $profileDir -ErrorAction SilentlyContinue
                }
                if ($LASTEXITCODE -ne 0) { throw "cargo clean failed for $($driver.Name)" }
                New-Item -ItemType Directory -Force $profileDir | Out-Null
                Wait-TargetDir $driver.FullName

                if ($MeasureUnprofiled) {
                    # Separate wall-clock step: a clean build with only the config rustflags (no
                    # -Zself-profile), then clean again so the profiled build also starts from scratch.
                    $env:RUSTFLAGS = @(Get-ConfigRustflags $driver.FullName) -join " "
                    Write-Host "Unprofiled build for wall-clock time: RUSTFLAGS=$env:RUSTFLAGS" -ForegroundColor Yellow
                    $unprofiledTime = Invoke-TimedBuild $crate
                    Write-Host "Unprofiled build took $($unprofiledTime.ToString('hh\:mm\:ss\.fff'))" -ForegroundColor Yellow
                    & cargo "+$Toolchain" clean
                    if ($LASTEXITCODE -ne 0) { throw "cargo clean failed for $($driver.Name)" }
                    Wait-TargetDir $driver.FullName
                }

                $existing = @(Get-ChildItem $profileDir -Filter *.mm_profdata | ForEach-Object Name)

                # The profile path must stay the same across runs: RUSTFLAGS is part of cargo's
                # fingerprint, so changing it would rebuild every dependency.
                $flags = @(Get-ConfigRustflags $driver.FullName) + "-Zself-profile=$profileDir"
                if ($SelfProfileEvents) { $flags += "-Zself-profile-events=$SelfProfileEvents" }
                $env:RUSTFLAGS = $flags -join " "
                Write-Host "RUSTFLAGS=$env:RUSTFLAGS"

                $buildTime = Invoke-TimedBuild $crate

                if ($Incremental) {
                    # The rebuild also re-runs the driver's build script and cargo's `___` probe, and
                    # profile names aren't unique per crate (build_script_build, host/target copies).
                    # Keep only the new driver crate profile; drop its old one and every other new file
                    # so aggregate-self-profile.py doesn't double count.
                    $added = Get-ChildItem $profileDir -Filter *.mm_profdata | Where-Object Name -notin $existing
                    $new = $added | Where-Object Name -like "$crateFile-*" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
                    if (-not $new) { throw "Build did not produce $crateFile-*.mm_profdata" }
                    $stale = @(Get-ChildItem $profileDir -Filter "$crateFile-*.mm_profdata" | Where-Object Name -ne $new.Name) +
                             @($added | Where-Object Name -ne $new.Name)
                    foreach ($f in $stale) {
                        Remove-Item (Join-Path $profileDir "summaries\$($f.BaseName).json") -ErrorAction SilentlyContinue
                        Remove-Item -Force $f.FullName
                    }
                }

                Save-BuildTime $profileDir $driver.Name $crate ([bool]$Incremental) $buildTime $unprofiledTime
            } finally {
                $env:RUSTFLAGS = $savedRustflags
                Pop-Location
            }
        }

        $profdata = Get-ChildItem $profileDir -Filter "$crateFile-*.mm_profdata" -ErrorAction SilentlyContinue |
            Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if (-not $profdata) { throw "No $crateFile-*.mm_profdata found in $profileDir" }

        # crox always writes chrome_profiler.json to the current directory, so run it in a scratch folder.
        $scratch = Join-Path ([System.IO.Path]::GetTempPath()) "crox-$crateFile-$PID"
        New-Item -ItemType Directory -Force $scratch | Out-Null
        Push-Location $scratch
        try {
            $croxArgs = @()
            if ($CroxMinimumDuration -gt 0) { $croxArgs += @("--minimum-duration", $CroxMinimumDuration) }
            & crox @croxArgs (Join-Path $profdata.DirectoryName $profdata.BaseName)
            if ($LASTEXITCODE -ne 0) { throw "crox failed for $($profdata.FullName)" }
            $json = Join-Path $OutputDir "$crateFile.json"
            Move-Item -Force (Join-Path $scratch "chrome_profiler.json") $json
        } finally {
            Pop-Location
            Remove-Item -Recurse -Force $scratch -ErrorAction SilentlyContinue
        }

        Get-ChildItem $OutputDir -Filter "$crateFile-*.mm_profdata" | Where-Object Name -ne $profdata.Name | Remove-Item -Force
        Copy-Item -Force $profdata.FullName $OutputDir

        $results += [pscustomobject]@{
            Driver      = $driver.Name
            Command     = if ($SkipBuild) { "-" } else { "cargo $(Get-BuildCommand)" }
            BuildTime   = if ($buildTime) { $buildTime.ToString("hh\:mm\:ss") } else { "skipped" }
            Unprofiled  = if ($unprofiledTime) { $unprofiledTime.ToString("hh\:mm\:ss") } else { "-" }
            Profiles    = (Get-ChildItem $profileDir -Filter *.mm_profdata).Count
            MmProfdata  = $profdata.Name
            ChromeTrace = "$crateFile.json"
        }
    }
} finally {
    $env:RUSTFLAGS = $savedRustflags
}

Write-Host "`nOutput: $OutputDir" -ForegroundColor Green
$results | Format-Table -AutoSize

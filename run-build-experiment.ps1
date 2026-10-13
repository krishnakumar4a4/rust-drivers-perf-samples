<#
.SYNOPSIS
    Build-time experiment: for each driver, 2 full builds followed by 2 driver-only builds.

.DESCRIPTION
    For every driver folder (or the ones given with -Drivers) this runs build-self-profile.ps1
    four times in a row, in this order:
      1. Full build            (cargo clean, then build every crate)
      2. Full build            (again)
      3. Driver-only build     (-Incremental: cargo clean -p <driver>, dependencies reused)
      4. Driver-only build     (again)

    Each run's wall-clock time is read from <driver>\self-profile\build-time.json and reported as
    full build 1, full build 2, driver-only build 1, driver-only build 2. The results are also
    appended to -ResultsCsv so runs can be compared over time.

    With -WdkBuild the exact same experiment uses `cargo +<toolchain> wdk build` instead of
    `cargo +<toolchain> build`. With -Release every build uses the release profile.

.EXAMPLE
    .\run-build-experiment.ps1

.EXAMPLE
    .\run-build-experiment.ps1 -WdkBuild

.EXAMPLE
    .\run-build-experiment.ps1 -Release -ResultsCsv build-experiment-release.csv

.EXAMPLE
    .\run-build-experiment.ps1 -Drivers sample-kmdf-no-tracing,sample-kmdf-with-traces

.EXAMPLE
    .\run-build-experiment.ps1 -DriverSet NonDummy    # only drivers without dummy traces
#>
[CmdletBinding()]
param(
    # Driver folder names (default: every top-level folder with a Cargo.toml).
    [string[]] $Drivers,

    # Which drivers to run: NonDummy (real/no tracing), Dummy (*-dummy-traces) or All.
    # With All, every non-dummy driver runs before any dummy one.
    [ValidateSet("All", "NonDummy", "Dummy")]
    [string] $DriverSet = "All",

    # Use `cargo wdk build` (compile + package/sign) instead of `cargo build`.
    [switch] $WdkBuild,

    # Build with the release profile instead of the dev profile.
    [switch] $Release,

    # rustup toolchain used for the builds.
    [string] $Toolchain = "stage1",

    # Extra arguments passed through to the cargo build command (use -Release for the release profile).
    [string[]] $CargoArgs = @(),

    # CSV file (relative to this script or absolute) that each experiment's results are appended to.
    [string] $ResultsCsv = "build-experiment-results.csv"
)

$ErrorActionPreference = "Stop"
$buildScript = Join-Path $PSScriptRoot "build-self-profile.ps1"
if (-not [System.IO.Path]::IsPathRooted($ResultsCsv)) { $ResultsCsv = Join-Path $PSScriptRoot $ResultsCsv }

# Non-dummy drivers first, then dummy ones; within each group no-tracing, then 1x / 10x / 100x traces.
function Get-TraceOrder([string] $name) {
    $multiplier = if ($name -match 'no-tracing') { 0 }
                  elseif ($name -match '(\d+)x-(dummy-)?traces') { [int]$Matches[1] }
                  elseif ($name -match 'with-(dummy-)?traces') { 1 }
                  else { [int]::MaxValue }
    [pscustomobject]@{ Dummy = $name -match 'dummy-traces'; Multiplier = $multiplier; Name = $name }
}

$driverNames = Get-ChildItem -Directory $PSScriptRoot |
    Where-Object { Test-Path (Join-Path $_.FullName "Cargo.toml") } | ForEach-Object Name
if ($Drivers) {
    $unknown = $Drivers | Where-Object { $_ -notin $driverNames }
    if ($unknown) { throw "Unknown driver folder(s): $($unknown -join ', ')" }
    $driverNames = $Drivers
}
$driverNames = @($driverNames | ForEach-Object { Get-TraceOrder $_ } |
    Where-Object { $DriverSet -eq "All" -or ($DriverSet -eq "Dummy") -eq $_.Dummy } |
    Sort-Object Dummy, Multiplier, Name | ForEach-Object Name)
if (-not $driverNames) { throw "No drivers match -DriverSet $DriverSet" }

$cargoProfile = if ($Release) { "release" } else { "dev" }
$command = if ($WdkBuild) { "cargo +$Toolchain wdk build" } else { "cargo +$Toolchain build" }
if ($Release) { $command += " --profile release" }

# Export-Csv -Append fails (or drops columns) when the existing file has different columns, e.g. a CSV
# written before the Profile column was added.
$csvColumns = "Timestamp", "Driver", "Command", "Profile", "CargoArgs",
              "FullBuild1_s", "FullBuild2_s", "DriverOnlyBuild1_s", "DriverOnlyBuild2_s"
if ((Test-Path $ResultsCsv) -and (Get-Item $ResultsCsv).Length -gt 0) {
    $existingColumns = (Get-Content $ResultsCsv -TotalCount 1) -split ',' | ForEach-Object { $_.Trim('"') }
    if (Compare-Object $existingColumns $csvColumns -SyncWindow 0) {
        throw "$ResultsCsv has different columns ($($existingColumns -join ', ')); use a new -ResultsCsv file."
    }
}
$steps = @(
    @{ Label = "Full build 1";        Incremental = $false; Key = "full_build_seconds" }
    @{ Label = "Full build 2";        Incremental = $false; Key = "full_build_seconds" }
    @{ Label = "Driver-only build 1"; Incremental = $true;  Key = "incremental_build_seconds" }
    @{ Label = "Driver-only build 2"; Incremental = $true;  Key = "incremental_build_seconds" }
)

Write-Host "Experiment: $command, drivers: $($driverNames -join ', ')" -ForegroundColor Green
$experimentStart = Get-Date
$results = @()
$failed = @()
foreach ($driver in $driverNames) {
    $times = @()
    try {
        foreach ($step in $steps) {
            Write-Host "`n##### $driver - $($step.Label) ($command) #####" -ForegroundColor Magenta
            $params = @{
                Drivers     = @($driver)
                Toolchain   = $Toolchain
                CargoArgs   = $CargoArgs
                Incremental = $step.Incremental
                WdkBuild    = [bool]$WdkBuild
                Release     = [bool]$Release
            }
            & $buildScript @params | Out-Host

            $timeFile = Join-Path $PSScriptRoot "$driver\self-profile\build-time.json"
            $seconds = (Get-Content $timeFile -Raw | ConvertFrom-Json).($step.Key)
            if ($null -eq $seconds) { throw "$($step.Key) missing from $timeFile" }
            $times += [double]$seconds
            Write-Host "$driver - $($step.Label): $([math]::Round($seconds, 1))s" -ForegroundColor Magenta
        }
    } catch {
        Write-Warning "$driver failed during '$($step.Label)', skipping the rest of its experiment: $_"
        $failed += $driver
        continue
    }
    $row = [pscustomobject]@{
        Timestamp          = $experimentStart.ToString("o")
        Driver             = $driver
        Command            = $command
        Profile            = $cargoProfile
        CargoArgs          = $CargoArgs -join " "
        FullBuild1_s       = $times[0]
        FullBuild2_s       = $times[1]
        DriverOnlyBuild1_s = $times[2]
        DriverOnlyBuild2_s = $times[3]
    }
    # Written per driver, so a later failure doesn't lose finished results.
    $row | Export-Csv -Path $ResultsCsv -Append -NoTypeInformation -Encoding utf8
    $results += $row
}
function Format-Duration([double] $s) { "{0,7:N1}s ({1:mm\:ss})" -f $s, [TimeSpan]::FromSeconds($s) }
Write-Host "`nBuild times ($command)" -ForegroundColor Green
$results | Format-Table -AutoSize Driver,
    @{ n = "Full build 1";        e = { Format-Duration $_.FullBuild1_s } },
    @{ n = "Full build 2";        e = { Format-Duration $_.FullBuild2_s } },
    @{ n = "Driver-only build 1"; e = { Format-Duration $_.DriverOnlyBuild1_s } },
    @{ n = "Driver-only build 2"; e = { Format-Duration $_.DriverOnlyBuild2_s } } | Out-Host
Write-Host "Results appended to $ResultsCsv"
if ($failed) {
    Write-Warning "Failed drivers (not in the CSV): $($failed -join ', ')"
    exit 1
}

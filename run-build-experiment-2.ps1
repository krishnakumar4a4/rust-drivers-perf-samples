<#
.SYNOPSIS
    Build-time experiment 2: one full release build per driver, then aggregate the profiles.

.DESCRIPTION
    1. Runs a full release-profile build (build-self-profile.ps1 -Release) for every driver, one after
       the other: drivers without dummy traces first (no-tracing, 1x, 10x, 100x), then the
       *-dummy-traces drivers.
    2. After each driver, appends its build wall-clock time and driver binary size to -ResultsCsv.
    3. Runs aggregate-self-profile.py on the drivers that built successfully and saves its output
       to -AggregateOutput.

    With -WdkBuild the builds use `cargo +<toolchain> wdk build --profile release` (binary: .sys)
    instead of `cargo +<toolchain> build --profile release` (binary: .dll).

.EXAMPLE
    .\run-build-experiment-2.ps1

.EXAMPLE
    .\run-build-experiment-2.ps1 -WdkBuild -ResultsCsv build-experiment-2-wdk.csv -AggregateOutput build-experiment-2-wdk-aggregate.txt
#>
[CmdletBinding()]
param(
    # Driver folder names (default: every top-level folder with a Cargo.toml).
    [string[]] $Drivers,

    # Which drivers to run: NonDummy, Dummy or All (non-dummy drivers first).
    [ValidateSet("All", "NonDummy", "Dummy")]
    [string] $DriverSet = "All",

    # Use `cargo wdk build` (compile + package/sign) instead of `cargo build`.
    [switch] $WdkBuild,

    # rustup toolchain used for the builds.
    [string] $Toolchain = "stage1",

    # Extra arguments passed through to the cargo build command.
    [string[]] $CargoArgs = @(),

    # CSV file (relative to this script or absolute) that each driver's results are appended to.
    [string] $ResultsCsv = "build-experiment-2-results.csv",

    # Text file (relative to this script or absolute) that receives aggregate-self-profile.py's output.
    [string] $AggregateOutput = "build-experiment-2-aggregate.txt"
)

$ErrorActionPreference = "Stop"
$buildScript = Join-Path $PSScriptRoot "build-self-profile.ps1"
$aggregateScript = Join-Path $PSScriptRoot "aggregate-self-profile.py"
foreach ($name in "ResultsCsv", "AggregateOutput") {
    $value = Get-Variable $name -ValueOnly
    if (-not [System.IO.Path]::IsPathRooted($value)) { Set-Variable $name (Join-Path $PSScriptRoot $value) }
}
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { throw "'python' was not found on PATH." }

# Non-dummy drivers first, then dummy ones; within each group no-tracing, then 1x / 10x / 100x traces.
function Get-TraceOrder([string] $name) {
    $multiplier = if ($name -match 'no-tracing') { 0 }
                  elseif ($name -match '(\d+)x-(dummy-)?traces') { [int]$Matches[1] }
                  elseif ($name -match 'with-(dummy-)?traces') { 1 }
                  else { [int]::MaxValue }
    [pscustomobject]@{ Dummy = $name -match 'dummy-traces'; Multiplier = $multiplier; Name = $name }
}

function Get-CrateFileName([string] $manifest) {
    $inPackage = $false
    foreach ($line in Get-Content $manifest) {
        if ($line -match '^\s*\[(.+)\]\s*$') { $inPackage = $Matches[1].Trim() -eq "package"; continue }
        if ($inPackage -and $line -match '^\s*name\s*=\s*"([^"]+)"') { return $Matches[1].Replace("-", "_") }
    }
    throw "Could not find [package] name in $manifest"
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

$command = if ($WdkBuild) { "cargo +$Toolchain wdk build --profile release" } else { "cargo +$Toolchain build --profile release" }

# Export-Csv -Append fails (or drops columns) when the existing file has different columns.
$csvColumns = "Timestamp", "Driver", "Command", "Profile", "CargoArgs", "BuildSeconds", "Binary", "BinaryBytes"
if ((Test-Path $ResultsCsv) -and (Get-Item $ResultsCsv).Length -gt 0) {
    $existingColumns = (Get-Content $ResultsCsv -TotalCount 1) -split ',' | ForEach-Object { $_.Trim('"') }
    if (Compare-Object $existingColumns $csvColumns -SyncWindow 0) {
        throw "$ResultsCsv has different columns ($($existingColumns -join ', ')); use a new -ResultsCsv file."
    }
}

Write-Host "Experiment 2: $command, drivers: $($driverNames -join ', ')" -ForegroundColor Green
$experimentStart = Get-Date
$results = @()
$failed = @()
foreach ($driver in $driverNames) {
    Write-Host "`n##### $driver - full release build ($command) #####" -ForegroundColor Magenta
    try {
        $params = @{
            Drivers   = @($driver)
            Toolchain = $Toolchain
            CargoArgs = $CargoArgs
            WdkBuild  = [bool]$WdkBuild
            Release   = $true
        }
        & $buildScript @params | Out-Host

        $timeFile = Join-Path $PSScriptRoot "$driver\self-profile\build-time.json"
        $seconds = (Get-Content $timeFile -Raw | ConvertFrom-Json).full_build_seconds
        if ($null -eq $seconds) { throw "full_build_seconds missing from $timeFile" }

        # `cargo wdk build` produces <crate>.sys; `cargo build` leaves <crate>.dll (target\release or
        # target\<triple>\release).
        $crateFile = Get-CrateFileName (Join-Path $PSScriptRoot "$driver\Cargo.toml")
        $ext = if ($WdkBuild) { "sys" } else { "dll" }
        $target = Join-Path $PSScriptRoot "$driver\target"
        $binary = @(Get-Item "$target\release\$crateFile.$ext", "$target\*\release\$crateFile.$ext" -ErrorAction SilentlyContinue) |
            Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if (-not $binary) { throw "No $crateFile.$ext found under $target" }
    } catch {
        Write-Warning "$driver failed: $_"
        $failed += $driver
        continue
    }

    $row = [pscustomobject]@{
        Timestamp    = $experimentStart.ToString("o")
        Driver       = $driver
        Command      = $command
        Profile      = "release"
        CargoArgs    = $CargoArgs -join " "
        BuildSeconds = [double]$seconds
        Binary       = $binary.FullName.Substring($PSScriptRoot.Length + 1)
        BinaryBytes  = $binary.Length
    }
    # Written per driver, so a later failure doesn't lose finished results.
    $row | Export-Csv -Path $ResultsCsv -Append -NoTypeInformation -Encoding utf8
    $results += $row
    Write-Host ("{0}: {1:N1}s, {2} = {3:N0} bytes" -f $driver, $seconds, (Split-Path $binary -Leaf), $binary.Length) -ForegroundColor Magenta
}

Write-Host "`nBuild times and binary sizes ($command)" -ForegroundColor Green
$results | Format-Table -AutoSize Driver,
    @{ n = "Build time"; e = { "{0,7:N1}s ({1:mm\:ss})" -f $_.BuildSeconds, [TimeSpan]::FromSeconds($_.BuildSeconds) } },
    @{ n = "Binary"; e = { Split-Path $_.Binary -Leaf } },
    @{ n = "Binary bytes"; e = { "{0:N0}" -f $_.BinaryBytes } } | Out-String -Width 240 | Write-Host
Write-Host "Results appended to $ResultsCsv"

if ($results) {
    Write-Host "`nRunning aggregate-self-profile.py on $($results.Count) driver(s) ..." -ForegroundColor Green
    $driverPaths = @($results | ForEach-Object { Join-Path $PSScriptRoot $_.Driver })
    # stdout (the tables) goes to the file; progress and warnings on stderr stay in the terminal.
    $output = & python $aggregateScript @driverPaths
    if ($LASTEXITCODE -ne 0) {
        $failed += "aggregate-self-profile.py (exit $LASTEXITCODE)"
    } else {
        $header = "aggregate-self-profile.py output - experiment started $($experimentStart.ToString('o')), $command"
        @($header, "") + $output | Set-Content -Path $AggregateOutput -Encoding utf8
        Write-Host "Aggregate output written to $AggregateOutput"
    }
}

if ($failed) {
    Write-Warning "Failed: $($failed -join ', ')"
    exit 1
}

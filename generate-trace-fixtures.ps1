[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

# Mode "Wpp" emits real `trace!` statements. Mode "Dummy" emits `dummy_trace(...)` calls
# (see <driver>\src\dummy_trace.rs) with exactly the same level, keyword, message, and fields,
# so each dummy fixture is the no-tracing-cost baseline for the traced fixture of the same size.
$fixtures = @(
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-traces"
        Total = 10
        InstancesPerScenario = 1
        Mode = "Wpp"
    },
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-10x-traces"
        Total = 100
        InstancesPerScenario = 10
        Mode = "Wpp"
    },
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-100x-traces"
        Total = 1000
        InstancesPerScenario = 100
        Mode = "Wpp"
    },
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-dummy-traces"
        Total = 10
        InstancesPerScenario = 1
        Mode = "Dummy"
    },
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-10x-dummy-traces"
        Total = 100
        InstancesPerScenario = 10
        Mode = "Dummy"
    },
    @{
        Path = Join-Path $PSScriptRoot "sample-kmdf-with-100x-dummy-traces"
        Total = 1000
        InstancesPerScenario = 100
        Mode = "Dummy"
    }
)

$levels = @("INFO", "VERBOSE", "WARNING", "ERROR", "CRITICAL")
$coreKeywords = @("LITERALS", "SIGNED", "UNSIGNED", "FLOATS", "STRINGS")
$detailKeywords = @("STATUS", "ENUM_PRIMARY", "GENERIC", "ENUM_SECONDARY", "MIXED")

function Get-Level([int] $scenario, [int] $instance) {
    return $levels[($scenario + $instance) % $levels.Count]
}

function Get-Keyword([int] $scenario, [int] $instance) {
    $keywords = if ($scenario -lt 5) { $coreKeywords } else { $detailKeywords }
    $baseIndex = $scenario % 5
    return $keywords[($baseIndex + [math]::Floor($instance / 5)) % $keywords.Count]
}

function New-ScenarioLines([hashtable] $fixture) {
    $lines = [System.Collections.Generic.List[string]]::new()
    if ($fixture.Mode -eq "Dummy") {
        $lines.Add("use crate::dummy_trace::*;")
        $lines.Add("")
    }
    return , $lines
}

function Add-Trace {
    param(
        [hashtable] $Fixture,
        [System.Collections.Generic.List[string]] $Lines,
        [string] $Level,
        [string] $Keyword,
        [string] $Message,
        [string[]] $Arguments
    )

    if ($Fixture.Mode -eq "Dummy") {
        $Lines.Add("    dummy_trace(")
        $lastArgumentSuffix = ","
    } else {
        $Lines.Add("    trace!(")
        $lastArgumentSuffix = ""
    }
    $Lines.Add("        $Level,")
    $Lines.Add("        $Keyword,")
    $Lines.Add("        `"$Message`",")
    $Lines.Add("        $($Arguments[0]),")
    $Lines.Add("        $($Arguments[1]),")
    $Lines.Add("        $($Arguments[2])$lastArgumentSuffix")
    $Lines.Add("    );")
}

function Write-RustFile([string] $path, [System.Collections.Generic.List[string]] $lines) {
    $parent = Split-Path -Parent $path
    if (-not (Test-Path -LiteralPath $parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }
    $content = ($lines -join "`n") + "`n"
    Set-Content -LiteralPath $path -Value $content -Encoding utf8 -NoNewline
}

function New-LiteralScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "00$id"
        $signed = $instance + 1
        $unsigned = 60000 + $instance
        $boolean = if (($instance % 2) -eq 0) { "true" } else { "false" }
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        Add-Trace $fixture $lines (Get-Level 0 $instance) (Get-Keyword 0 $instance) `
            "fixture-$($fixture.Total) scenario-00 instance-$id`: signed={}, unsigned={}, bool={}" `
            @("-$($signed)_i8", "$($unsigned)_u16", $boolean)
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_00$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-SignedScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "01$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix(signed_i16_$suffix`: i16, signed_i32_$suffix`: i32, signed_i64_$suffix`: i64) {")
        Add-Trace $fixture $lines (Get-Level 1 $instance) (Get-Keyword 1 $instance) `
            "fixture-$($fixture.Total) scenario-01 instance-$id`: i16={}, i32={}, i64={}" `
            @("signed_i16_$suffix", "signed_i32_$suffix", "signed_i64_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_01$id(-$($instance + 1600), -$($instance + 32000), -$($instance + 64000));")
    }
    $lines.Add("}")
    return $lines
}

function New-UnsignedScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "02$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let unsigned_u8_$suffix = $($instance % 256)_u8;")
        $lines.Add("    let unsigned_u32_$suffix = $([uint64]4000000000 + $instance)_u32;")
        $lines.Add("    let unsigned_u64_$suffix = $([uint64]8000000000 + $instance)_u64;")
        Add-Trace $fixture $lines (Get-Level 2 $instance) (Get-Keyword 2 $instance) `
            "fixture-$($fixture.Total) scenario-02 instance-$id`: u8={}, u32={}, u64={}" `
            @("unsigned_u8_$suffix", "unsigned_u32_$suffix", "unsigned_u64_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_02$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-FloatScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "03$id"
        $boolean = if (($instance % 2) -eq 0) { "false" } else { "true" }
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let float_f32_$suffix = $($instance + 3).25_f32;")
        $lines.Add("    let float_f64_$suffix = $($instance + 30).125_f64;")
        $lines.Add("    let bool_$suffix = $boolean;")
        Add-Trace $fixture $lines (Get-Level 3 $instance) (Get-Keyword 3 $instance) `
            "fixture-$($fixture.Total) scenario-03 instance-$id`: f32={}, f64={}, bool={}" `
            @("float_f32_$suffix", "float_f64_$suffix", "bool_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_03$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-StringScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    $lines.Add("fn returned_text() -> &'static str {")
    $lines.Add("    `"returned-text`"")
    $lines.Add("}")
    $lines.Add("")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "04$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let str_$suffix = `"variable-$suffix`";")
        $lines.Add("    let c_str_$suffix = c`"c-string-$suffix`";")
        $lines.Add("    let returned_str_$suffix = returned_text();")
        Add-Trace $fixture $lines (Get-Level 4 $instance) (Get-Keyword 4 $instance) `
            "fixture-$($fixture.Total) scenario-04 instance-$id`: str={}, cstr={}, returned={}" `
            @("str_$suffix", "c_str_$suffix", "returned_str_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_04$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-StatusScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    $lines.Add("use wdf::{status_codes, IRQL, NtStatus};")
    $lines.Add("")
    $lines.Add("fn returned_usize(value: usize) -> usize {")
    $lines.Add("    value")
    $lines.Add("}")
    $lines.Add("")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "05$id"
        $status = if (($instance % 2) -eq 0) { "STATUS_SUCCESS" } else { "STATUS_ACCESS_DENIED" }
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let irql_$suffix = IRQL::current().unwrap_or(IRQL::Passive);")
        $lines.Add("    let status_$suffix = NtStatus::from(status_codes::$status);")
        $lines.Add("    let usize_$suffix = returned_usize($($instance + 5000));")
        Add-Trace $fixture $lines (Get-Level 5 $instance) (Get-Keyword 5 $instance) `
            "fixture-$($fixture.Total) scenario-05 instance-$id`: irql={}, status={}, usize={}" `
            @("irql_$suffix", "status_$suffix", "usize_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_05$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-PrimaryEnumScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    $lines.Add("use crate::{ByteState, LongState, ShortState};")
    $lines.Add("")
    $byteVariants = @("Unknown", "Starting", "Ready")
    $shortVariants = @("Unknown", "Warning", "Failed")
    $longVariants = @("Offline", "Online", "Recovering")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "06$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let byte_state_$suffix = ByteState::$($byteVariants[$instance % 3]);")
        $lines.Add("    let short_state_$suffix = ShortState::$($shortVariants[$instance % 3]);")
        $lines.Add("    let long_state_$suffix = LongState::$($longVariants[$instance % 3]);")
        Add-Trace $fixture $lines (Get-Level 6 $instance) (Get-Keyword 6 $instance) `
            "fixture-$($fixture.Total) scenario-06 instance-$id`: byte={}, short={}, long={}" `
            @("byte_state_$suffix", "short_state_$suffix", "long_state_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_06$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-GenericScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "07$id"
        $lines.Add("#[inline(never)]")
        if ($fixture.Mode -eq "Dummy") {
            $lines.Add("fn emit_$suffix<T, U, V>(generic_t_$suffix`: T, generic_u_$suffix`: U, generic_v_$suffix`: V) {")
        } else {
            $lines.Add("fn emit_$suffix<T, U, V>(generic_t_$suffix`: T, generic_u_$suffix`: U, generic_v_$suffix`: V)")
            $lines.Add("where")
            $lines.Add("    T: wpp::IntoWppField,")
            $lines.Add("    U: wpp::IntoWppField,")
            $lines.Add("    V: wpp::IntoWppField,")
            $lines.Add("{")
        }
        Add-Trace $fixture $lines (Get-Level 7 $instance) (Get-Keyword 7 $instance) `
            "fixture-$($fixture.Total) scenario-07 instance-$id`: generic-t={}, generic-u={}, generic-v={}" `
            @("generic_t_$suffix", "generic_u_$suffix", "generic_v_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_07$id(-$($instance + 7)_isize, `"generic-$id`", $($instance + 70)_u32);")
    }
    $lines.Add("}")
    return $lines
}

function New-SecondaryEnumScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    $lines.Add("use crate::{ImplicitState, MixedState};")
    $lines.Add("")
    $lines.Add("fn returned_i32(value: i32) -> i32 {")
    $lines.Add("    value")
    $lines.Add("}")
    $lines.Add("")
    $implicitVariants = @("Unknown", "Starting", "Ready")
    $mixedVariants = @("Unknown", "Warning", "Failed")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "08$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix() {")
        $lines.Add("    let implicit_state_$suffix = ImplicitState::$($implicitVariants[$instance % 3]);")
        $lines.Add("    let mixed_state_$suffix = MixedState::$($mixedVariants[$instance % 3]);")
        $lines.Add("    let returned_i32_$suffix = returned_i32(-$($instance + 8000));")
        Add-Trace $fixture $lines (Get-Level 8 $instance) (Get-Keyword 8 $instance) `
            "fixture-$($fixture.Total) scenario-08 instance-$id`: implicit={}, mixed={}, returned-i32={}" `
            @("implicit_state_$suffix", "mixed_state_$suffix", "returned_i32_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_08$id();")
    }
    $lines.Add("}")
    return $lines
}

function New-MixedScenario([hashtable] $fixture, [int] $startInstance, [int] $instanceCount) {
    $lines = New-ScenarioLines $fixture
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $suffix = "09$id"
        $lines.Add("#[inline(never)]")
        $lines.Add("fn emit_$suffix(mixed_u8_$suffix`: u8, mixed_i16_$suffix`: i16, mixed_usize_$suffix`: usize) {")
        Add-Trace $fixture $lines (Get-Level 9 $instance) (Get-Keyword 9 $instance) `
            "fixture-$($fixture.Total) scenario-09 instance-$id`: u8={}, i16={}, usize={}" `
            @("mixed_u8_$suffix", "mixed_i16_$suffix", "mixed_usize_$suffix")
        $lines.Add("}")
        $lines.Add("")
    }
    $lines.Add("pub(crate) fn emit() {")
    for ($instance = $startInstance; $instance -lt ($startInstance + $instanceCount); $instance++) {
        $id = "{0:D3}" -f $instance
        $lines.Add("    emit_09$id($($instance + 90), -$($instance + 900), $($instance + 9000));")
    }
    $lines.Add("}")
    return $lines
}

$scenarios = @(
    @{ Name = "scenario_00_literals"; Generator = "New-LiteralScenario" },
    @{ Name = "scenario_01_signed"; Generator = "New-SignedScenario" },
    @{ Name = "scenario_02_unsigned"; Generator = "New-UnsignedScenario" },
    @{ Name = "scenario_03_floats"; Generator = "New-FloatScenario" },
    @{ Name = "scenario_04_strings"; Generator = "New-StringScenario" },
    @{ Name = "scenario_05_status"; Generator = "New-StatusScenario" },
    @{ Name = "scenario_06_enum_primary"; Generator = "New-PrimaryEnumScenario" },
    @{ Name = "scenario_07_generic"; Generator = "New-GenericScenario" },
    @{ Name = "scenario_08_enum_secondary"; Generator = "New-SecondaryEnumScenario" },
    @{ Name = "scenario_09_mixed"; Generator = "New-MixedScenario" }
)

foreach ($fixture in $fixtures) {
    $sourcePath = Join-Path $fixture.Path "src"

    foreach ($scenario in $scenarios) {
        $flatPath = Join-Path $sourcePath "$($scenario.Name).rs"
        $shardPath = Join-Path $sourcePath $scenario.Name
        $generator = $scenario.Generator

        if ($fixture.InstancesPerScenario -le 10) {
            if (Test-Path -LiteralPath $shardPath) {
                Remove-Item -LiteralPath $shardPath -Recurse -Force
            }
            Write-RustFile $flatPath (& $generator $fixture 0 $fixture.InstancesPerScenario)
            continue
        }

        if (Test-Path -LiteralPath $flatPath) {
            Remove-Item -LiteralPath $flatPath -Force
        }
        if (Test-Path -LiteralPath $shardPath) {
            Remove-Item -LiteralPath $shardPath -Recurse -Force
        }
        New-Item -ItemType Directory -Path $shardPath -Force | Out-Null

        $moduleLines = [System.Collections.Generic.List[string]]::new()
        $shardCount = [math]::Ceiling($fixture.InstancesPerScenario / 10)
        for ($shard = 0; $shard -lt $shardCount; $shard++) {
            $shardName = "shard_{0:D2}" -f $shard
            $moduleLines.Add("mod $shardName;")
            $startInstance = $shard * 10
            $instanceCount = [math]::Min(10, $fixture.InstancesPerScenario - $startInstance)
            Write-RustFile (Join-Path $shardPath "$shardName.rs") (
                & $generator $fixture $startInstance $instanceCount
            )
        }
        $moduleLines.Add("")
        $moduleLines.Add("pub(crate) fn emit() {")
        for ($shard = 0; $shard -lt $shardCount; $shard++) {
            $shardName = "shard_{0:D2}" -f $shard
            $moduleLines.Add("    $shardName::emit();")
        }
        $moduleLines.Add("}")
        Write-RustFile (Join-Path $shardPath "mod.rs") $moduleLines
    }
}

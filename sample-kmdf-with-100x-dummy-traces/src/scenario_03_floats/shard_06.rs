use crate::dummy_trace::*;

#[inline(never)]
fn emit_03060() {
    let float_f32_03060 = 63.25_f32;
    let float_f64_03060 = 90.125_f64;
    let bool_03060 = false;
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-03 instance-060: f32={}, f64={}, bool={}",
        float_f32_03060,
        float_f64_03060,
        bool_03060,
    );
}

#[inline(never)]
fn emit_03061() {
    let float_f32_03061 = 64.25_f32;
    let float_f64_03061 = 91.125_f64;
    let bool_03061 = true;
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-03 instance-061: f32={}, f64={}, bool={}",
        float_f32_03061,
        float_f64_03061,
        bool_03061,
    );
}

#[inline(never)]
fn emit_03062() {
    let float_f32_03062 = 65.25_f32;
    let float_f64_03062 = 92.125_f64;
    let bool_03062 = false;
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-03 instance-062: f32={}, f64={}, bool={}",
        float_f32_03062,
        float_f64_03062,
        bool_03062,
    );
}

#[inline(never)]
fn emit_03063() {
    let float_f32_03063 = 66.25_f32;
    let float_f64_03063 = 93.125_f64;
    let bool_03063 = true;
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-03 instance-063: f32={}, f64={}, bool={}",
        float_f32_03063,
        float_f64_03063,
        bool_03063,
    );
}

#[inline(never)]
fn emit_03064() {
    let float_f32_03064 = 67.25_f32;
    let float_f64_03064 = 94.125_f64;
    let bool_03064 = false;
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-03 instance-064: f32={}, f64={}, bool={}",
        float_f32_03064,
        float_f64_03064,
        bool_03064,
    );
}

#[inline(never)]
fn emit_03065() {
    let float_f32_03065 = 68.25_f32;
    let float_f64_03065 = 95.125_f64;
    let bool_03065 = true;
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-03 instance-065: f32={}, f64={}, bool={}",
        float_f32_03065,
        float_f64_03065,
        bool_03065,
    );
}

#[inline(never)]
fn emit_03066() {
    let float_f32_03066 = 69.25_f32;
    let float_f64_03066 = 96.125_f64;
    let bool_03066 = false;
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-03 instance-066: f32={}, f64={}, bool={}",
        float_f32_03066,
        float_f64_03066,
        bool_03066,
    );
}

#[inline(never)]
fn emit_03067() {
    let float_f32_03067 = 70.25_f32;
    let float_f64_03067 = 97.125_f64;
    let bool_03067 = true;
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-03 instance-067: f32={}, f64={}, bool={}",
        float_f32_03067,
        float_f64_03067,
        bool_03067,
    );
}

#[inline(never)]
fn emit_03068() {
    let float_f32_03068 = 71.25_f32;
    let float_f64_03068 = 98.125_f64;
    let bool_03068 = false;
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-03 instance-068: f32={}, f64={}, bool={}",
        float_f32_03068,
        float_f64_03068,
        bool_03068,
    );
}

#[inline(never)]
fn emit_03069() {
    let float_f32_03069 = 72.25_f32;
    let float_f64_03069 = 99.125_f64;
    let bool_03069 = true;
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-03 instance-069: f32={}, f64={}, bool={}",
        float_f32_03069,
        float_f64_03069,
        bool_03069,
    );
}

pub(crate) fn emit() {
    emit_03060();
    emit_03061();
    emit_03062();
    emit_03063();
    emit_03064();
    emit_03065();
    emit_03066();
    emit_03067();
    emit_03068();
    emit_03069();
}

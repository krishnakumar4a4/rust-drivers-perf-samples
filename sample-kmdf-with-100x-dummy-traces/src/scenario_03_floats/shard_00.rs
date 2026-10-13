use crate::dummy_trace::*;

#[inline(never)]
fn emit_03000() {
    let float_f32_03000 = 3.25_f32;
    let float_f64_03000 = 30.125_f64;
    let bool_03000 = false;
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-03 instance-000: f32={}, f64={}, bool={}",
        float_f32_03000,
        float_f64_03000,
        bool_03000,
    );
}

#[inline(never)]
fn emit_03001() {
    let float_f32_03001 = 4.25_f32;
    let float_f64_03001 = 31.125_f64;
    let bool_03001 = true;
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-03 instance-001: f32={}, f64={}, bool={}",
        float_f32_03001,
        float_f64_03001,
        bool_03001,
    );
}

#[inline(never)]
fn emit_03002() {
    let float_f32_03002 = 5.25_f32;
    let float_f64_03002 = 32.125_f64;
    let bool_03002 = false;
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-03 instance-002: f32={}, f64={}, bool={}",
        float_f32_03002,
        float_f64_03002,
        bool_03002,
    );
}

#[inline(never)]
fn emit_03003() {
    let float_f32_03003 = 6.25_f32;
    let float_f64_03003 = 33.125_f64;
    let bool_03003 = true;
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-03 instance-003: f32={}, f64={}, bool={}",
        float_f32_03003,
        float_f64_03003,
        bool_03003,
    );
}

#[inline(never)]
fn emit_03004() {
    let float_f32_03004 = 7.25_f32;
    let float_f64_03004 = 34.125_f64;
    let bool_03004 = false;
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-03 instance-004: f32={}, f64={}, bool={}",
        float_f32_03004,
        float_f64_03004,
        bool_03004,
    );
}

#[inline(never)]
fn emit_03005() {
    let float_f32_03005 = 8.25_f32;
    let float_f64_03005 = 35.125_f64;
    let bool_03005 = true;
    dummy_trace(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-03 instance-005: f32={}, f64={}, bool={}",
        float_f32_03005,
        float_f64_03005,
        bool_03005,
    );
}

#[inline(never)]
fn emit_03006() {
    let float_f32_03006 = 9.25_f32;
    let float_f64_03006 = 36.125_f64;
    let bool_03006 = false;
    dummy_trace(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-03 instance-006: f32={}, f64={}, bool={}",
        float_f32_03006,
        float_f64_03006,
        bool_03006,
    );
}

#[inline(never)]
fn emit_03007() {
    let float_f32_03007 = 10.25_f32;
    let float_f64_03007 = 37.125_f64;
    let bool_03007 = true;
    dummy_trace(
        INFO,
        STRINGS,
        "fixture-1000 scenario-03 instance-007: f32={}, f64={}, bool={}",
        float_f32_03007,
        float_f64_03007,
        bool_03007,
    );
}

#[inline(never)]
fn emit_03008() {
    let float_f32_03008 = 11.25_f32;
    let float_f64_03008 = 38.125_f64;
    let bool_03008 = false;
    dummy_trace(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-03 instance-008: f32={}, f64={}, bool={}",
        float_f32_03008,
        float_f64_03008,
        bool_03008,
    );
}

#[inline(never)]
fn emit_03009() {
    let float_f32_03009 = 12.25_f32;
    let float_f64_03009 = 39.125_f64;
    let bool_03009 = true;
    dummy_trace(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-03 instance-009: f32={}, f64={}, bool={}",
        float_f32_03009,
        float_f64_03009,
        bool_03009,
    );
}

pub(crate) fn emit() {
    emit_03000();
    emit_03001();
    emit_03002();
    emit_03003();
    emit_03004();
    emit_03005();
    emit_03006();
    emit_03007();
    emit_03008();
    emit_03009();
}

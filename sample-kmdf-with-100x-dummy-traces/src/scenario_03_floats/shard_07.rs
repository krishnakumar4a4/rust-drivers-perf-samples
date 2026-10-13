use crate::dummy_trace::*;

#[inline(never)]
fn emit_03070() {
    let float_f32_03070 = 73.25_f32;
    let float_f64_03070 = 100.125_f64;
    let bool_03070 = false;
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-070: f32={}, f64={}, bool={}",
        float_f32_03070,
        float_f64_03070,
        bool_03070,
    );
}

#[inline(never)]
fn emit_03071() {
    let float_f32_03071 = 74.25_f32;
    let float_f64_03071 = 101.125_f64;
    let bool_03071 = true;
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-071: f32={}, f64={}, bool={}",
        float_f32_03071,
        float_f64_03071,
        bool_03071,
    );
}

#[inline(never)]
fn emit_03072() {
    let float_f32_03072 = 75.25_f32;
    let float_f64_03072 = 102.125_f64;
    let bool_03072 = false;
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-072: f32={}, f64={}, bool={}",
        float_f32_03072,
        float_f64_03072,
        bool_03072,
    );
}

#[inline(never)]
fn emit_03073() {
    let float_f32_03073 = 76.25_f32;
    let float_f64_03073 = 103.125_f64;
    let bool_03073 = true;
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-073: f32={}, f64={}, bool={}",
        float_f32_03073,
        float_f64_03073,
        bool_03073,
    );
}

#[inline(never)]
fn emit_03074() {
    let float_f32_03074 = 77.25_f32;
    let float_f64_03074 = 104.125_f64;
    let bool_03074 = false;
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-074: f32={}, f64={}, bool={}",
        float_f32_03074,
        float_f64_03074,
        bool_03074,
    );
}

#[inline(never)]
fn emit_03075() {
    let float_f32_03075 = 78.25_f32;
    let float_f64_03075 = 105.125_f64;
    let bool_03075 = true;
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-03 instance-075: f32={}, f64={}, bool={}",
        float_f32_03075,
        float_f64_03075,
        bool_03075,
    );
}

#[inline(never)]
fn emit_03076() {
    let float_f32_03076 = 79.25_f32;
    let float_f64_03076 = 106.125_f64;
    let bool_03076 = false;
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-03 instance-076: f32={}, f64={}, bool={}",
        float_f32_03076,
        float_f64_03076,
        bool_03076,
    );
}

#[inline(never)]
fn emit_03077() {
    let float_f32_03077 = 80.25_f32;
    let float_f64_03077 = 107.125_f64;
    let bool_03077 = true;
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-03 instance-077: f32={}, f64={}, bool={}",
        float_f32_03077,
        float_f64_03077,
        bool_03077,
    );
}

#[inline(never)]
fn emit_03078() {
    let float_f32_03078 = 81.25_f32;
    let float_f64_03078 = 108.125_f64;
    let bool_03078 = false;
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-03 instance-078: f32={}, f64={}, bool={}",
        float_f32_03078,
        float_f64_03078,
        bool_03078,
    );
}

#[inline(never)]
fn emit_03079() {
    let float_f32_03079 = 82.25_f32;
    let float_f64_03079 = 109.125_f64;
    let bool_03079 = true;
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-03 instance-079: f32={}, f64={}, bool={}",
        float_f32_03079,
        float_f64_03079,
        bool_03079,
    );
}

pub(crate) fn emit() {
    emit_03070();
    emit_03071();
    emit_03072();
    emit_03073();
    emit_03074();
    emit_03075();
    emit_03076();
    emit_03077();
    emit_03078();
    emit_03079();
}

use crate::dummy_trace::*;

#[inline(never)]
fn emit_03010() {
    let float_f32_03010 = 13.25_f32;
    let float_f64_03010 = 40.125_f64;
    let bool_03010 = false;
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-03 instance-010: f32={}, f64={}, bool={}",
        float_f32_03010,
        float_f64_03010,
        bool_03010,
    );
}

#[inline(never)]
fn emit_03011() {
    let float_f32_03011 = 14.25_f32;
    let float_f64_03011 = 41.125_f64;
    let bool_03011 = true;
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-03 instance-011: f32={}, f64={}, bool={}",
        float_f32_03011,
        float_f64_03011,
        bool_03011,
    );
}

#[inline(never)]
fn emit_03012() {
    let float_f32_03012 = 15.25_f32;
    let float_f64_03012 = 42.125_f64;
    let bool_03012 = false;
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-03 instance-012: f32={}, f64={}, bool={}",
        float_f32_03012,
        float_f64_03012,
        bool_03012,
    );
}

#[inline(never)]
fn emit_03013() {
    let float_f32_03013 = 16.25_f32;
    let float_f64_03013 = 43.125_f64;
    let bool_03013 = true;
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-03 instance-013: f32={}, f64={}, bool={}",
        float_f32_03013,
        float_f64_03013,
        bool_03013,
    );
}

#[inline(never)]
fn emit_03014() {
    let float_f32_03014 = 17.25_f32;
    let float_f64_03014 = 44.125_f64;
    let bool_03014 = false;
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-03 instance-014: f32={}, f64={}, bool={}",
        float_f32_03014,
        float_f64_03014,
        bool_03014,
    );
}

#[inline(never)]
fn emit_03015() {
    let float_f32_03015 = 18.25_f32;
    let float_f64_03015 = 45.125_f64;
    let bool_03015 = true;
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-03 instance-015: f32={}, f64={}, bool={}",
        float_f32_03015,
        float_f64_03015,
        bool_03015,
    );
}

#[inline(never)]
fn emit_03016() {
    let float_f32_03016 = 19.25_f32;
    let float_f64_03016 = 46.125_f64;
    let bool_03016 = false;
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-03 instance-016: f32={}, f64={}, bool={}",
        float_f32_03016,
        float_f64_03016,
        bool_03016,
    );
}

#[inline(never)]
fn emit_03017() {
    let float_f32_03017 = 20.25_f32;
    let float_f64_03017 = 47.125_f64;
    let bool_03017 = true;
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-03 instance-017: f32={}, f64={}, bool={}",
        float_f32_03017,
        float_f64_03017,
        bool_03017,
    );
}

#[inline(never)]
fn emit_03018() {
    let float_f32_03018 = 21.25_f32;
    let float_f64_03018 = 48.125_f64;
    let bool_03018 = false;
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-03 instance-018: f32={}, f64={}, bool={}",
        float_f32_03018,
        float_f64_03018,
        bool_03018,
    );
}

#[inline(never)]
fn emit_03019() {
    let float_f32_03019 = 22.25_f32;
    let float_f64_03019 = 49.125_f64;
    let bool_03019 = true;
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-03 instance-019: f32={}, f64={}, bool={}",
        float_f32_03019,
        float_f64_03019,
        bool_03019,
    );
}

pub(crate) fn emit() {
    emit_03010();
    emit_03011();
    emit_03012();
    emit_03013();
    emit_03014();
    emit_03015();
    emit_03016();
    emit_03017();
    emit_03018();
    emit_03019();
}

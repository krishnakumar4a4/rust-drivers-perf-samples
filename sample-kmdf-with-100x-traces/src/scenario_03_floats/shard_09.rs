#[inline(never)]
fn emit_03090() {
    let float_f32_03090 = 93.25_f32;
    let float_f64_03090 = 120.125_f64;
    let bool_03090 = false;
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-03 instance-090: f32={}, f64={}, bool={}",
        float_f32_03090,
        float_f64_03090,
        bool_03090
    );
}

#[inline(never)]
fn emit_03091() {
    let float_f32_03091 = 94.25_f32;
    let float_f64_03091 = 121.125_f64;
    let bool_03091 = true;
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-03 instance-091: f32={}, f64={}, bool={}",
        float_f32_03091,
        float_f64_03091,
        bool_03091
    );
}

#[inline(never)]
fn emit_03092() {
    let float_f32_03092 = 95.25_f32;
    let float_f64_03092 = 122.125_f64;
    let bool_03092 = false;
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-03 instance-092: f32={}, f64={}, bool={}",
        float_f32_03092,
        float_f64_03092,
        bool_03092
    );
}

#[inline(never)]
fn emit_03093() {
    let float_f32_03093 = 96.25_f32;
    let float_f64_03093 = 123.125_f64;
    let bool_03093 = true;
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-03 instance-093: f32={}, f64={}, bool={}",
        float_f32_03093,
        float_f64_03093,
        bool_03093
    );
}

#[inline(never)]
fn emit_03094() {
    let float_f32_03094 = 97.25_f32;
    let float_f64_03094 = 124.125_f64;
    let bool_03094 = false;
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-03 instance-094: f32={}, f64={}, bool={}",
        float_f32_03094,
        float_f64_03094,
        bool_03094
    );
}

#[inline(never)]
fn emit_03095() {
    let float_f32_03095 = 98.25_f32;
    let float_f64_03095 = 125.125_f64;
    let bool_03095 = true;
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-095: f32={}, f64={}, bool={}",
        float_f32_03095,
        float_f64_03095,
        bool_03095
    );
}

#[inline(never)]
fn emit_03096() {
    let float_f32_03096 = 99.25_f32;
    let float_f64_03096 = 126.125_f64;
    let bool_03096 = false;
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-096: f32={}, f64={}, bool={}",
        float_f32_03096,
        float_f64_03096,
        bool_03096
    );
}

#[inline(never)]
fn emit_03097() {
    let float_f32_03097 = 100.25_f32;
    let float_f64_03097 = 127.125_f64;
    let bool_03097 = true;
    trace!(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-097: f32={}, f64={}, bool={}",
        float_f32_03097,
        float_f64_03097,
        bool_03097
    );
}

#[inline(never)]
fn emit_03098() {
    let float_f32_03098 = 101.25_f32;
    let float_f64_03098 = 128.125_f64;
    let bool_03098 = false;
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-098: f32={}, f64={}, bool={}",
        float_f32_03098,
        float_f64_03098,
        bool_03098
    );
}

#[inline(never)]
fn emit_03099() {
    let float_f32_03099 = 102.25_f32;
    let float_f64_03099 = 129.125_f64;
    let bool_03099 = true;
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-099: f32={}, f64={}, bool={}",
        float_f32_03099,
        float_f64_03099,
        bool_03099
    );
}

pub(crate) fn emit() {
    emit_03090();
    emit_03091();
    emit_03092();
    emit_03093();
    emit_03094();
    emit_03095();
    emit_03096();
    emit_03097();
    emit_03098();
    emit_03099();
}

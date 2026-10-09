#[inline(never)]
fn emit_03040() {
    let float_f32_03040 = 43.25_f32;
    let float_f64_03040 = 70.125_f64;
    let bool_03040 = false;
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-03 instance-040: f32={}, f64={}, bool={}",
        float_f32_03040,
        float_f64_03040,
        bool_03040
    );
}

#[inline(never)]
fn emit_03041() {
    let float_f32_03041 = 44.25_f32;
    let float_f64_03041 = 71.125_f64;
    let bool_03041 = true;
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-03 instance-041: f32={}, f64={}, bool={}",
        float_f32_03041,
        float_f64_03041,
        bool_03041
    );
}

#[inline(never)]
fn emit_03042() {
    let float_f32_03042 = 45.25_f32;
    let float_f64_03042 = 72.125_f64;
    let bool_03042 = false;
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-03 instance-042: f32={}, f64={}, bool={}",
        float_f32_03042,
        float_f64_03042,
        bool_03042
    );
}

#[inline(never)]
fn emit_03043() {
    let float_f32_03043 = 46.25_f32;
    let float_f64_03043 = 73.125_f64;
    let bool_03043 = true;
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-03 instance-043: f32={}, f64={}, bool={}",
        float_f32_03043,
        float_f64_03043,
        bool_03043
    );
}

#[inline(never)]
fn emit_03044() {
    let float_f32_03044 = 47.25_f32;
    let float_f64_03044 = 74.125_f64;
    let bool_03044 = false;
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-03 instance-044: f32={}, f64={}, bool={}",
        float_f32_03044,
        float_f64_03044,
        bool_03044
    );
}

#[inline(never)]
fn emit_03045() {
    let float_f32_03045 = 48.25_f32;
    let float_f64_03045 = 75.125_f64;
    let bool_03045 = true;
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-045: f32={}, f64={}, bool={}",
        float_f32_03045,
        float_f64_03045,
        bool_03045
    );
}

#[inline(never)]
fn emit_03046() {
    let float_f32_03046 = 49.25_f32;
    let float_f64_03046 = 76.125_f64;
    let bool_03046 = false;
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-046: f32={}, f64={}, bool={}",
        float_f32_03046,
        float_f64_03046,
        bool_03046
    );
}

#[inline(never)]
fn emit_03047() {
    let float_f32_03047 = 50.25_f32;
    let float_f64_03047 = 77.125_f64;
    let bool_03047 = true;
    trace!(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-047: f32={}, f64={}, bool={}",
        float_f32_03047,
        float_f64_03047,
        bool_03047
    );
}

#[inline(never)]
fn emit_03048() {
    let float_f32_03048 = 51.25_f32;
    let float_f64_03048 = 78.125_f64;
    let bool_03048 = false;
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-048: f32={}, f64={}, bool={}",
        float_f32_03048,
        float_f64_03048,
        bool_03048
    );
}

#[inline(never)]
fn emit_03049() {
    let float_f32_03049 = 52.25_f32;
    let float_f64_03049 = 79.125_f64;
    let bool_03049 = true;
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-03 instance-049: f32={}, f64={}, bool={}",
        float_f32_03049,
        float_f64_03049,
        bool_03049
    );
}

pub(crate) fn emit() {
    emit_03040();
    emit_03041();
    emit_03042();
    emit_03043();
    emit_03044();
    emit_03045();
    emit_03046();
    emit_03047();
    emit_03048();
    emit_03049();
}

#[inline(never)]
fn emit_03000() {
    let float_f32_03000 = 3.25_f32;
    let float_f64_03000 = 30.125_f64;
    let bool_03000 = false;
    trace!(
        ERROR,
        FLOATS,
        "fixture-10 scenario-03 instance-000: f32={}, f64={}, bool={}",
        float_f32_03000,
        float_f64_03000,
        bool_03000
    );
}

pub(crate) fn emit() {
    emit_03000();
}

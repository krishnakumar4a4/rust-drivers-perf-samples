#[inline(never)]
fn emit_00000() {
    trace!(
        INFO,
        LITERALS,
        "fixture-10 scenario-00 instance-000: signed={}, unsigned={}, bool={}",
        -1_i8,
        60000_u16,
        true
    );
}

pub(crate) fn emit() {
    emit_00000();
}

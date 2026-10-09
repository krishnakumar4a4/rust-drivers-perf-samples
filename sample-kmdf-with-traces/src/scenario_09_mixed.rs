#[inline(never)]
fn emit_09000(mixed_u8_09000: u8, mixed_i16_09000: i16, mixed_usize_09000: usize) {
    trace!(
        CRITICAL,
        MIXED,
        "fixture-10 scenario-09 instance-000: u8={}, i16={}, usize={}",
        mixed_u8_09000,
        mixed_i16_09000,
        mixed_usize_09000
    );
}

pub(crate) fn emit() {
    emit_09000(90, -900, 9000);
}

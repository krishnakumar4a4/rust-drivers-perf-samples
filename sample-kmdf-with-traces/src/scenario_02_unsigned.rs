#[inline(never)]
fn emit_02000() {
    let unsigned_u8_02000 = 0_u8;
    let unsigned_u32_02000 = 4000000000_u32;
    let unsigned_u64_02000 = 8000000000_u64;
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-10 scenario-02 instance-000: u8={}, u32={}, u64={}",
        unsigned_u8_02000,
        unsigned_u32_02000,
        unsigned_u64_02000
    );
}

pub(crate) fn emit() {
    emit_02000();
}

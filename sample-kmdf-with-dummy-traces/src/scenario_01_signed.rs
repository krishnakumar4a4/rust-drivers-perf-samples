use crate::dummy_trace::*;

#[inline(never)]
fn emit_01000(signed_i16_01000: i16, signed_i32_01000: i32, signed_i64_01000: i64) {
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-10 scenario-01 instance-000: i16={}, i32={}, i64={}",
        signed_i16_01000,
        signed_i32_01000,
        signed_i64_01000,
    );
}

pub(crate) fn emit() {
    emit_01000(-1600, -32000, -64000);
}

use crate::dummy_trace::*;

#[inline(never)]
fn emit_01050(signed_i16_01050: i16, signed_i32_01050: i32, signed_i64_01050: i64) {
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-01 instance-050: i16={}, i32={}, i64={}",
        signed_i16_01050,
        signed_i32_01050,
        signed_i64_01050,
    );
}

#[inline(never)]
fn emit_01051(signed_i16_01051: i16, signed_i32_01051: i32, signed_i64_01051: i64) {
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-01 instance-051: i16={}, i32={}, i64={}",
        signed_i16_01051,
        signed_i32_01051,
        signed_i64_01051,
    );
}

#[inline(never)]
fn emit_01052(signed_i16_01052: i16, signed_i32_01052: i32, signed_i64_01052: i64) {
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-01 instance-052: i16={}, i32={}, i64={}",
        signed_i16_01052,
        signed_i32_01052,
        signed_i64_01052,
    );
}

#[inline(never)]
fn emit_01053(signed_i16_01053: i16, signed_i32_01053: i32, signed_i64_01053: i64) {
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-01 instance-053: i16={}, i32={}, i64={}",
        signed_i16_01053,
        signed_i32_01053,
        signed_i64_01053,
    );
}

#[inline(never)]
fn emit_01054(signed_i16_01054: i16, signed_i32_01054: i32, signed_i64_01054: i64) {
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-01 instance-054: i16={}, i32={}, i64={}",
        signed_i16_01054,
        signed_i32_01054,
        signed_i64_01054,
    );
}

#[inline(never)]
fn emit_01055(signed_i16_01055: i16, signed_i32_01055: i32, signed_i64_01055: i64) {
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-055: i16={}, i32={}, i64={}",
        signed_i16_01055,
        signed_i32_01055,
        signed_i64_01055,
    );
}

#[inline(never)]
fn emit_01056(signed_i16_01056: i16, signed_i32_01056: i32, signed_i64_01056: i64) {
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-056: i16={}, i32={}, i64={}",
        signed_i16_01056,
        signed_i32_01056,
        signed_i64_01056,
    );
}

#[inline(never)]
fn emit_01057(signed_i16_01057: i16, signed_i32_01057: i32, signed_i64_01057: i64) {
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-057: i16={}, i32={}, i64={}",
        signed_i16_01057,
        signed_i32_01057,
        signed_i64_01057,
    );
}

#[inline(never)]
fn emit_01058(signed_i16_01058: i16, signed_i32_01058: i32, signed_i64_01058: i64) {
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-058: i16={}, i32={}, i64={}",
        signed_i16_01058,
        signed_i32_01058,
        signed_i64_01058,
    );
}

#[inline(never)]
fn emit_01059(signed_i16_01059: i16, signed_i32_01059: i32, signed_i64_01059: i64) {
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-059: i16={}, i32={}, i64={}",
        signed_i16_01059,
        signed_i32_01059,
        signed_i64_01059,
    );
}

pub(crate) fn emit() {
    emit_01050(-1650, -32050, -64050);
    emit_01051(-1651, -32051, -64051);
    emit_01052(-1652, -32052, -64052);
    emit_01053(-1653, -32053, -64053);
    emit_01054(-1654, -32054, -64054);
    emit_01055(-1655, -32055, -64055);
    emit_01056(-1656, -32056, -64056);
    emit_01057(-1657, -32057, -64057);
    emit_01058(-1658, -32058, -64058);
    emit_01059(-1659, -32059, -64059);
}

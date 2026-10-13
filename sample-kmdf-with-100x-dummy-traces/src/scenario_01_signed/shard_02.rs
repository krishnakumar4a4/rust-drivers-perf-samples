use crate::dummy_trace::*;

#[inline(never)]
fn emit_01020(signed_i16_01020: i16, signed_i32_01020: i32, signed_i64_01020: i64) {
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-01 instance-020: i16={}, i32={}, i64={}",
        signed_i16_01020,
        signed_i32_01020,
        signed_i64_01020,
    );
}

#[inline(never)]
fn emit_01021(signed_i16_01021: i16, signed_i32_01021: i32, signed_i64_01021: i64) {
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-01 instance-021: i16={}, i32={}, i64={}",
        signed_i16_01021,
        signed_i32_01021,
        signed_i64_01021,
    );
}

#[inline(never)]
fn emit_01022(signed_i16_01022: i16, signed_i32_01022: i32, signed_i64_01022: i64) {
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-01 instance-022: i16={}, i32={}, i64={}",
        signed_i16_01022,
        signed_i32_01022,
        signed_i64_01022,
    );
}

#[inline(never)]
fn emit_01023(signed_i16_01023: i16, signed_i32_01023: i32, signed_i64_01023: i64) {
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-01 instance-023: i16={}, i32={}, i64={}",
        signed_i16_01023,
        signed_i32_01023,
        signed_i64_01023,
    );
}

#[inline(never)]
fn emit_01024(signed_i16_01024: i16, signed_i32_01024: i32, signed_i64_01024: i64) {
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-01 instance-024: i16={}, i32={}, i64={}",
        signed_i16_01024,
        signed_i32_01024,
        signed_i64_01024,
    );
}

#[inline(never)]
fn emit_01025(signed_i16_01025: i16, signed_i32_01025: i32, signed_i64_01025: i64) {
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-01 instance-025: i16={}, i32={}, i64={}",
        signed_i16_01025,
        signed_i32_01025,
        signed_i64_01025,
    );
}

#[inline(never)]
fn emit_01026(signed_i16_01026: i16, signed_i32_01026: i32, signed_i64_01026: i64) {
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-01 instance-026: i16={}, i32={}, i64={}",
        signed_i16_01026,
        signed_i32_01026,
        signed_i64_01026,
    );
}

#[inline(never)]
fn emit_01027(signed_i16_01027: i16, signed_i32_01027: i32, signed_i64_01027: i64) {
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-01 instance-027: i16={}, i32={}, i64={}",
        signed_i16_01027,
        signed_i32_01027,
        signed_i64_01027,
    );
}

#[inline(never)]
fn emit_01028(signed_i16_01028: i16, signed_i32_01028: i32, signed_i64_01028: i64) {
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-01 instance-028: i16={}, i32={}, i64={}",
        signed_i16_01028,
        signed_i32_01028,
        signed_i64_01028,
    );
}

#[inline(never)]
fn emit_01029(signed_i16_01029: i16, signed_i32_01029: i32, signed_i64_01029: i64) {
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-01 instance-029: i16={}, i32={}, i64={}",
        signed_i16_01029,
        signed_i32_01029,
        signed_i64_01029,
    );
}

pub(crate) fn emit() {
    emit_01020(-1620, -32020, -64020);
    emit_01021(-1621, -32021, -64021);
    emit_01022(-1622, -32022, -64022);
    emit_01023(-1623, -32023, -64023);
    emit_01024(-1624, -32024, -64024);
    emit_01025(-1625, -32025, -64025);
    emit_01026(-1626, -32026, -64026);
    emit_01027(-1627, -32027, -64027);
    emit_01028(-1628, -32028, -64028);
    emit_01029(-1629, -32029, -64029);
}

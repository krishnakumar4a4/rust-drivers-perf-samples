use crate::dummy_trace::*;

#[inline(never)]
fn emit_01070(signed_i16_01070: i16, signed_i32_01070: i32, signed_i64_01070: i64) {
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-01 instance-070: i16={}, i32={}, i64={}",
        signed_i16_01070,
        signed_i32_01070,
        signed_i64_01070,
    );
}

#[inline(never)]
fn emit_01071(signed_i16_01071: i16, signed_i32_01071: i32, signed_i64_01071: i64) {
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-01 instance-071: i16={}, i32={}, i64={}",
        signed_i16_01071,
        signed_i32_01071,
        signed_i64_01071,
    );
}

#[inline(never)]
fn emit_01072(signed_i16_01072: i16, signed_i32_01072: i32, signed_i64_01072: i64) {
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-01 instance-072: i16={}, i32={}, i64={}",
        signed_i16_01072,
        signed_i32_01072,
        signed_i64_01072,
    );
}

#[inline(never)]
fn emit_01073(signed_i16_01073: i16, signed_i32_01073: i32, signed_i64_01073: i64) {
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-01 instance-073: i16={}, i32={}, i64={}",
        signed_i16_01073,
        signed_i32_01073,
        signed_i64_01073,
    );
}

#[inline(never)]
fn emit_01074(signed_i16_01074: i16, signed_i32_01074: i32, signed_i64_01074: i64) {
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-01 instance-074: i16={}, i32={}, i64={}",
        signed_i16_01074,
        signed_i32_01074,
        signed_i64_01074,
    );
}

#[inline(never)]
fn emit_01075(signed_i16_01075: i16, signed_i32_01075: i32, signed_i64_01075: i64) {
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-01 instance-075: i16={}, i32={}, i64={}",
        signed_i16_01075,
        signed_i32_01075,
        signed_i64_01075,
    );
}

#[inline(never)]
fn emit_01076(signed_i16_01076: i16, signed_i32_01076: i32, signed_i64_01076: i64) {
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-01 instance-076: i16={}, i32={}, i64={}",
        signed_i16_01076,
        signed_i32_01076,
        signed_i64_01076,
    );
}

#[inline(never)]
fn emit_01077(signed_i16_01077: i16, signed_i32_01077: i32, signed_i64_01077: i64) {
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-01 instance-077: i16={}, i32={}, i64={}",
        signed_i16_01077,
        signed_i32_01077,
        signed_i64_01077,
    );
}

#[inline(never)]
fn emit_01078(signed_i16_01078: i16, signed_i32_01078: i32, signed_i64_01078: i64) {
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-01 instance-078: i16={}, i32={}, i64={}",
        signed_i16_01078,
        signed_i32_01078,
        signed_i64_01078,
    );
}

#[inline(never)]
fn emit_01079(signed_i16_01079: i16, signed_i32_01079: i32, signed_i64_01079: i64) {
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-01 instance-079: i16={}, i32={}, i64={}",
        signed_i16_01079,
        signed_i32_01079,
        signed_i64_01079,
    );
}

pub(crate) fn emit() {
    emit_01070(-1670, -32070, -64070);
    emit_01071(-1671, -32071, -64071);
    emit_01072(-1672, -32072, -64072);
    emit_01073(-1673, -32073, -64073);
    emit_01074(-1674, -32074, -64074);
    emit_01075(-1675, -32075, -64075);
    emit_01076(-1676, -32076, -64076);
    emit_01077(-1677, -32077, -64077);
    emit_01078(-1678, -32078, -64078);
    emit_01079(-1679, -32079, -64079);
}

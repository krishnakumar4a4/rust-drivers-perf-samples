use crate::dummy_trace::*;

#[inline(never)]
fn emit_01030(signed_i16_01030: i16, signed_i32_01030: i32, signed_i64_01030: i64) {
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-030: i16={}, i32={}, i64={}",
        signed_i16_01030,
        signed_i32_01030,
        signed_i64_01030,
    );
}

#[inline(never)]
fn emit_01031(signed_i16_01031: i16, signed_i32_01031: i32, signed_i64_01031: i64) {
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-031: i16={}, i32={}, i64={}",
        signed_i16_01031,
        signed_i32_01031,
        signed_i64_01031,
    );
}

#[inline(never)]
fn emit_01032(signed_i16_01032: i16, signed_i32_01032: i32, signed_i64_01032: i64) {
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-032: i16={}, i32={}, i64={}",
        signed_i16_01032,
        signed_i32_01032,
        signed_i64_01032,
    );
}

#[inline(never)]
fn emit_01033(signed_i16_01033: i16, signed_i32_01033: i32, signed_i64_01033: i64) {
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-033: i16={}, i32={}, i64={}",
        signed_i16_01033,
        signed_i32_01033,
        signed_i64_01033,
    );
}

#[inline(never)]
fn emit_01034(signed_i16_01034: i16, signed_i32_01034: i32, signed_i64_01034: i64) {
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-034: i16={}, i32={}, i64={}",
        signed_i16_01034,
        signed_i32_01034,
        signed_i64_01034,
    );
}

#[inline(never)]
fn emit_01035(signed_i16_01035: i16, signed_i32_01035: i32, signed_i64_01035: i64) {
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-01 instance-035: i16={}, i32={}, i64={}",
        signed_i16_01035,
        signed_i32_01035,
        signed_i64_01035,
    );
}

#[inline(never)]
fn emit_01036(signed_i16_01036: i16, signed_i32_01036: i32, signed_i64_01036: i64) {
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-01 instance-036: i16={}, i32={}, i64={}",
        signed_i16_01036,
        signed_i32_01036,
        signed_i64_01036,
    );
}

#[inline(never)]
fn emit_01037(signed_i16_01037: i16, signed_i32_01037: i32, signed_i64_01037: i64) {
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-01 instance-037: i16={}, i32={}, i64={}",
        signed_i16_01037,
        signed_i32_01037,
        signed_i64_01037,
    );
}

#[inline(never)]
fn emit_01038(signed_i16_01038: i16, signed_i32_01038: i32, signed_i64_01038: i64) {
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-01 instance-038: i16={}, i32={}, i64={}",
        signed_i16_01038,
        signed_i32_01038,
        signed_i64_01038,
    );
}

#[inline(never)]
fn emit_01039(signed_i16_01039: i16, signed_i32_01039: i32, signed_i64_01039: i64) {
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-01 instance-039: i16={}, i32={}, i64={}",
        signed_i16_01039,
        signed_i32_01039,
        signed_i64_01039,
    );
}

pub(crate) fn emit() {
    emit_01030(-1630, -32030, -64030);
    emit_01031(-1631, -32031, -64031);
    emit_01032(-1632, -32032, -64032);
    emit_01033(-1633, -32033, -64033);
    emit_01034(-1634, -32034, -64034);
    emit_01035(-1635, -32035, -64035);
    emit_01036(-1636, -32036, -64036);
    emit_01037(-1637, -32037, -64037);
    emit_01038(-1638, -32038, -64038);
    emit_01039(-1639, -32039, -64039);
}

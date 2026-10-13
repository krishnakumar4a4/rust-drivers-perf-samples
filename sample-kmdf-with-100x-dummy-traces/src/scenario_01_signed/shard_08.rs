use crate::dummy_trace::*;

#[inline(never)]
fn emit_01080(signed_i16_01080: i16, signed_i32_01080: i32, signed_i64_01080: i64) {
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-080: i16={}, i32={}, i64={}",
        signed_i16_01080,
        signed_i32_01080,
        signed_i64_01080,
    );
}

#[inline(never)]
fn emit_01081(signed_i16_01081: i16, signed_i32_01081: i32, signed_i64_01081: i64) {
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-081: i16={}, i32={}, i64={}",
        signed_i16_01081,
        signed_i32_01081,
        signed_i64_01081,
    );
}

#[inline(never)]
fn emit_01082(signed_i16_01082: i16, signed_i32_01082: i32, signed_i64_01082: i64) {
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-082: i16={}, i32={}, i64={}",
        signed_i16_01082,
        signed_i32_01082,
        signed_i64_01082,
    );
}

#[inline(never)]
fn emit_01083(signed_i16_01083: i16, signed_i32_01083: i32, signed_i64_01083: i64) {
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-083: i16={}, i32={}, i64={}",
        signed_i16_01083,
        signed_i32_01083,
        signed_i64_01083,
    );
}

#[inline(never)]
fn emit_01084(signed_i16_01084: i16, signed_i32_01084: i32, signed_i64_01084: i64) {
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-01 instance-084: i16={}, i32={}, i64={}",
        signed_i16_01084,
        signed_i32_01084,
        signed_i64_01084,
    );
}

#[inline(never)]
fn emit_01085(signed_i16_01085: i16, signed_i32_01085: i32, signed_i64_01085: i64) {
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-01 instance-085: i16={}, i32={}, i64={}",
        signed_i16_01085,
        signed_i32_01085,
        signed_i64_01085,
    );
}

#[inline(never)]
fn emit_01086(signed_i16_01086: i16, signed_i32_01086: i32, signed_i64_01086: i64) {
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-01 instance-086: i16={}, i32={}, i64={}",
        signed_i16_01086,
        signed_i32_01086,
        signed_i64_01086,
    );
}

#[inline(never)]
fn emit_01087(signed_i16_01087: i16, signed_i32_01087: i32, signed_i64_01087: i64) {
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-01 instance-087: i16={}, i32={}, i64={}",
        signed_i16_01087,
        signed_i32_01087,
        signed_i64_01087,
    );
}

#[inline(never)]
fn emit_01088(signed_i16_01088: i16, signed_i32_01088: i32, signed_i64_01088: i64) {
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-01 instance-088: i16={}, i32={}, i64={}",
        signed_i16_01088,
        signed_i32_01088,
        signed_i64_01088,
    );
}

#[inline(never)]
fn emit_01089(signed_i16_01089: i16, signed_i32_01089: i32, signed_i64_01089: i64) {
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-01 instance-089: i16={}, i32={}, i64={}",
        signed_i16_01089,
        signed_i32_01089,
        signed_i64_01089,
    );
}

pub(crate) fn emit() {
    emit_01080(-1680, -32080, -64080);
    emit_01081(-1681, -32081, -64081);
    emit_01082(-1682, -32082, -64082);
    emit_01083(-1683, -32083, -64083);
    emit_01084(-1684, -32084, -64084);
    emit_01085(-1685, -32085, -64085);
    emit_01086(-1686, -32086, -64086);
    emit_01087(-1687, -32087, -64087);
    emit_01088(-1688, -32088, -64088);
    emit_01089(-1689, -32089, -64089);
}

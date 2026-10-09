#[inline(never)]
fn emit_01000(signed_i16_01000: i16, signed_i32_01000: i32, signed_i64_01000: i64) {
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-100 scenario-01 instance-000: i16={}, i32={}, i64={}",
        signed_i16_01000,
        signed_i32_01000,
        signed_i64_01000
    );
}

#[inline(never)]
fn emit_01001(signed_i16_01001: i16, signed_i32_01001: i32, signed_i64_01001: i64) {
    trace!(
        WARNING,
        SIGNED,
        "fixture-100 scenario-01 instance-001: i16={}, i32={}, i64={}",
        signed_i16_01001,
        signed_i32_01001,
        signed_i64_01001
    );
}

#[inline(never)]
fn emit_01002(signed_i16_01002: i16, signed_i32_01002: i32, signed_i64_01002: i64) {
    trace!(
        ERROR,
        SIGNED,
        "fixture-100 scenario-01 instance-002: i16={}, i32={}, i64={}",
        signed_i16_01002,
        signed_i32_01002,
        signed_i64_01002
    );
}

#[inline(never)]
fn emit_01003(signed_i16_01003: i16, signed_i32_01003: i32, signed_i64_01003: i64) {
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-100 scenario-01 instance-003: i16={}, i32={}, i64={}",
        signed_i16_01003,
        signed_i32_01003,
        signed_i64_01003
    );
}

#[inline(never)]
fn emit_01004(signed_i16_01004: i16, signed_i32_01004: i32, signed_i64_01004: i64) {
    trace!(
        INFO,
        SIGNED,
        "fixture-100 scenario-01 instance-004: i16={}, i32={}, i64={}",
        signed_i16_01004,
        signed_i32_01004,
        signed_i64_01004
    );
}

#[inline(never)]
fn emit_01005(signed_i16_01005: i16, signed_i32_01005: i32, signed_i64_01005: i64) {
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-100 scenario-01 instance-005: i16={}, i32={}, i64={}",
        signed_i16_01005,
        signed_i32_01005,
        signed_i64_01005
    );
}

#[inline(never)]
fn emit_01006(signed_i16_01006: i16, signed_i32_01006: i32, signed_i64_01006: i64) {
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-100 scenario-01 instance-006: i16={}, i32={}, i64={}",
        signed_i16_01006,
        signed_i32_01006,
        signed_i64_01006
    );
}

#[inline(never)]
fn emit_01007(signed_i16_01007: i16, signed_i32_01007: i32, signed_i64_01007: i64) {
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-100 scenario-01 instance-007: i16={}, i32={}, i64={}",
        signed_i16_01007,
        signed_i32_01007,
        signed_i64_01007
    );
}

#[inline(never)]
fn emit_01008(signed_i16_01008: i16, signed_i32_01008: i32, signed_i64_01008: i64) {
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-100 scenario-01 instance-008: i16={}, i32={}, i64={}",
        signed_i16_01008,
        signed_i32_01008,
        signed_i64_01008
    );
}

#[inline(never)]
fn emit_01009(signed_i16_01009: i16, signed_i32_01009: i32, signed_i64_01009: i64) {
    trace!(
        INFO,
        UNSIGNED,
        "fixture-100 scenario-01 instance-009: i16={}, i32={}, i64={}",
        signed_i16_01009,
        signed_i32_01009,
        signed_i64_01009
    );
}

pub(crate) fn emit() {
    emit_01000(-1600, -32000, -64000);
    emit_01001(-1601, -32001, -64001);
    emit_01002(-1602, -32002, -64002);
    emit_01003(-1603, -32003, -64003);
    emit_01004(-1604, -32004, -64004);
    emit_01005(-1605, -32005, -64005);
    emit_01006(-1606, -32006, -64006);
    emit_01007(-1607, -32007, -64007);
    emit_01008(-1608, -32008, -64008);
    emit_01009(-1609, -32009, -64009);
}

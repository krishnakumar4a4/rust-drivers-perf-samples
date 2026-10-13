use crate::dummy_trace::*;

#[inline(never)]
fn emit_09000(mixed_u8_09000: u8, mixed_i16_09000: i16, mixed_usize_09000: usize) {
    dummy_trace(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-09 instance-000: u8={}, i16={}, usize={}",
        mixed_u8_09000,
        mixed_i16_09000,
        mixed_usize_09000,
    );
}

#[inline(never)]
fn emit_09001(mixed_u8_09001: u8, mixed_i16_09001: i16, mixed_usize_09001: usize) {
    dummy_trace(
        INFO,
        MIXED,
        "fixture-1000 scenario-09 instance-001: u8={}, i16={}, usize={}",
        mixed_u8_09001,
        mixed_i16_09001,
        mixed_usize_09001,
    );
}

#[inline(never)]
fn emit_09002(mixed_u8_09002: u8, mixed_i16_09002: i16, mixed_usize_09002: usize) {
    dummy_trace(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-09 instance-002: u8={}, i16={}, usize={}",
        mixed_u8_09002,
        mixed_i16_09002,
        mixed_usize_09002,
    );
}

#[inline(never)]
fn emit_09003(mixed_u8_09003: u8, mixed_i16_09003: i16, mixed_usize_09003: usize) {
    dummy_trace(
        WARNING,
        MIXED,
        "fixture-1000 scenario-09 instance-003: u8={}, i16={}, usize={}",
        mixed_u8_09003,
        mixed_i16_09003,
        mixed_usize_09003,
    );
}

#[inline(never)]
fn emit_09004(mixed_u8_09004: u8, mixed_i16_09004: i16, mixed_usize_09004: usize) {
    dummy_trace(
        ERROR,
        MIXED,
        "fixture-1000 scenario-09 instance-004: u8={}, i16={}, usize={}",
        mixed_u8_09004,
        mixed_i16_09004,
        mixed_usize_09004,
    );
}

#[inline(never)]
fn emit_09005(mixed_u8_09005: u8, mixed_i16_09005: i16, mixed_usize_09005: usize) {
    dummy_trace(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-09 instance-005: u8={}, i16={}, usize={}",
        mixed_u8_09005,
        mixed_i16_09005,
        mixed_usize_09005,
    );
}

#[inline(never)]
fn emit_09006(mixed_u8_09006: u8, mixed_i16_09006: i16, mixed_usize_09006: usize) {
    dummy_trace(
        INFO,
        STATUS,
        "fixture-1000 scenario-09 instance-006: u8={}, i16={}, usize={}",
        mixed_u8_09006,
        mixed_i16_09006,
        mixed_usize_09006,
    );
}

#[inline(never)]
fn emit_09007(mixed_u8_09007: u8, mixed_i16_09007: i16, mixed_usize_09007: usize) {
    dummy_trace(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-09 instance-007: u8={}, i16={}, usize={}",
        mixed_u8_09007,
        mixed_i16_09007,
        mixed_usize_09007,
    );
}

#[inline(never)]
fn emit_09008(mixed_u8_09008: u8, mixed_i16_09008: i16, mixed_usize_09008: usize) {
    dummy_trace(
        WARNING,
        STATUS,
        "fixture-1000 scenario-09 instance-008: u8={}, i16={}, usize={}",
        mixed_u8_09008,
        mixed_i16_09008,
        mixed_usize_09008,
    );
}

#[inline(never)]
fn emit_09009(mixed_u8_09009: u8, mixed_i16_09009: i16, mixed_usize_09009: usize) {
    dummy_trace(
        ERROR,
        STATUS,
        "fixture-1000 scenario-09 instance-009: u8={}, i16={}, usize={}",
        mixed_u8_09009,
        mixed_i16_09009,
        mixed_usize_09009,
    );
}

pub(crate) fn emit() {
    emit_09000(90, -900, 9000);
    emit_09001(91, -901, 9001);
    emit_09002(92, -902, 9002);
    emit_09003(93, -903, 9003);
    emit_09004(94, -904, 9004);
    emit_09005(95, -905, 9005);
    emit_09006(96, -906, 9006);
    emit_09007(97, -907, 9007);
    emit_09008(98, -908, 9008);
    emit_09009(99, -909, 9009);
}

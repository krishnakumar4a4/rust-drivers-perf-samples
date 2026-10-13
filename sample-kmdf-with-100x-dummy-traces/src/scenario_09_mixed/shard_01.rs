use crate::dummy_trace::*;

#[inline(never)]
fn emit_09010(mixed_u8_09010: u8, mixed_i16_09010: i16, mixed_usize_09010: usize) {
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-010: u8={}, i16={}, usize={}",
        mixed_u8_09010,
        mixed_i16_09010,
        mixed_usize_09010,
    );
}

#[inline(never)]
fn emit_09011(mixed_u8_09011: u8, mixed_i16_09011: i16, mixed_usize_09011: usize) {
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-011: u8={}, i16={}, usize={}",
        mixed_u8_09011,
        mixed_i16_09011,
        mixed_usize_09011,
    );
}

#[inline(never)]
fn emit_09012(mixed_u8_09012: u8, mixed_i16_09012: i16, mixed_usize_09012: usize) {
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-012: u8={}, i16={}, usize={}",
        mixed_u8_09012,
        mixed_i16_09012,
        mixed_usize_09012,
    );
}

#[inline(never)]
fn emit_09013(mixed_u8_09013: u8, mixed_i16_09013: i16, mixed_usize_09013: usize) {
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-013: u8={}, i16={}, usize={}",
        mixed_u8_09013,
        mixed_i16_09013,
        mixed_usize_09013,
    );
}

#[inline(never)]
fn emit_09014(mixed_u8_09014: u8, mixed_i16_09014: i16, mixed_usize_09014: usize) {
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-014: u8={}, i16={}, usize={}",
        mixed_u8_09014,
        mixed_i16_09014,
        mixed_usize_09014,
    );
}

#[inline(never)]
fn emit_09015(mixed_u8_09015: u8, mixed_i16_09015: i16, mixed_usize_09015: usize) {
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-09 instance-015: u8={}, i16={}, usize={}",
        mixed_u8_09015,
        mixed_i16_09015,
        mixed_usize_09015,
    );
}

#[inline(never)]
fn emit_09016(mixed_u8_09016: u8, mixed_i16_09016: i16, mixed_usize_09016: usize) {
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-09 instance-016: u8={}, i16={}, usize={}",
        mixed_u8_09016,
        mixed_i16_09016,
        mixed_usize_09016,
    );
}

#[inline(never)]
fn emit_09017(mixed_u8_09017: u8, mixed_i16_09017: i16, mixed_usize_09017: usize) {
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-09 instance-017: u8={}, i16={}, usize={}",
        mixed_u8_09017,
        mixed_i16_09017,
        mixed_usize_09017,
    );
}

#[inline(never)]
fn emit_09018(mixed_u8_09018: u8, mixed_i16_09018: i16, mixed_usize_09018: usize) {
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-09 instance-018: u8={}, i16={}, usize={}",
        mixed_u8_09018,
        mixed_i16_09018,
        mixed_usize_09018,
    );
}

#[inline(never)]
fn emit_09019(mixed_u8_09019: u8, mixed_i16_09019: i16, mixed_usize_09019: usize) {
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-09 instance-019: u8={}, i16={}, usize={}",
        mixed_u8_09019,
        mixed_i16_09019,
        mixed_usize_09019,
    );
}

pub(crate) fn emit() {
    emit_09010(100, -910, 9010);
    emit_09011(101, -911, 9011);
    emit_09012(102, -912, 9012);
    emit_09013(103, -913, 9013);
    emit_09014(104, -914, 9014);
    emit_09015(105, -915, 9015);
    emit_09016(106, -916, 9016);
    emit_09017(107, -917, 9017);
    emit_09018(108, -918, 9018);
    emit_09019(109, -919, 9019);
}

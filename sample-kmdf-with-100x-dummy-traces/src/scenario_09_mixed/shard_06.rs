use crate::dummy_trace::*;

#[inline(never)]
fn emit_09060(mixed_u8_09060: u8, mixed_i16_09060: i16, mixed_usize_09060: usize) {
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-060: u8={}, i16={}, usize={}",
        mixed_u8_09060,
        mixed_i16_09060,
        mixed_usize_09060,
    );
}

#[inline(never)]
fn emit_09061(mixed_u8_09061: u8, mixed_i16_09061: i16, mixed_usize_09061: usize) {
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-061: u8={}, i16={}, usize={}",
        mixed_u8_09061,
        mixed_i16_09061,
        mixed_usize_09061,
    );
}

#[inline(never)]
fn emit_09062(mixed_u8_09062: u8, mixed_i16_09062: i16, mixed_usize_09062: usize) {
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-062: u8={}, i16={}, usize={}",
        mixed_u8_09062,
        mixed_i16_09062,
        mixed_usize_09062,
    );
}

#[inline(never)]
fn emit_09063(mixed_u8_09063: u8, mixed_i16_09063: i16, mixed_usize_09063: usize) {
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-063: u8={}, i16={}, usize={}",
        mixed_u8_09063,
        mixed_i16_09063,
        mixed_usize_09063,
    );
}

#[inline(never)]
fn emit_09064(mixed_u8_09064: u8, mixed_i16_09064: i16, mixed_usize_09064: usize) {
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-09 instance-064: u8={}, i16={}, usize={}",
        mixed_u8_09064,
        mixed_i16_09064,
        mixed_usize_09064,
    );
}

#[inline(never)]
fn emit_09065(mixed_u8_09065: u8, mixed_i16_09065: i16, mixed_usize_09065: usize) {
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-09 instance-065: u8={}, i16={}, usize={}",
        mixed_u8_09065,
        mixed_i16_09065,
        mixed_usize_09065,
    );
}

#[inline(never)]
fn emit_09066(mixed_u8_09066: u8, mixed_i16_09066: i16, mixed_usize_09066: usize) {
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-09 instance-066: u8={}, i16={}, usize={}",
        mixed_u8_09066,
        mixed_i16_09066,
        mixed_usize_09066,
    );
}

#[inline(never)]
fn emit_09067(mixed_u8_09067: u8, mixed_i16_09067: i16, mixed_usize_09067: usize) {
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-09 instance-067: u8={}, i16={}, usize={}",
        mixed_u8_09067,
        mixed_i16_09067,
        mixed_usize_09067,
    );
}

#[inline(never)]
fn emit_09068(mixed_u8_09068: u8, mixed_i16_09068: i16, mixed_usize_09068: usize) {
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-09 instance-068: u8={}, i16={}, usize={}",
        mixed_u8_09068,
        mixed_i16_09068,
        mixed_usize_09068,
    );
}

#[inline(never)]
fn emit_09069(mixed_u8_09069: u8, mixed_i16_09069: i16, mixed_usize_09069: usize) {
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-09 instance-069: u8={}, i16={}, usize={}",
        mixed_u8_09069,
        mixed_i16_09069,
        mixed_usize_09069,
    );
}

pub(crate) fn emit() {
    emit_09060(150, -960, 9060);
    emit_09061(151, -961, 9061);
    emit_09062(152, -962, 9062);
    emit_09063(153, -963, 9063);
    emit_09064(154, -964, 9064);
    emit_09065(155, -965, 9065);
    emit_09066(156, -966, 9066);
    emit_09067(157, -967, 9067);
    emit_09068(158, -968, 9068);
    emit_09069(159, -969, 9069);
}

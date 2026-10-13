use crate::dummy_trace::*;

#[inline(never)]
fn emit_09070(mixed_u8_09070: u8, mixed_i16_09070: i16, mixed_usize_09070: usize) {
    dummy_trace(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-09 instance-070: u8={}, i16={}, usize={}",
        mixed_u8_09070,
        mixed_i16_09070,
        mixed_usize_09070,
    );
}

#[inline(never)]
fn emit_09071(mixed_u8_09071: u8, mixed_i16_09071: i16, mixed_usize_09071: usize) {
    dummy_trace(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-09 instance-071: u8={}, i16={}, usize={}",
        mixed_u8_09071,
        mixed_i16_09071,
        mixed_usize_09071,
    );
}

#[inline(never)]
fn emit_09072(mixed_u8_09072: u8, mixed_i16_09072: i16, mixed_usize_09072: usize) {
    dummy_trace(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-09 instance-072: u8={}, i16={}, usize={}",
        mixed_u8_09072,
        mixed_i16_09072,
        mixed_usize_09072,
    );
}

#[inline(never)]
fn emit_09073(mixed_u8_09073: u8, mixed_i16_09073: i16, mixed_usize_09073: usize) {
    dummy_trace(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-09 instance-073: u8={}, i16={}, usize={}",
        mixed_u8_09073,
        mixed_i16_09073,
        mixed_usize_09073,
    );
}

#[inline(never)]
fn emit_09074(mixed_u8_09074: u8, mixed_i16_09074: i16, mixed_usize_09074: usize) {
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-09 instance-074: u8={}, i16={}, usize={}",
        mixed_u8_09074,
        mixed_i16_09074,
        mixed_usize_09074,
    );
}

#[inline(never)]
fn emit_09075(mixed_u8_09075: u8, mixed_i16_09075: i16, mixed_usize_09075: usize) {
    dummy_trace(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-09 instance-075: u8={}, i16={}, usize={}",
        mixed_u8_09075,
        mixed_i16_09075,
        mixed_usize_09075,
    );
}

#[inline(never)]
fn emit_09076(mixed_u8_09076: u8, mixed_i16_09076: i16, mixed_usize_09076: usize) {
    dummy_trace(
        INFO,
        MIXED,
        "fixture-1000 scenario-09 instance-076: u8={}, i16={}, usize={}",
        mixed_u8_09076,
        mixed_i16_09076,
        mixed_usize_09076,
    );
}

#[inline(never)]
fn emit_09077(mixed_u8_09077: u8, mixed_i16_09077: i16, mixed_usize_09077: usize) {
    dummy_trace(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-09 instance-077: u8={}, i16={}, usize={}",
        mixed_u8_09077,
        mixed_i16_09077,
        mixed_usize_09077,
    );
}

#[inline(never)]
fn emit_09078(mixed_u8_09078: u8, mixed_i16_09078: i16, mixed_usize_09078: usize) {
    dummy_trace(
        WARNING,
        MIXED,
        "fixture-1000 scenario-09 instance-078: u8={}, i16={}, usize={}",
        mixed_u8_09078,
        mixed_i16_09078,
        mixed_usize_09078,
    );
}

#[inline(never)]
fn emit_09079(mixed_u8_09079: u8, mixed_i16_09079: i16, mixed_usize_09079: usize) {
    dummy_trace(
        ERROR,
        MIXED,
        "fixture-1000 scenario-09 instance-079: u8={}, i16={}, usize={}",
        mixed_u8_09079,
        mixed_i16_09079,
        mixed_usize_09079,
    );
}

pub(crate) fn emit() {
    emit_09070(160, -970, 9070);
    emit_09071(161, -971, 9071);
    emit_09072(162, -972, 9072);
    emit_09073(163, -973, 9073);
    emit_09074(164, -974, 9074);
    emit_09075(165, -975, 9075);
    emit_09076(166, -976, 9076);
    emit_09077(167, -977, 9077);
    emit_09078(168, -978, 9078);
    emit_09079(169, -979, 9079);
}

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08020() {
    let implicit_state_08020 = ImplicitState::Ready;
    let mixed_state_08020 = MixedState::Failed;
    let returned_i32_08020 = returned_i32(-8020);
    trace!(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-08 instance-020: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08020,
        mixed_state_08020,
        returned_i32_08020
    );
}

#[inline(never)]
fn emit_08021() {
    let implicit_state_08021 = ImplicitState::Unknown;
    let mixed_state_08021 = MixedState::Unknown;
    let returned_i32_08021 = returned_i32(-8021);
    trace!(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-08 instance-021: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08021,
        mixed_state_08021,
        returned_i32_08021
    );
}

#[inline(never)]
fn emit_08022() {
    let implicit_state_08022 = ImplicitState::Starting;
    let mixed_state_08022 = MixedState::Warning;
    let returned_i32_08022 = returned_i32(-8022);
    trace!(
        INFO,
        GENERIC,
        "fixture-1000 scenario-08 instance-022: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08022,
        mixed_state_08022,
        returned_i32_08022
    );
}

#[inline(never)]
fn emit_08023() {
    let implicit_state_08023 = ImplicitState::Ready;
    let mixed_state_08023 = MixedState::Failed;
    let returned_i32_08023 = returned_i32(-8023);
    trace!(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-08 instance-023: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08023,
        mixed_state_08023,
        returned_i32_08023
    );
}

#[inline(never)]
fn emit_08024() {
    let implicit_state_08024 = ImplicitState::Unknown;
    let mixed_state_08024 = MixedState::Unknown;
    let returned_i32_08024 = returned_i32(-8024);
    trace!(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-08 instance-024: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08024,
        mixed_state_08024,
        returned_i32_08024
    );
}

#[inline(never)]
fn emit_08025() {
    let implicit_state_08025 = ImplicitState::Starting;
    let mixed_state_08025 = MixedState::Warning;
    let returned_i32_08025 = returned_i32(-8025);
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-025: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08025,
        mixed_state_08025,
        returned_i32_08025
    );
}

#[inline(never)]
fn emit_08026() {
    let implicit_state_08026 = ImplicitState::Ready;
    let mixed_state_08026 = MixedState::Failed;
    let returned_i32_08026 = returned_i32(-8026);
    trace!(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-026: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08026,
        mixed_state_08026,
        returned_i32_08026
    );
}

#[inline(never)]
fn emit_08027() {
    let implicit_state_08027 = ImplicitState::Unknown;
    let mixed_state_08027 = MixedState::Unknown;
    let returned_i32_08027 = returned_i32(-8027);
    trace!(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-027: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08027,
        mixed_state_08027,
        returned_i32_08027
    );
}

#[inline(never)]
fn emit_08028() {
    let implicit_state_08028 = ImplicitState::Starting;
    let mixed_state_08028 = MixedState::Warning;
    let returned_i32_08028 = returned_i32(-8028);
    trace!(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-028: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08028,
        mixed_state_08028,
        returned_i32_08028
    );
}

#[inline(never)]
fn emit_08029() {
    let implicit_state_08029 = ImplicitState::Ready;
    let mixed_state_08029 = MixedState::Failed;
    let returned_i32_08029 = returned_i32(-8029);
    trace!(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-029: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08029,
        mixed_state_08029,
        returned_i32_08029
    );
}

pub(crate) fn emit() {
    emit_08020();
    emit_08021();
    emit_08022();
    emit_08023();
    emit_08024();
    emit_08025();
    emit_08026();
    emit_08027();
    emit_08028();
    emit_08029();
}

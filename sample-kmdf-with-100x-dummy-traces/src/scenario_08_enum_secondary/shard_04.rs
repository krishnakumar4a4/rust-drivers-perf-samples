use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08040() {
    let implicit_state_08040 = ImplicitState::Starting;
    let mixed_state_08040 = MixedState::Warning;
    let returned_i32_08040 = returned_i32(-8040);
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-040: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08040,
        mixed_state_08040,
        returned_i32_08040,
    );
}

#[inline(never)]
fn emit_08041() {
    let implicit_state_08041 = ImplicitState::Ready;
    let mixed_state_08041 = MixedState::Failed;
    let returned_i32_08041 = returned_i32(-8041);
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-041: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08041,
        mixed_state_08041,
        returned_i32_08041,
    );
}

#[inline(never)]
fn emit_08042() {
    let implicit_state_08042 = ImplicitState::Unknown;
    let mixed_state_08042 = MixedState::Unknown;
    let returned_i32_08042 = returned_i32(-8042);
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-042: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08042,
        mixed_state_08042,
        returned_i32_08042,
    );
}

#[inline(never)]
fn emit_08043() {
    let implicit_state_08043 = ImplicitState::Starting;
    let mixed_state_08043 = MixedState::Warning;
    let returned_i32_08043 = returned_i32(-8043);
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-043: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08043,
        mixed_state_08043,
        returned_i32_08043,
    );
}

#[inline(never)]
fn emit_08044() {
    let implicit_state_08044 = ImplicitState::Ready;
    let mixed_state_08044 = MixedState::Failed;
    let returned_i32_08044 = returned_i32(-8044);
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-044: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08044,
        mixed_state_08044,
        returned_i32_08044,
    );
}

#[inline(never)]
fn emit_08045() {
    let implicit_state_08045 = ImplicitState::Unknown;
    let mixed_state_08045 = MixedState::Unknown;
    let returned_i32_08045 = returned_i32(-8045);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-08 instance-045: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08045,
        mixed_state_08045,
        returned_i32_08045,
    );
}

#[inline(never)]
fn emit_08046() {
    let implicit_state_08046 = ImplicitState::Starting;
    let mixed_state_08046 = MixedState::Warning;
    let returned_i32_08046 = returned_i32(-8046);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-08 instance-046: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08046,
        mixed_state_08046,
        returned_i32_08046,
    );
}

#[inline(never)]
fn emit_08047() {
    let implicit_state_08047 = ImplicitState::Ready;
    let mixed_state_08047 = MixedState::Failed;
    let returned_i32_08047 = returned_i32(-8047);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-08 instance-047: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08047,
        mixed_state_08047,
        returned_i32_08047,
    );
}

#[inline(never)]
fn emit_08048() {
    let implicit_state_08048 = ImplicitState::Unknown;
    let mixed_state_08048 = MixedState::Unknown;
    let returned_i32_08048 = returned_i32(-8048);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-08 instance-048: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08048,
        mixed_state_08048,
        returned_i32_08048,
    );
}

#[inline(never)]
fn emit_08049() {
    let implicit_state_08049 = ImplicitState::Starting;
    let mixed_state_08049 = MixedState::Warning;
    let returned_i32_08049 = returned_i32(-8049);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-08 instance-049: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08049,
        mixed_state_08049,
        returned_i32_08049,
    );
}

pub(crate) fn emit() {
    emit_08040();
    emit_08041();
    emit_08042();
    emit_08043();
    emit_08044();
    emit_08045();
    emit_08046();
    emit_08047();
    emit_08048();
    emit_08049();
}

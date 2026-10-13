use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08090() {
    let implicit_state_08090 = ImplicitState::Unknown;
    let mixed_state_08090 = MixedState::Unknown;
    let returned_i32_08090 = returned_i32(-8090);
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-090: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08090,
        mixed_state_08090,
        returned_i32_08090,
    );
}

#[inline(never)]
fn emit_08091() {
    let implicit_state_08091 = ImplicitState::Starting;
    let mixed_state_08091 = MixedState::Warning;
    let returned_i32_08091 = returned_i32(-8091);
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-091: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08091,
        mixed_state_08091,
        returned_i32_08091,
    );
}

#[inline(never)]
fn emit_08092() {
    let implicit_state_08092 = ImplicitState::Ready;
    let mixed_state_08092 = MixedState::Failed;
    let returned_i32_08092 = returned_i32(-8092);
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-092: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08092,
        mixed_state_08092,
        returned_i32_08092,
    );
}

#[inline(never)]
fn emit_08093() {
    let implicit_state_08093 = ImplicitState::Unknown;
    let mixed_state_08093 = MixedState::Unknown;
    let returned_i32_08093 = returned_i32(-8093);
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-093: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08093,
        mixed_state_08093,
        returned_i32_08093,
    );
}

#[inline(never)]
fn emit_08094() {
    let implicit_state_08094 = ImplicitState::Starting;
    let mixed_state_08094 = MixedState::Warning;
    let returned_i32_08094 = returned_i32(-8094);
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-094: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08094,
        mixed_state_08094,
        returned_i32_08094,
    );
}

#[inline(never)]
fn emit_08095() {
    let implicit_state_08095 = ImplicitState::Ready;
    let mixed_state_08095 = MixedState::Failed;
    let returned_i32_08095 = returned_i32(-8095);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-08 instance-095: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08095,
        mixed_state_08095,
        returned_i32_08095,
    );
}

#[inline(never)]
fn emit_08096() {
    let implicit_state_08096 = ImplicitState::Unknown;
    let mixed_state_08096 = MixedState::Unknown;
    let returned_i32_08096 = returned_i32(-8096);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-08 instance-096: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08096,
        mixed_state_08096,
        returned_i32_08096,
    );
}

#[inline(never)]
fn emit_08097() {
    let implicit_state_08097 = ImplicitState::Starting;
    let mixed_state_08097 = MixedState::Warning;
    let returned_i32_08097 = returned_i32(-8097);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-08 instance-097: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08097,
        mixed_state_08097,
        returned_i32_08097,
    );
}

#[inline(never)]
fn emit_08098() {
    let implicit_state_08098 = ImplicitState::Ready;
    let mixed_state_08098 = MixedState::Failed;
    let returned_i32_08098 = returned_i32(-8098);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-08 instance-098: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08098,
        mixed_state_08098,
        returned_i32_08098,
    );
}

#[inline(never)]
fn emit_08099() {
    let implicit_state_08099 = ImplicitState::Unknown;
    let mixed_state_08099 = MixedState::Unknown;
    let returned_i32_08099 = returned_i32(-8099);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-08 instance-099: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08099,
        mixed_state_08099,
        returned_i32_08099,
    );
}

pub(crate) fn emit() {
    emit_08090();
    emit_08091();
    emit_08092();
    emit_08093();
    emit_08094();
    emit_08095();
    emit_08096();
    emit_08097();
    emit_08098();
    emit_08099();
}

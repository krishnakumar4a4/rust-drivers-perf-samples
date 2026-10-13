use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08070() {
    let implicit_state_08070 = ImplicitState::Starting;
    let mixed_state_08070 = MixedState::Warning;
    let returned_i32_08070 = returned_i32(-8070);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-08 instance-070: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08070,
        mixed_state_08070,
        returned_i32_08070,
    );
}

#[inline(never)]
fn emit_08071() {
    let implicit_state_08071 = ImplicitState::Ready;
    let mixed_state_08071 = MixedState::Failed;
    let returned_i32_08071 = returned_i32(-8071);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-08 instance-071: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08071,
        mixed_state_08071,
        returned_i32_08071,
    );
}

#[inline(never)]
fn emit_08072() {
    let implicit_state_08072 = ImplicitState::Unknown;
    let mixed_state_08072 = MixedState::Unknown;
    let returned_i32_08072 = returned_i32(-8072);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-08 instance-072: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08072,
        mixed_state_08072,
        returned_i32_08072,
    );
}

#[inline(never)]
fn emit_08073() {
    let implicit_state_08073 = ImplicitState::Starting;
    let mixed_state_08073 = MixedState::Warning;
    let returned_i32_08073 = returned_i32(-8073);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-08 instance-073: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08073,
        mixed_state_08073,
        returned_i32_08073,
    );
}

#[inline(never)]
fn emit_08074() {
    let implicit_state_08074 = ImplicitState::Ready;
    let mixed_state_08074 = MixedState::Failed;
    let returned_i32_08074 = returned_i32(-8074);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-08 instance-074: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08074,
        mixed_state_08074,
        returned_i32_08074,
    );
}

#[inline(never)]
fn emit_08075() {
    let implicit_state_08075 = ImplicitState::Unknown;
    let mixed_state_08075 = MixedState::Unknown;
    let returned_i32_08075 = returned_i32(-8075);
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-075: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08075,
        mixed_state_08075,
        returned_i32_08075,
    );
}

#[inline(never)]
fn emit_08076() {
    let implicit_state_08076 = ImplicitState::Starting;
    let mixed_state_08076 = MixedState::Warning;
    let returned_i32_08076 = returned_i32(-8076);
    dummy_trace(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-076: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08076,
        mixed_state_08076,
        returned_i32_08076,
    );
}

#[inline(never)]
fn emit_08077() {
    let implicit_state_08077 = ImplicitState::Ready;
    let mixed_state_08077 = MixedState::Failed;
    let returned_i32_08077 = returned_i32(-8077);
    dummy_trace(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-077: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08077,
        mixed_state_08077,
        returned_i32_08077,
    );
}

#[inline(never)]
fn emit_08078() {
    let implicit_state_08078 = ImplicitState::Unknown;
    let mixed_state_08078 = MixedState::Unknown;
    let returned_i32_08078 = returned_i32(-8078);
    dummy_trace(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-078: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08078,
        mixed_state_08078,
        returned_i32_08078,
    );
}

#[inline(never)]
fn emit_08079() {
    let implicit_state_08079 = ImplicitState::Starting;
    let mixed_state_08079 = MixedState::Warning;
    let returned_i32_08079 = returned_i32(-8079);
    dummy_trace(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-079: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08079,
        mixed_state_08079,
        returned_i32_08079,
    );
}

pub(crate) fn emit() {
    emit_08070();
    emit_08071();
    emit_08072();
    emit_08073();
    emit_08074();
    emit_08075();
    emit_08076();
    emit_08077();
    emit_08078();
    emit_08079();
}

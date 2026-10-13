use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08010() {
    let implicit_state_08010 = ImplicitState::Starting;
    let mixed_state_08010 = MixedState::Warning;
    let returned_i32_08010 = returned_i32(-8010);
    dummy_trace(
        ERROR,
        STATUS,
        "fixture-1000 scenario-08 instance-010: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08010,
        mixed_state_08010,
        returned_i32_08010,
    );
}

#[inline(never)]
fn emit_08011() {
    let implicit_state_08011 = ImplicitState::Ready;
    let mixed_state_08011 = MixedState::Failed;
    let returned_i32_08011 = returned_i32(-8011);
    dummy_trace(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-08 instance-011: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08011,
        mixed_state_08011,
        returned_i32_08011,
    );
}

#[inline(never)]
fn emit_08012() {
    let implicit_state_08012 = ImplicitState::Unknown;
    let mixed_state_08012 = MixedState::Unknown;
    let returned_i32_08012 = returned_i32(-8012);
    dummy_trace(
        INFO,
        STATUS,
        "fixture-1000 scenario-08 instance-012: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08012,
        mixed_state_08012,
        returned_i32_08012,
    );
}

#[inline(never)]
fn emit_08013() {
    let implicit_state_08013 = ImplicitState::Starting;
    let mixed_state_08013 = MixedState::Warning;
    let returned_i32_08013 = returned_i32(-8013);
    dummy_trace(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-08 instance-013: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08013,
        mixed_state_08013,
        returned_i32_08013,
    );
}

#[inline(never)]
fn emit_08014() {
    let implicit_state_08014 = ImplicitState::Ready;
    let mixed_state_08014 = MixedState::Failed;
    let returned_i32_08014 = returned_i32(-8014);
    dummy_trace(
        WARNING,
        STATUS,
        "fixture-1000 scenario-08 instance-014: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08014,
        mixed_state_08014,
        returned_i32_08014,
    );
}

#[inline(never)]
fn emit_08015() {
    let implicit_state_08015 = ImplicitState::Unknown;
    let mixed_state_08015 = MixedState::Unknown;
    let returned_i32_08015 = returned_i32(-8015);
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-015: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08015,
        mixed_state_08015,
        returned_i32_08015,
    );
}

#[inline(never)]
fn emit_08016() {
    let implicit_state_08016 = ImplicitState::Starting;
    let mixed_state_08016 = MixedState::Warning;
    let returned_i32_08016 = returned_i32(-8016);
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-016: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08016,
        mixed_state_08016,
        returned_i32_08016,
    );
}

#[inline(never)]
fn emit_08017() {
    let implicit_state_08017 = ImplicitState::Ready;
    let mixed_state_08017 = MixedState::Failed;
    let returned_i32_08017 = returned_i32(-8017);
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-017: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08017,
        mixed_state_08017,
        returned_i32_08017,
    );
}

#[inline(never)]
fn emit_08018() {
    let implicit_state_08018 = ImplicitState::Unknown;
    let mixed_state_08018 = MixedState::Unknown;
    let returned_i32_08018 = returned_i32(-8018);
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-018: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08018,
        mixed_state_08018,
        returned_i32_08018,
    );
}

#[inline(never)]
fn emit_08019() {
    let implicit_state_08019 = ImplicitState::Starting;
    let mixed_state_08019 = MixedState::Warning;
    let returned_i32_08019 = returned_i32(-8019);
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-019: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08019,
        mixed_state_08019,
        returned_i32_08019,
    );
}

pub(crate) fn emit() {
    emit_08010();
    emit_08011();
    emit_08012();
    emit_08013();
    emit_08014();
    emit_08015();
    emit_08016();
    emit_08017();
    emit_08018();
    emit_08019();
}

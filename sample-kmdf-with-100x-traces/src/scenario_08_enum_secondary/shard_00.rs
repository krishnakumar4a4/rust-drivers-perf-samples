use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08000() {
    let implicit_state_08000 = ImplicitState::Unknown;
    let mixed_state_08000 = MixedState::Unknown;
    let returned_i32_08000 = returned_i32(-8000);
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-000: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08000,
        mixed_state_08000,
        returned_i32_08000
    );
}

#[inline(never)]
fn emit_08001() {
    let implicit_state_08001 = ImplicitState::Starting;
    let mixed_state_08001 = MixedState::Warning;
    let returned_i32_08001 = returned_i32(-8001);
    trace!(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-001: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08001,
        mixed_state_08001,
        returned_i32_08001
    );
}

#[inline(never)]
fn emit_08002() {
    let implicit_state_08002 = ImplicitState::Ready;
    let mixed_state_08002 = MixedState::Failed;
    let returned_i32_08002 = returned_i32(-8002);
    trace!(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-002: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08002,
        mixed_state_08002,
        returned_i32_08002
    );
}

#[inline(never)]
fn emit_08003() {
    let implicit_state_08003 = ImplicitState::Unknown;
    let mixed_state_08003 = MixedState::Unknown;
    let returned_i32_08003 = returned_i32(-8003);
    trace!(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-003: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08003,
        mixed_state_08003,
        returned_i32_08003
    );
}

#[inline(never)]
fn emit_08004() {
    let implicit_state_08004 = ImplicitState::Starting;
    let mixed_state_08004 = MixedState::Warning;
    let returned_i32_08004 = returned_i32(-8004);
    trace!(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-004: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08004,
        mixed_state_08004,
        returned_i32_08004
    );
}

#[inline(never)]
fn emit_08005() {
    let implicit_state_08005 = ImplicitState::Ready;
    let mixed_state_08005 = MixedState::Failed;
    let returned_i32_08005 = returned_i32(-8005);
    trace!(
        ERROR,
        MIXED,
        "fixture-1000 scenario-08 instance-005: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08005,
        mixed_state_08005,
        returned_i32_08005
    );
}

#[inline(never)]
fn emit_08006() {
    let implicit_state_08006 = ImplicitState::Unknown;
    let mixed_state_08006 = MixedState::Unknown;
    let returned_i32_08006 = returned_i32(-8006);
    trace!(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-08 instance-006: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08006,
        mixed_state_08006,
        returned_i32_08006
    );
}

#[inline(never)]
fn emit_08007() {
    let implicit_state_08007 = ImplicitState::Starting;
    let mixed_state_08007 = MixedState::Warning;
    let returned_i32_08007 = returned_i32(-8007);
    trace!(
        INFO,
        MIXED,
        "fixture-1000 scenario-08 instance-007: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08007,
        mixed_state_08007,
        returned_i32_08007
    );
}

#[inline(never)]
fn emit_08008() {
    let implicit_state_08008 = ImplicitState::Ready;
    let mixed_state_08008 = MixedState::Failed;
    let returned_i32_08008 = returned_i32(-8008);
    trace!(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-08 instance-008: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08008,
        mixed_state_08008,
        returned_i32_08008
    );
}

#[inline(never)]
fn emit_08009() {
    let implicit_state_08009 = ImplicitState::Unknown;
    let mixed_state_08009 = MixedState::Unknown;
    let returned_i32_08009 = returned_i32(-8009);
    trace!(
        WARNING,
        MIXED,
        "fixture-1000 scenario-08 instance-009: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08009,
        mixed_state_08009,
        returned_i32_08009
    );
}

pub(crate) fn emit() {
    emit_08000();
    emit_08001();
    emit_08002();
    emit_08003();
    emit_08004();
    emit_08005();
    emit_08006();
    emit_08007();
    emit_08008();
    emit_08009();
}

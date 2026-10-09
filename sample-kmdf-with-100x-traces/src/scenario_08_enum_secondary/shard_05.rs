use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08050() {
    let implicit_state_08050 = ImplicitState::Ready;
    let mixed_state_08050 = MixedState::Failed;
    let returned_i32_08050 = returned_i32(-8050);
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-050: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08050,
        mixed_state_08050,
        returned_i32_08050
    );
}

#[inline(never)]
fn emit_08051() {
    let implicit_state_08051 = ImplicitState::Unknown;
    let mixed_state_08051 = MixedState::Unknown;
    let returned_i32_08051 = returned_i32(-8051);
    trace!(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-051: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08051,
        mixed_state_08051,
        returned_i32_08051
    );
}

#[inline(never)]
fn emit_08052() {
    let implicit_state_08052 = ImplicitState::Starting;
    let mixed_state_08052 = MixedState::Warning;
    let returned_i32_08052 = returned_i32(-8052);
    trace!(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-052: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08052,
        mixed_state_08052,
        returned_i32_08052
    );
}

#[inline(never)]
fn emit_08053() {
    let implicit_state_08053 = ImplicitState::Ready;
    let mixed_state_08053 = MixedState::Failed;
    let returned_i32_08053 = returned_i32(-8053);
    trace!(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-053: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08053,
        mixed_state_08053,
        returned_i32_08053
    );
}

#[inline(never)]
fn emit_08054() {
    let implicit_state_08054 = ImplicitState::Unknown;
    let mixed_state_08054 = MixedState::Unknown;
    let returned_i32_08054 = returned_i32(-8054);
    trace!(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-08 instance-054: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08054,
        mixed_state_08054,
        returned_i32_08054
    );
}

#[inline(never)]
fn emit_08055() {
    let implicit_state_08055 = ImplicitState::Starting;
    let mixed_state_08055 = MixedState::Warning;
    let returned_i32_08055 = returned_i32(-8055);
    trace!(
        ERROR,
        MIXED,
        "fixture-1000 scenario-08 instance-055: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08055,
        mixed_state_08055,
        returned_i32_08055
    );
}

#[inline(never)]
fn emit_08056() {
    let implicit_state_08056 = ImplicitState::Ready;
    let mixed_state_08056 = MixedState::Failed;
    let returned_i32_08056 = returned_i32(-8056);
    trace!(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-08 instance-056: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08056,
        mixed_state_08056,
        returned_i32_08056
    );
}

#[inline(never)]
fn emit_08057() {
    let implicit_state_08057 = ImplicitState::Unknown;
    let mixed_state_08057 = MixedState::Unknown;
    let returned_i32_08057 = returned_i32(-8057);
    trace!(
        INFO,
        MIXED,
        "fixture-1000 scenario-08 instance-057: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08057,
        mixed_state_08057,
        returned_i32_08057
    );
}

#[inline(never)]
fn emit_08058() {
    let implicit_state_08058 = ImplicitState::Starting;
    let mixed_state_08058 = MixedState::Warning;
    let returned_i32_08058 = returned_i32(-8058);
    trace!(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-08 instance-058: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08058,
        mixed_state_08058,
        returned_i32_08058
    );
}

#[inline(never)]
fn emit_08059() {
    let implicit_state_08059 = ImplicitState::Ready;
    let mixed_state_08059 = MixedState::Failed;
    let returned_i32_08059 = returned_i32(-8059);
    trace!(
        WARNING,
        MIXED,
        "fixture-1000 scenario-08 instance-059: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08059,
        mixed_state_08059,
        returned_i32_08059
    );
}

pub(crate) fn emit() {
    emit_08050();
    emit_08051();
    emit_08052();
    emit_08053();
    emit_08054();
    emit_08055();
    emit_08056();
    emit_08057();
    emit_08058();
    emit_08059();
}

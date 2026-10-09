use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08060() {
    let implicit_state_08060 = ImplicitState::Unknown;
    let mixed_state_08060 = MixedState::Unknown;
    let returned_i32_08060 = returned_i32(-8060);
    trace!(
        ERROR,
        STATUS,
        "fixture-1000 scenario-08 instance-060: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08060,
        mixed_state_08060,
        returned_i32_08060
    );
}

#[inline(never)]
fn emit_08061() {
    let implicit_state_08061 = ImplicitState::Starting;
    let mixed_state_08061 = MixedState::Warning;
    let returned_i32_08061 = returned_i32(-8061);
    trace!(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-08 instance-061: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08061,
        mixed_state_08061,
        returned_i32_08061
    );
}

#[inline(never)]
fn emit_08062() {
    let implicit_state_08062 = ImplicitState::Ready;
    let mixed_state_08062 = MixedState::Failed;
    let returned_i32_08062 = returned_i32(-8062);
    trace!(
        INFO,
        STATUS,
        "fixture-1000 scenario-08 instance-062: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08062,
        mixed_state_08062,
        returned_i32_08062
    );
}

#[inline(never)]
fn emit_08063() {
    let implicit_state_08063 = ImplicitState::Unknown;
    let mixed_state_08063 = MixedState::Unknown;
    let returned_i32_08063 = returned_i32(-8063);
    trace!(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-08 instance-063: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08063,
        mixed_state_08063,
        returned_i32_08063
    );
}

#[inline(never)]
fn emit_08064() {
    let implicit_state_08064 = ImplicitState::Starting;
    let mixed_state_08064 = MixedState::Warning;
    let returned_i32_08064 = returned_i32(-8064);
    trace!(
        WARNING,
        STATUS,
        "fixture-1000 scenario-08 instance-064: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08064,
        mixed_state_08064,
        returned_i32_08064
    );
}

#[inline(never)]
fn emit_08065() {
    let implicit_state_08065 = ImplicitState::Ready;
    let mixed_state_08065 = MixedState::Failed;
    let returned_i32_08065 = returned_i32(-8065);
    trace!(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-065: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08065,
        mixed_state_08065,
        returned_i32_08065
    );
}

#[inline(never)]
fn emit_08066() {
    let implicit_state_08066 = ImplicitState::Unknown;
    let mixed_state_08066 = MixedState::Unknown;
    let returned_i32_08066 = returned_i32(-8066);
    trace!(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-066: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08066,
        mixed_state_08066,
        returned_i32_08066
    );
}

#[inline(never)]
fn emit_08067() {
    let implicit_state_08067 = ImplicitState::Starting;
    let mixed_state_08067 = MixedState::Warning;
    let returned_i32_08067 = returned_i32(-8067);
    trace!(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-067: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08067,
        mixed_state_08067,
        returned_i32_08067
    );
}

#[inline(never)]
fn emit_08068() {
    let implicit_state_08068 = ImplicitState::Ready;
    let mixed_state_08068 = MixedState::Failed;
    let returned_i32_08068 = returned_i32(-8068);
    trace!(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-068: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08068,
        mixed_state_08068,
        returned_i32_08068
    );
}

#[inline(never)]
fn emit_08069() {
    let implicit_state_08069 = ImplicitState::Unknown;
    let mixed_state_08069 = MixedState::Unknown;
    let returned_i32_08069 = returned_i32(-8069);
    trace!(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-08 instance-069: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08069,
        mixed_state_08069,
        returned_i32_08069
    );
}

pub(crate) fn emit() {
    emit_08060();
    emit_08061();
    emit_08062();
    emit_08063();
    emit_08064();
    emit_08065();
    emit_08066();
    emit_08067();
    emit_08068();
    emit_08069();
}

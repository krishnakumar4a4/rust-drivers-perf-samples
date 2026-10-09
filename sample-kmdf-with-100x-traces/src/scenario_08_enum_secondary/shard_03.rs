use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08030() {
    let implicit_state_08030 = ImplicitState::Unknown;
    let mixed_state_08030 = MixedState::Unknown;
    let returned_i32_08030 = returned_i32(-8030);
    trace!(
        ERROR,
        MIXED,
        "fixture-1000 scenario-08 instance-030: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08030,
        mixed_state_08030,
        returned_i32_08030
    );
}

#[inline(never)]
fn emit_08031() {
    let implicit_state_08031 = ImplicitState::Starting;
    let mixed_state_08031 = MixedState::Warning;
    let returned_i32_08031 = returned_i32(-8031);
    trace!(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-08 instance-031: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08031,
        mixed_state_08031,
        returned_i32_08031
    );
}

#[inline(never)]
fn emit_08032() {
    let implicit_state_08032 = ImplicitState::Ready;
    let mixed_state_08032 = MixedState::Failed;
    let returned_i32_08032 = returned_i32(-8032);
    trace!(
        INFO,
        MIXED,
        "fixture-1000 scenario-08 instance-032: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08032,
        mixed_state_08032,
        returned_i32_08032
    );
}

#[inline(never)]
fn emit_08033() {
    let implicit_state_08033 = ImplicitState::Unknown;
    let mixed_state_08033 = MixedState::Unknown;
    let returned_i32_08033 = returned_i32(-8033);
    trace!(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-08 instance-033: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08033,
        mixed_state_08033,
        returned_i32_08033
    );
}

#[inline(never)]
fn emit_08034() {
    let implicit_state_08034 = ImplicitState::Starting;
    let mixed_state_08034 = MixedState::Warning;
    let returned_i32_08034 = returned_i32(-8034);
    trace!(
        WARNING,
        MIXED,
        "fixture-1000 scenario-08 instance-034: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08034,
        mixed_state_08034,
        returned_i32_08034
    );
}

#[inline(never)]
fn emit_08035() {
    let implicit_state_08035 = ImplicitState::Ready;
    let mixed_state_08035 = MixedState::Failed;
    let returned_i32_08035 = returned_i32(-8035);
    trace!(
        ERROR,
        STATUS,
        "fixture-1000 scenario-08 instance-035: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08035,
        mixed_state_08035,
        returned_i32_08035
    );
}

#[inline(never)]
fn emit_08036() {
    let implicit_state_08036 = ImplicitState::Unknown;
    let mixed_state_08036 = MixedState::Unknown;
    let returned_i32_08036 = returned_i32(-8036);
    trace!(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-08 instance-036: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08036,
        mixed_state_08036,
        returned_i32_08036
    );
}

#[inline(never)]
fn emit_08037() {
    let implicit_state_08037 = ImplicitState::Starting;
    let mixed_state_08037 = MixedState::Warning;
    let returned_i32_08037 = returned_i32(-8037);
    trace!(
        INFO,
        STATUS,
        "fixture-1000 scenario-08 instance-037: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08037,
        mixed_state_08037,
        returned_i32_08037
    );
}

#[inline(never)]
fn emit_08038() {
    let implicit_state_08038 = ImplicitState::Ready;
    let mixed_state_08038 = MixedState::Failed;
    let returned_i32_08038 = returned_i32(-8038);
    trace!(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-08 instance-038: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08038,
        mixed_state_08038,
        returned_i32_08038
    );
}

#[inline(never)]
fn emit_08039() {
    let implicit_state_08039 = ImplicitState::Unknown;
    let mixed_state_08039 = MixedState::Unknown;
    let returned_i32_08039 = returned_i32(-8039);
    trace!(
        WARNING,
        STATUS,
        "fixture-1000 scenario-08 instance-039: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08039,
        mixed_state_08039,
        returned_i32_08039
    );
}

pub(crate) fn emit() {
    emit_08030();
    emit_08031();
    emit_08032();
    emit_08033();
    emit_08034();
    emit_08035();
    emit_08036();
    emit_08037();
    emit_08038();
    emit_08039();
}

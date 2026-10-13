use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08000() {
    let implicit_state_08000 = ImplicitState::Unknown;
    let mixed_state_08000 = MixedState::Unknown;
    let returned_i32_08000 = returned_i32(-8000);
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-10 scenario-08 instance-000: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08000,
        mixed_state_08000,
        returned_i32_08000,
    );
}

pub(crate) fn emit() {
    emit_08000();
}

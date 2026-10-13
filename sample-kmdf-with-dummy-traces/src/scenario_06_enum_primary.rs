use crate::dummy_trace::*;

use crate::{ByteState, LongState, ShortState};

#[inline(never)]
fn emit_06000() {
    let byte_state_06000 = ByteState::Unknown;
    let short_state_06000 = ShortState::Unknown;
    let long_state_06000 = LongState::Offline;
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-10 scenario-06 instance-000: byte={}, short={}, long={}",
        byte_state_06000,
        short_state_06000,
        long_state_06000,
    );
}

pub(crate) fn emit() {
    emit_06000();
}

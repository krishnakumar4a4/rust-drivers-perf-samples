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
        "fixture-1000 scenario-06 instance-000: byte={}, short={}, long={}",
        byte_state_06000,
        short_state_06000,
        long_state_06000,
    );
}

#[inline(never)]
fn emit_06001() {
    let byte_state_06001 = ByteState::Starting;
    let short_state_06001 = ShortState::Warning;
    let long_state_06001 = LongState::Online;
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-06 instance-001: byte={}, short={}, long={}",
        byte_state_06001,
        short_state_06001,
        long_state_06001,
    );
}

#[inline(never)]
fn emit_06002() {
    let byte_state_06002 = ByteState::Ready;
    let short_state_06002 = ShortState::Failed;
    let long_state_06002 = LongState::Recovering;
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-06 instance-002: byte={}, short={}, long={}",
        byte_state_06002,
        short_state_06002,
        long_state_06002,
    );
}

#[inline(never)]
fn emit_06003() {
    let byte_state_06003 = ByteState::Unknown;
    let short_state_06003 = ShortState::Unknown;
    let long_state_06003 = LongState::Offline;
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-06 instance-003: byte={}, short={}, long={}",
        byte_state_06003,
        short_state_06003,
        long_state_06003,
    );
}

#[inline(never)]
fn emit_06004() {
    let byte_state_06004 = ByteState::Starting;
    let short_state_06004 = ShortState::Warning;
    let long_state_06004 = LongState::Online;
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-06 instance-004: byte={}, short={}, long={}",
        byte_state_06004,
        short_state_06004,
        long_state_06004,
    );
}

#[inline(never)]
fn emit_06005() {
    let byte_state_06005 = ByteState::Ready;
    let short_state_06005 = ShortState::Failed;
    let long_state_06005 = LongState::Recovering;
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-06 instance-005: byte={}, short={}, long={}",
        byte_state_06005,
        short_state_06005,
        long_state_06005,
    );
}

#[inline(never)]
fn emit_06006() {
    let byte_state_06006 = ByteState::Unknown;
    let short_state_06006 = ShortState::Unknown;
    let long_state_06006 = LongState::Offline;
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-06 instance-006: byte={}, short={}, long={}",
        byte_state_06006,
        short_state_06006,
        long_state_06006,
    );
}

#[inline(never)]
fn emit_06007() {
    let byte_state_06007 = ByteState::Starting;
    let short_state_06007 = ShortState::Warning;
    let long_state_06007 = LongState::Online;
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-06 instance-007: byte={}, short={}, long={}",
        byte_state_06007,
        short_state_06007,
        long_state_06007,
    );
}

#[inline(never)]
fn emit_06008() {
    let byte_state_06008 = ByteState::Ready;
    let short_state_06008 = ShortState::Failed;
    let long_state_06008 = LongState::Recovering;
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-06 instance-008: byte={}, short={}, long={}",
        byte_state_06008,
        short_state_06008,
        long_state_06008,
    );
}

#[inline(never)]
fn emit_06009() {
    let byte_state_06009 = ByteState::Unknown;
    let short_state_06009 = ShortState::Unknown;
    let long_state_06009 = LongState::Offline;
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-06 instance-009: byte={}, short={}, long={}",
        byte_state_06009,
        short_state_06009,
        long_state_06009,
    );
}

pub(crate) fn emit() {
    emit_06000();
    emit_06001();
    emit_06002();
    emit_06003();
    emit_06004();
    emit_06005();
    emit_06006();
    emit_06007();
    emit_06008();
    emit_06009();
}

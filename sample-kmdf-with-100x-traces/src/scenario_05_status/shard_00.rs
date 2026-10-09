use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05000() {
    let irql_05000 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05000 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05000 = returned_usize(5000);
    trace!(
        INFO,
        STATUS,
        "fixture-1000 scenario-05 instance-000: irql={}, status={}, usize={}",
        irql_05000,
        status_05000,
        usize_05000
    );
}

#[inline(never)]
fn emit_05001() {
    let irql_05001 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05001 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05001 = returned_usize(5001);
    trace!(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-05 instance-001: irql={}, status={}, usize={}",
        irql_05001,
        status_05001,
        usize_05001
    );
}

#[inline(never)]
fn emit_05002() {
    let irql_05002 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05002 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05002 = returned_usize(5002);
    trace!(
        WARNING,
        STATUS,
        "fixture-1000 scenario-05 instance-002: irql={}, status={}, usize={}",
        irql_05002,
        status_05002,
        usize_05002
    );
}

#[inline(never)]
fn emit_05003() {
    let irql_05003 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05003 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05003 = returned_usize(5003);
    trace!(
        ERROR,
        STATUS,
        "fixture-1000 scenario-05 instance-003: irql={}, status={}, usize={}",
        irql_05003,
        status_05003,
        usize_05003
    );
}

#[inline(never)]
fn emit_05004() {
    let irql_05004 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05004 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05004 = returned_usize(5004);
    trace!(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-05 instance-004: irql={}, status={}, usize={}",
        irql_05004,
        status_05004,
        usize_05004
    );
}

#[inline(never)]
fn emit_05005() {
    let irql_05005 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05005 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05005 = returned_usize(5005);
    trace!(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-005: irql={}, status={}, usize={}",
        irql_05005,
        status_05005,
        usize_05005
    );
}

#[inline(never)]
fn emit_05006() {
    let irql_05006 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05006 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05006 = returned_usize(5006);
    trace!(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-006: irql={}, status={}, usize={}",
        irql_05006,
        status_05006,
        usize_05006
    );
}

#[inline(never)]
fn emit_05007() {
    let irql_05007 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05007 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05007 = returned_usize(5007);
    trace!(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-007: irql={}, status={}, usize={}",
        irql_05007,
        status_05007,
        usize_05007
    );
}

#[inline(never)]
fn emit_05008() {
    let irql_05008 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05008 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05008 = returned_usize(5008);
    trace!(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-008: irql={}, status={}, usize={}",
        irql_05008,
        status_05008,
        usize_05008
    );
}

#[inline(never)]
fn emit_05009() {
    let irql_05009 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05009 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05009 = returned_usize(5009);
    trace!(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-009: irql={}, status={}, usize={}",
        irql_05009,
        status_05009,
        usize_05009
    );
}

pub(crate) fn emit() {
    emit_05000();
    emit_05001();
    emit_05002();
    emit_05003();
    emit_05004();
    emit_05005();
    emit_05006();
    emit_05007();
    emit_05008();
    emit_05009();
}

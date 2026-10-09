use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05070() {
    let irql_05070 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05070 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05070 = returned_usize(5070);
    trace!(
        INFO,
        MIXED,
        "fixture-1000 scenario-05 instance-070: irql={}, status={}, usize={}",
        irql_05070,
        status_05070,
        usize_05070
    );
}

#[inline(never)]
fn emit_05071() {
    let irql_05071 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05071 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05071 = returned_usize(5071);
    trace!(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-05 instance-071: irql={}, status={}, usize={}",
        irql_05071,
        status_05071,
        usize_05071
    );
}

#[inline(never)]
fn emit_05072() {
    let irql_05072 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05072 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05072 = returned_usize(5072);
    trace!(
        WARNING,
        MIXED,
        "fixture-1000 scenario-05 instance-072: irql={}, status={}, usize={}",
        irql_05072,
        status_05072,
        usize_05072
    );
}

#[inline(never)]
fn emit_05073() {
    let irql_05073 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05073 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05073 = returned_usize(5073);
    trace!(
        ERROR,
        MIXED,
        "fixture-1000 scenario-05 instance-073: irql={}, status={}, usize={}",
        irql_05073,
        status_05073,
        usize_05073
    );
}

#[inline(never)]
fn emit_05074() {
    let irql_05074 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05074 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05074 = returned_usize(5074);
    trace!(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-05 instance-074: irql={}, status={}, usize={}",
        irql_05074,
        status_05074,
        usize_05074
    );
}

#[inline(never)]
fn emit_05075() {
    let irql_05075 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05075 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05075 = returned_usize(5075);
    trace!(
        INFO,
        STATUS,
        "fixture-1000 scenario-05 instance-075: irql={}, status={}, usize={}",
        irql_05075,
        status_05075,
        usize_05075
    );
}

#[inline(never)]
fn emit_05076() {
    let irql_05076 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05076 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05076 = returned_usize(5076);
    trace!(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-05 instance-076: irql={}, status={}, usize={}",
        irql_05076,
        status_05076,
        usize_05076
    );
}

#[inline(never)]
fn emit_05077() {
    let irql_05077 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05077 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05077 = returned_usize(5077);
    trace!(
        WARNING,
        STATUS,
        "fixture-1000 scenario-05 instance-077: irql={}, status={}, usize={}",
        irql_05077,
        status_05077,
        usize_05077
    );
}

#[inline(never)]
fn emit_05078() {
    let irql_05078 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05078 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05078 = returned_usize(5078);
    trace!(
        ERROR,
        STATUS,
        "fixture-1000 scenario-05 instance-078: irql={}, status={}, usize={}",
        irql_05078,
        status_05078,
        usize_05078
    );
}

#[inline(never)]
fn emit_05079() {
    let irql_05079 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05079 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05079 = returned_usize(5079);
    trace!(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-05 instance-079: irql={}, status={}, usize={}",
        irql_05079,
        status_05079,
        usize_05079
    );
}

pub(crate) fn emit() {
    emit_05070();
    emit_05071();
    emit_05072();
    emit_05073();
    emit_05074();
    emit_05075();
    emit_05076();
    emit_05077();
    emit_05078();
    emit_05079();
}

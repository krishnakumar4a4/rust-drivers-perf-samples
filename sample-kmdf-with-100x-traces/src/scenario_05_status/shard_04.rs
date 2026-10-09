use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05040() {
    let irql_05040 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05040 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05040 = returned_usize(5040);
    trace!(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-040: irql={}, status={}, usize={}",
        irql_05040,
        status_05040,
        usize_05040
    );
}

#[inline(never)]
fn emit_05041() {
    let irql_05041 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05041 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05041 = returned_usize(5041);
    trace!(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-041: irql={}, status={}, usize={}",
        irql_05041,
        status_05041,
        usize_05041
    );
}

#[inline(never)]
fn emit_05042() {
    let irql_05042 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05042 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05042 = returned_usize(5042);
    trace!(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-042: irql={}, status={}, usize={}",
        irql_05042,
        status_05042,
        usize_05042
    );
}

#[inline(never)]
fn emit_05043() {
    let irql_05043 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05043 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05043 = returned_usize(5043);
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-043: irql={}, status={}, usize={}",
        irql_05043,
        status_05043,
        usize_05043
    );
}

#[inline(never)]
fn emit_05044() {
    let irql_05044 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05044 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05044 = returned_usize(5044);
    trace!(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-044: irql={}, status={}, usize={}",
        irql_05044,
        status_05044,
        usize_05044
    );
}

#[inline(never)]
fn emit_05045() {
    let irql_05045 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05045 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05045 = returned_usize(5045);
    trace!(
        INFO,
        MIXED,
        "fixture-1000 scenario-05 instance-045: irql={}, status={}, usize={}",
        irql_05045,
        status_05045,
        usize_05045
    );
}

#[inline(never)]
fn emit_05046() {
    let irql_05046 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05046 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05046 = returned_usize(5046);
    trace!(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-05 instance-046: irql={}, status={}, usize={}",
        irql_05046,
        status_05046,
        usize_05046
    );
}

#[inline(never)]
fn emit_05047() {
    let irql_05047 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05047 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05047 = returned_usize(5047);
    trace!(
        WARNING,
        MIXED,
        "fixture-1000 scenario-05 instance-047: irql={}, status={}, usize={}",
        irql_05047,
        status_05047,
        usize_05047
    );
}

#[inline(never)]
fn emit_05048() {
    let irql_05048 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05048 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05048 = returned_usize(5048);
    trace!(
        ERROR,
        MIXED,
        "fixture-1000 scenario-05 instance-048: irql={}, status={}, usize={}",
        irql_05048,
        status_05048,
        usize_05048
    );
}

#[inline(never)]
fn emit_05049() {
    let irql_05049 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05049 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05049 = returned_usize(5049);
    trace!(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-05 instance-049: irql={}, status={}, usize={}",
        irql_05049,
        status_05049,
        usize_05049
    );
}

pub(crate) fn emit() {
    emit_05040();
    emit_05041();
    emit_05042();
    emit_05043();
    emit_05044();
    emit_05045();
    emit_05046();
    emit_05047();
    emit_05048();
    emit_05049();
}

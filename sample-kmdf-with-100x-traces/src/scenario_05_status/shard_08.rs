use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05080() {
    let irql_05080 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05080 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05080 = returned_usize(5080);
    trace!(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-080: irql={}, status={}, usize={}",
        irql_05080,
        status_05080,
        usize_05080
    );
}

#[inline(never)]
fn emit_05081() {
    let irql_05081 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05081 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05081 = returned_usize(5081);
    trace!(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-081: irql={}, status={}, usize={}",
        irql_05081,
        status_05081,
        usize_05081
    );
}

#[inline(never)]
fn emit_05082() {
    let irql_05082 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05082 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05082 = returned_usize(5082);
    trace!(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-082: irql={}, status={}, usize={}",
        irql_05082,
        status_05082,
        usize_05082
    );
}

#[inline(never)]
fn emit_05083() {
    let irql_05083 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05083 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05083 = returned_usize(5083);
    trace!(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-083: irql={}, status={}, usize={}",
        irql_05083,
        status_05083,
        usize_05083
    );
}

#[inline(never)]
fn emit_05084() {
    let irql_05084 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05084 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05084 = returned_usize(5084);
    trace!(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-084: irql={}, status={}, usize={}",
        irql_05084,
        status_05084,
        usize_05084
    );
}

#[inline(never)]
fn emit_05085() {
    let irql_05085 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05085 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05085 = returned_usize(5085);
    trace!(
        INFO,
        GENERIC,
        "fixture-1000 scenario-05 instance-085: irql={}, status={}, usize={}",
        irql_05085,
        status_05085,
        usize_05085
    );
}

#[inline(never)]
fn emit_05086() {
    let irql_05086 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05086 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05086 = returned_usize(5086);
    trace!(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-05 instance-086: irql={}, status={}, usize={}",
        irql_05086,
        status_05086,
        usize_05086
    );
}

#[inline(never)]
fn emit_05087() {
    let irql_05087 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05087 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05087 = returned_usize(5087);
    trace!(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-05 instance-087: irql={}, status={}, usize={}",
        irql_05087,
        status_05087,
        usize_05087
    );
}

#[inline(never)]
fn emit_05088() {
    let irql_05088 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05088 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05088 = returned_usize(5088);
    trace!(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-05 instance-088: irql={}, status={}, usize={}",
        irql_05088,
        status_05088,
        usize_05088
    );
}

#[inline(never)]
fn emit_05089() {
    let irql_05089 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05089 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05089 = returned_usize(5089);
    trace!(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-05 instance-089: irql={}, status={}, usize={}",
        irql_05089,
        status_05089,
        usize_05089
    );
}

pub(crate) fn emit() {
    emit_05080();
    emit_05081();
    emit_05082();
    emit_05083();
    emit_05084();
    emit_05085();
    emit_05086();
    emit_05087();
    emit_05088();
    emit_05089();
}

use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05060() {
    let irql_05060 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05060 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05060 = returned_usize(5060);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-05 instance-060: irql={}, status={}, usize={}",
        irql_05060,
        status_05060,
        usize_05060,
    );
}

#[inline(never)]
fn emit_05061() {
    let irql_05061 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05061 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05061 = returned_usize(5061);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-05 instance-061: irql={}, status={}, usize={}",
        irql_05061,
        status_05061,
        usize_05061,
    );
}

#[inline(never)]
fn emit_05062() {
    let irql_05062 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05062 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05062 = returned_usize(5062);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-05 instance-062: irql={}, status={}, usize={}",
        irql_05062,
        status_05062,
        usize_05062,
    );
}

#[inline(never)]
fn emit_05063() {
    let irql_05063 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05063 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05063 = returned_usize(5063);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-05 instance-063: irql={}, status={}, usize={}",
        irql_05063,
        status_05063,
        usize_05063,
    );
}

#[inline(never)]
fn emit_05064() {
    let irql_05064 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05064 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05064 = returned_usize(5064);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-05 instance-064: irql={}, status={}, usize={}",
        irql_05064,
        status_05064,
        usize_05064,
    );
}

#[inline(never)]
fn emit_05065() {
    let irql_05065 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05065 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05065 = returned_usize(5065);
    dummy_trace(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-065: irql={}, status={}, usize={}",
        irql_05065,
        status_05065,
        usize_05065,
    );
}

#[inline(never)]
fn emit_05066() {
    let irql_05066 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05066 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05066 = returned_usize(5066);
    dummy_trace(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-066: irql={}, status={}, usize={}",
        irql_05066,
        status_05066,
        usize_05066,
    );
}

#[inline(never)]
fn emit_05067() {
    let irql_05067 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05067 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05067 = returned_usize(5067);
    dummy_trace(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-067: irql={}, status={}, usize={}",
        irql_05067,
        status_05067,
        usize_05067,
    );
}

#[inline(never)]
fn emit_05068() {
    let irql_05068 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05068 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05068 = returned_usize(5068);
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-068: irql={}, status={}, usize={}",
        irql_05068,
        status_05068,
        usize_05068,
    );
}

#[inline(never)]
fn emit_05069() {
    let irql_05069 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05069 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05069 = returned_usize(5069);
    dummy_trace(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-069: irql={}, status={}, usize={}",
        irql_05069,
        status_05069,
        usize_05069,
    );
}

pub(crate) fn emit() {
    emit_05060();
    emit_05061();
    emit_05062();
    emit_05063();
    emit_05064();
    emit_05065();
    emit_05066();
    emit_05067();
    emit_05068();
    emit_05069();
}

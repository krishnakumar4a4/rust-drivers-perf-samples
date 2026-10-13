use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05010() {
    let irql_05010 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05010 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05010 = returned_usize(5010);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-05 instance-010: irql={}, status={}, usize={}",
        irql_05010,
        status_05010,
        usize_05010,
    );
}

#[inline(never)]
fn emit_05011() {
    let irql_05011 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05011 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05011 = returned_usize(5011);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-05 instance-011: irql={}, status={}, usize={}",
        irql_05011,
        status_05011,
        usize_05011,
    );
}

#[inline(never)]
fn emit_05012() {
    let irql_05012 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05012 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05012 = returned_usize(5012);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-05 instance-012: irql={}, status={}, usize={}",
        irql_05012,
        status_05012,
        usize_05012,
    );
}

#[inline(never)]
fn emit_05013() {
    let irql_05013 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05013 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05013 = returned_usize(5013);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-05 instance-013: irql={}, status={}, usize={}",
        irql_05013,
        status_05013,
        usize_05013,
    );
}

#[inline(never)]
fn emit_05014() {
    let irql_05014 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05014 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05014 = returned_usize(5014);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-05 instance-014: irql={}, status={}, usize={}",
        irql_05014,
        status_05014,
        usize_05014,
    );
}

#[inline(never)]
fn emit_05015() {
    let irql_05015 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05015 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05015 = returned_usize(5015);
    dummy_trace(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-015: irql={}, status={}, usize={}",
        irql_05015,
        status_05015,
        usize_05015,
    );
}

#[inline(never)]
fn emit_05016() {
    let irql_05016 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05016 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05016 = returned_usize(5016);
    dummy_trace(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-016: irql={}, status={}, usize={}",
        irql_05016,
        status_05016,
        usize_05016,
    );
}

#[inline(never)]
fn emit_05017() {
    let irql_05017 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05017 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05017 = returned_usize(5017);
    dummy_trace(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-017: irql={}, status={}, usize={}",
        irql_05017,
        status_05017,
        usize_05017,
    );
}

#[inline(never)]
fn emit_05018() {
    let irql_05018 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05018 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05018 = returned_usize(5018);
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-018: irql={}, status={}, usize={}",
        irql_05018,
        status_05018,
        usize_05018,
    );
}

#[inline(never)]
fn emit_05019() {
    let irql_05019 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05019 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05019 = returned_usize(5019);
    dummy_trace(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-019: irql={}, status={}, usize={}",
        irql_05019,
        status_05019,
        usize_05019,
    );
}

pub(crate) fn emit() {
    emit_05010();
    emit_05011();
    emit_05012();
    emit_05013();
    emit_05014();
    emit_05015();
    emit_05016();
    emit_05017();
    emit_05018();
    emit_05019();
}

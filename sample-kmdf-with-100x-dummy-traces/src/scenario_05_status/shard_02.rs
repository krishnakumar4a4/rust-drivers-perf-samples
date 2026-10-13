use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05020() {
    let irql_05020 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05020 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05020 = returned_usize(5020);
    dummy_trace(
        INFO,
        MIXED,
        "fixture-1000 scenario-05 instance-020: irql={}, status={}, usize={}",
        irql_05020,
        status_05020,
        usize_05020,
    );
}

#[inline(never)]
fn emit_05021() {
    let irql_05021 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05021 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05021 = returned_usize(5021);
    dummy_trace(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-05 instance-021: irql={}, status={}, usize={}",
        irql_05021,
        status_05021,
        usize_05021,
    );
}

#[inline(never)]
fn emit_05022() {
    let irql_05022 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05022 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05022 = returned_usize(5022);
    dummy_trace(
        WARNING,
        MIXED,
        "fixture-1000 scenario-05 instance-022: irql={}, status={}, usize={}",
        irql_05022,
        status_05022,
        usize_05022,
    );
}

#[inline(never)]
fn emit_05023() {
    let irql_05023 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05023 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05023 = returned_usize(5023);
    dummy_trace(
        ERROR,
        MIXED,
        "fixture-1000 scenario-05 instance-023: irql={}, status={}, usize={}",
        irql_05023,
        status_05023,
        usize_05023,
    );
}

#[inline(never)]
fn emit_05024() {
    let irql_05024 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05024 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05024 = returned_usize(5024);
    dummy_trace(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-05 instance-024: irql={}, status={}, usize={}",
        irql_05024,
        status_05024,
        usize_05024,
    );
}

#[inline(never)]
fn emit_05025() {
    let irql_05025 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05025 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05025 = returned_usize(5025);
    dummy_trace(
        INFO,
        STATUS,
        "fixture-1000 scenario-05 instance-025: irql={}, status={}, usize={}",
        irql_05025,
        status_05025,
        usize_05025,
    );
}

#[inline(never)]
fn emit_05026() {
    let irql_05026 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05026 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05026 = returned_usize(5026);
    dummy_trace(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-05 instance-026: irql={}, status={}, usize={}",
        irql_05026,
        status_05026,
        usize_05026,
    );
}

#[inline(never)]
fn emit_05027() {
    let irql_05027 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05027 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05027 = returned_usize(5027);
    dummy_trace(
        WARNING,
        STATUS,
        "fixture-1000 scenario-05 instance-027: irql={}, status={}, usize={}",
        irql_05027,
        status_05027,
        usize_05027,
    );
}

#[inline(never)]
fn emit_05028() {
    let irql_05028 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05028 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05028 = returned_usize(5028);
    dummy_trace(
        ERROR,
        STATUS,
        "fixture-1000 scenario-05 instance-028: irql={}, status={}, usize={}",
        irql_05028,
        status_05028,
        usize_05028,
    );
}

#[inline(never)]
fn emit_05029() {
    let irql_05029 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05029 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05029 = returned_usize(5029);
    dummy_trace(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-05 instance-029: irql={}, status={}, usize={}",
        irql_05029,
        status_05029,
        usize_05029,
    );
}

pub(crate) fn emit() {
    emit_05020();
    emit_05021();
    emit_05022();
    emit_05023();
    emit_05024();
    emit_05025();
    emit_05026();
    emit_05027();
    emit_05028();
    emit_05029();
}

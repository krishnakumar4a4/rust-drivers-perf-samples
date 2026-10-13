use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05090() {
    let irql_05090 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05090 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05090 = returned_usize(5090);
    dummy_trace(
        INFO,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-090: irql={}, status={}, usize={}",
        irql_05090,
        status_05090,
        usize_05090,
    );
}

#[inline(never)]
fn emit_05091() {
    let irql_05091 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05091 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05091 = returned_usize(5091);
    dummy_trace(
        VERBOSE,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-091: irql={}, status={}, usize={}",
        irql_05091,
        status_05091,
        usize_05091,
    );
}

#[inline(never)]
fn emit_05092() {
    let irql_05092 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05092 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05092 = returned_usize(5092);
    dummy_trace(
        WARNING,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-092: irql={}, status={}, usize={}",
        irql_05092,
        status_05092,
        usize_05092,
    );
}

#[inline(never)]
fn emit_05093() {
    let irql_05093 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05093 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05093 = returned_usize(5093);
    dummy_trace(
        ERROR,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-093: irql={}, status={}, usize={}",
        irql_05093,
        status_05093,
        usize_05093,
    );
}

#[inline(never)]
fn emit_05094() {
    let irql_05094 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05094 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05094 = returned_usize(5094);
    dummy_trace(
        CRITICAL,
        ENUM_SECONDARY,
        "fixture-1000 scenario-05 instance-094: irql={}, status={}, usize={}",
        irql_05094,
        status_05094,
        usize_05094,
    );
}

#[inline(never)]
fn emit_05095() {
    let irql_05095 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05095 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05095 = returned_usize(5095);
    dummy_trace(
        INFO,
        MIXED,
        "fixture-1000 scenario-05 instance-095: irql={}, status={}, usize={}",
        irql_05095,
        status_05095,
        usize_05095,
    );
}

#[inline(never)]
fn emit_05096() {
    let irql_05096 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05096 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05096 = returned_usize(5096);
    dummy_trace(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-05 instance-096: irql={}, status={}, usize={}",
        irql_05096,
        status_05096,
        usize_05096,
    );
}

#[inline(never)]
fn emit_05097() {
    let irql_05097 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05097 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05097 = returned_usize(5097);
    dummy_trace(
        WARNING,
        MIXED,
        "fixture-1000 scenario-05 instance-097: irql={}, status={}, usize={}",
        irql_05097,
        status_05097,
        usize_05097,
    );
}

#[inline(never)]
fn emit_05098() {
    let irql_05098 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05098 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05098 = returned_usize(5098);
    dummy_trace(
        ERROR,
        MIXED,
        "fixture-1000 scenario-05 instance-098: irql={}, status={}, usize={}",
        irql_05098,
        status_05098,
        usize_05098,
    );
}

#[inline(never)]
fn emit_05099() {
    let irql_05099 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05099 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05099 = returned_usize(5099);
    dummy_trace(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-05 instance-099: irql={}, status={}, usize={}",
        irql_05099,
        status_05099,
        usize_05099,
    );
}

pub(crate) fn emit() {
    emit_05090();
    emit_05091();
    emit_05092();
    emit_05093();
    emit_05094();
    emit_05095();
    emit_05096();
    emit_05097();
    emit_05098();
    emit_05099();
}

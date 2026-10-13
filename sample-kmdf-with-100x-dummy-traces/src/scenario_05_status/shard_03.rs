use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05030() {
    let irql_05030 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05030 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05030 = returned_usize(5030);
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-030: irql={}, status={}, usize={}",
        irql_05030,
        status_05030,
        usize_05030,
    );
}

#[inline(never)]
fn emit_05031() {
    let irql_05031 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05031 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05031 = returned_usize(5031);
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-031: irql={}, status={}, usize={}",
        irql_05031,
        status_05031,
        usize_05031,
    );
}

#[inline(never)]
fn emit_05032() {
    let irql_05032 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05032 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05032 = returned_usize(5032);
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-032: irql={}, status={}, usize={}",
        irql_05032,
        status_05032,
        usize_05032,
    );
}

#[inline(never)]
fn emit_05033() {
    let irql_05033 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05033 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05033 = returned_usize(5033);
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-033: irql={}, status={}, usize={}",
        irql_05033,
        status_05033,
        usize_05033,
    );
}

#[inline(never)]
fn emit_05034() {
    let irql_05034 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05034 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05034 = returned_usize(5034);
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-034: irql={}, status={}, usize={}",
        irql_05034,
        status_05034,
        usize_05034,
    );
}

#[inline(never)]
fn emit_05035() {
    let irql_05035 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05035 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05035 = returned_usize(5035);
    dummy_trace(
        INFO,
        GENERIC,
        "fixture-1000 scenario-05 instance-035: irql={}, status={}, usize={}",
        irql_05035,
        status_05035,
        usize_05035,
    );
}

#[inline(never)]
fn emit_05036() {
    let irql_05036 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05036 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05036 = returned_usize(5036);
    dummy_trace(
        VERBOSE,
        GENERIC,
        "fixture-1000 scenario-05 instance-036: irql={}, status={}, usize={}",
        irql_05036,
        status_05036,
        usize_05036,
    );
}

#[inline(never)]
fn emit_05037() {
    let irql_05037 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05037 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05037 = returned_usize(5037);
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-1000 scenario-05 instance-037: irql={}, status={}, usize={}",
        irql_05037,
        status_05037,
        usize_05037,
    );
}

#[inline(never)]
fn emit_05038() {
    let irql_05038 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05038 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05038 = returned_usize(5038);
    dummy_trace(
        ERROR,
        GENERIC,
        "fixture-1000 scenario-05 instance-038: irql={}, status={}, usize={}",
        irql_05038,
        status_05038,
        usize_05038,
    );
}

#[inline(never)]
fn emit_05039() {
    let irql_05039 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05039 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05039 = returned_usize(5039);
    dummy_trace(
        CRITICAL,
        GENERIC,
        "fixture-1000 scenario-05 instance-039: irql={}, status={}, usize={}",
        irql_05039,
        status_05039,
        usize_05039,
    );
}

pub(crate) fn emit() {
    emit_05030();
    emit_05031();
    emit_05032();
    emit_05033();
    emit_05034();
    emit_05035();
    emit_05036();
    emit_05037();
    emit_05038();
    emit_05039();
}

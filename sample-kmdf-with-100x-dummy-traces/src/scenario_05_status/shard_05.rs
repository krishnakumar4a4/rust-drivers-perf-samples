use crate::dummy_trace::*;

use wdf::{status_codes, IRQL, NtStatus};

fn returned_usize(value: usize) -> usize {
    value
}

#[inline(never)]
fn emit_05050() {
    let irql_05050 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05050 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05050 = returned_usize(5050);
    dummy_trace(
        INFO,
        STATUS,
        "fixture-1000 scenario-05 instance-050: irql={}, status={}, usize={}",
        irql_05050,
        status_05050,
        usize_05050,
    );
}

#[inline(never)]
fn emit_05051() {
    let irql_05051 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05051 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05051 = returned_usize(5051);
    dummy_trace(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-05 instance-051: irql={}, status={}, usize={}",
        irql_05051,
        status_05051,
        usize_05051,
    );
}

#[inline(never)]
fn emit_05052() {
    let irql_05052 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05052 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05052 = returned_usize(5052);
    dummy_trace(
        WARNING,
        STATUS,
        "fixture-1000 scenario-05 instance-052: irql={}, status={}, usize={}",
        irql_05052,
        status_05052,
        usize_05052,
    );
}

#[inline(never)]
fn emit_05053() {
    let irql_05053 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05053 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05053 = returned_usize(5053);
    dummy_trace(
        ERROR,
        STATUS,
        "fixture-1000 scenario-05 instance-053: irql={}, status={}, usize={}",
        irql_05053,
        status_05053,
        usize_05053,
    );
}

#[inline(never)]
fn emit_05054() {
    let irql_05054 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05054 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05054 = returned_usize(5054);
    dummy_trace(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-05 instance-054: irql={}, status={}, usize={}",
        irql_05054,
        status_05054,
        usize_05054,
    );
}

#[inline(never)]
fn emit_05055() {
    let irql_05055 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05055 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05055 = returned_usize(5055);
    dummy_trace(
        INFO,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-055: irql={}, status={}, usize={}",
        irql_05055,
        status_05055,
        usize_05055,
    );
}

#[inline(never)]
fn emit_05056() {
    let irql_05056 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05056 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05056 = returned_usize(5056);
    dummy_trace(
        VERBOSE,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-056: irql={}, status={}, usize={}",
        irql_05056,
        status_05056,
        usize_05056,
    );
}

#[inline(never)]
fn emit_05057() {
    let irql_05057 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05057 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05057 = returned_usize(5057);
    dummy_trace(
        WARNING,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-057: irql={}, status={}, usize={}",
        irql_05057,
        status_05057,
        usize_05057,
    );
}

#[inline(never)]
fn emit_05058() {
    let irql_05058 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05058 = NtStatus::from(status_codes::STATUS_SUCCESS);
    let usize_05058 = returned_usize(5058);
    dummy_trace(
        ERROR,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-058: irql={}, status={}, usize={}",
        irql_05058,
        status_05058,
        usize_05058,
    );
}

#[inline(never)]
fn emit_05059() {
    let irql_05059 = IRQL::current().unwrap_or(IRQL::Passive);
    let status_05059 = NtStatus::from(status_codes::STATUS_ACCESS_DENIED);
    let usize_05059 = returned_usize(5059);
    dummy_trace(
        CRITICAL,
        ENUM_PRIMARY,
        "fixture-1000 scenario-05 instance-059: irql={}, status={}, usize={}",
        irql_05059,
        status_05059,
        usize_05059,
    );
}

pub(crate) fn emit() {
    emit_05050();
    emit_05051();
    emit_05052();
    emit_05053();
    emit_05054();
    emit_05055();
    emit_05056();
    emit_05057();
    emit_05058();
    emit_05059();
}

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
        "fixture-10 scenario-05 instance-000: irql={}, status={}, usize={}",
        irql_05000,
        status_05000,
        usize_05000
    );
}

pub(crate) fn emit() {
    emit_05000();
}

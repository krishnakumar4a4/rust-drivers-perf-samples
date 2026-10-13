use crate::dummy_trace::*;

#[inline(never)]
fn emit_07000<T, U, V>(generic_t_07000: T, generic_u_07000: U, generic_v_07000: V) {
    dummy_trace(
        WARNING,
        GENERIC,
        "fixture-10 scenario-07 instance-000: generic-t={}, generic-u={}, generic-v={}",
        generic_t_07000,
        generic_u_07000,
        generic_v_07000,
    );
}

pub(crate) fn emit() {
    emit_07000(-7_isize, "generic-000", 70_u32);
}

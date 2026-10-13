use crate::dummy_trace::*;

fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04000() {
    let str_04000 = "variable-04000";
    let c_str_04000 = c"c-string-04000";
    let returned_str_04000 = returned_text();
    dummy_trace(
        CRITICAL,
        STRINGS,
        "fixture-10 scenario-04 instance-000: str={}, cstr={}, returned={}",
        str_04000,
        c_str_04000,
        returned_str_04000,
    );
}

pub(crate) fn emit() {
    emit_04000();
}

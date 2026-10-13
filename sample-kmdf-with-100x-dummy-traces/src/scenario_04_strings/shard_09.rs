use crate::dummy_trace::*;

fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04090() {
    let str_04090 = "variable-04090";
    let c_str_04090 = c"c-string-04090";
    let returned_str_04090 = returned_text();
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-090: str={}, cstr={}, returned={}",
        str_04090,
        c_str_04090,
        returned_str_04090,
    );
}

#[inline(never)]
fn emit_04091() {
    let str_04091 = "variable-04091";
    let c_str_04091 = c"c-string-04091";
    let returned_str_04091 = returned_text();
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-091: str={}, cstr={}, returned={}",
        str_04091,
        c_str_04091,
        returned_str_04091,
    );
}

#[inline(never)]
fn emit_04092() {
    let str_04092 = "variable-04092";
    let c_str_04092 = c"c-string-04092";
    let returned_str_04092 = returned_text();
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-092: str={}, cstr={}, returned={}",
        str_04092,
        c_str_04092,
        returned_str_04092,
    );
}

#[inline(never)]
fn emit_04093() {
    let str_04093 = "variable-04093";
    let c_str_04093 = c"c-string-04093";
    let returned_str_04093 = returned_text();
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-093: str={}, cstr={}, returned={}",
        str_04093,
        c_str_04093,
        returned_str_04093,
    );
}

#[inline(never)]
fn emit_04094() {
    let str_04094 = "variable-04094";
    let c_str_04094 = c"c-string-04094";
    let returned_str_04094 = returned_text();
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-094: str={}, cstr={}, returned={}",
        str_04094,
        c_str_04094,
        returned_str_04094,
    );
}

#[inline(never)]
fn emit_04095() {
    let str_04095 = "variable-04095";
    let c_str_04095 = c"c-string-04095";
    let returned_str_04095 = returned_text();
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-04 instance-095: str={}, cstr={}, returned={}",
        str_04095,
        c_str_04095,
        returned_str_04095,
    );
}

#[inline(never)]
fn emit_04096() {
    let str_04096 = "variable-04096";
    let c_str_04096 = c"c-string-04096";
    let returned_str_04096 = returned_text();
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-04 instance-096: str={}, cstr={}, returned={}",
        str_04096,
        c_str_04096,
        returned_str_04096,
    );
}

#[inline(never)]
fn emit_04097() {
    let str_04097 = "variable-04097";
    let c_str_04097 = c"c-string-04097";
    let returned_str_04097 = returned_text();
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-04 instance-097: str={}, cstr={}, returned={}",
        str_04097,
        c_str_04097,
        returned_str_04097,
    );
}

#[inline(never)]
fn emit_04098() {
    let str_04098 = "variable-04098";
    let c_str_04098 = c"c-string-04098";
    let returned_str_04098 = returned_text();
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-04 instance-098: str={}, cstr={}, returned={}",
        str_04098,
        c_str_04098,
        returned_str_04098,
    );
}

#[inline(never)]
fn emit_04099() {
    let str_04099 = "variable-04099";
    let c_str_04099 = c"c-string-04099";
    let returned_str_04099 = returned_text();
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-04 instance-099: str={}, cstr={}, returned={}",
        str_04099,
        c_str_04099,
        returned_str_04099,
    );
}

pub(crate) fn emit() {
    emit_04090();
    emit_04091();
    emit_04092();
    emit_04093();
    emit_04094();
    emit_04095();
    emit_04096();
    emit_04097();
    emit_04098();
    emit_04099();
}

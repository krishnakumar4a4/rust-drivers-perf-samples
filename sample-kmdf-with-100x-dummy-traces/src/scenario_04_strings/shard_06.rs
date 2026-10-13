use crate::dummy_trace::*;

fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04060() {
    let str_04060 = "variable-04060";
    let c_str_04060 = c"c-string-04060";
    let returned_str_04060 = returned_text();
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-04 instance-060: str={}, cstr={}, returned={}",
        str_04060,
        c_str_04060,
        returned_str_04060,
    );
}

#[inline(never)]
fn emit_04061() {
    let str_04061 = "variable-04061";
    let c_str_04061 = c"c-string-04061";
    let returned_str_04061 = returned_text();
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-04 instance-061: str={}, cstr={}, returned={}",
        str_04061,
        c_str_04061,
        returned_str_04061,
    );
}

#[inline(never)]
fn emit_04062() {
    let str_04062 = "variable-04062";
    let c_str_04062 = c"c-string-04062";
    let returned_str_04062 = returned_text();
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-04 instance-062: str={}, cstr={}, returned={}",
        str_04062,
        c_str_04062,
        returned_str_04062,
    );
}

#[inline(never)]
fn emit_04063() {
    let str_04063 = "variable-04063";
    let c_str_04063 = c"c-string-04063";
    let returned_str_04063 = returned_text();
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-04 instance-063: str={}, cstr={}, returned={}",
        str_04063,
        c_str_04063,
        returned_str_04063,
    );
}

#[inline(never)]
fn emit_04064() {
    let str_04064 = "variable-04064";
    let c_str_04064 = c"c-string-04064";
    let returned_str_04064 = returned_text();
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-04 instance-064: str={}, cstr={}, returned={}",
        str_04064,
        c_str_04064,
        returned_str_04064,
    );
}

#[inline(never)]
fn emit_04065() {
    let str_04065 = "variable-04065";
    let c_str_04065 = c"c-string-04065";
    let returned_str_04065 = returned_text();
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-065: str={}, cstr={}, returned={}",
        str_04065,
        c_str_04065,
        returned_str_04065,
    );
}

#[inline(never)]
fn emit_04066() {
    let str_04066 = "variable-04066";
    let c_str_04066 = c"c-string-04066";
    let returned_str_04066 = returned_text();
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-066: str={}, cstr={}, returned={}",
        str_04066,
        c_str_04066,
        returned_str_04066,
    );
}

#[inline(never)]
fn emit_04067() {
    let str_04067 = "variable-04067";
    let c_str_04067 = c"c-string-04067";
    let returned_str_04067 = returned_text();
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-067: str={}, cstr={}, returned={}",
        str_04067,
        c_str_04067,
        returned_str_04067,
    );
}

#[inline(never)]
fn emit_04068() {
    let str_04068 = "variable-04068";
    let c_str_04068 = c"c-string-04068";
    let returned_str_04068 = returned_text();
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-068: str={}, cstr={}, returned={}",
        str_04068,
        c_str_04068,
        returned_str_04068,
    );
}

#[inline(never)]
fn emit_04069() {
    let str_04069 = "variable-04069";
    let c_str_04069 = c"c-string-04069";
    let returned_str_04069 = returned_text();
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-069: str={}, cstr={}, returned={}",
        str_04069,
        c_str_04069,
        returned_str_04069,
    );
}

pub(crate) fn emit() {
    emit_04060();
    emit_04061();
    emit_04062();
    emit_04063();
    emit_04064();
    emit_04065();
    emit_04066();
    emit_04067();
    emit_04068();
    emit_04069();
}

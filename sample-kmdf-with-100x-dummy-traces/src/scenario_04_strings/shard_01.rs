use crate::dummy_trace::*;

fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04010() {
    let str_04010 = "variable-04010";
    let c_str_04010 = c"c-string-04010";
    let returned_str_04010 = returned_text();
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-04 instance-010: str={}, cstr={}, returned={}",
        str_04010,
        c_str_04010,
        returned_str_04010,
    );
}

#[inline(never)]
fn emit_04011() {
    let str_04011 = "variable-04011";
    let c_str_04011 = c"c-string-04011";
    let returned_str_04011 = returned_text();
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-04 instance-011: str={}, cstr={}, returned={}",
        str_04011,
        c_str_04011,
        returned_str_04011,
    );
}

#[inline(never)]
fn emit_04012() {
    let str_04012 = "variable-04012";
    let c_str_04012 = c"c-string-04012";
    let returned_str_04012 = returned_text();
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-04 instance-012: str={}, cstr={}, returned={}",
        str_04012,
        c_str_04012,
        returned_str_04012,
    );
}

#[inline(never)]
fn emit_04013() {
    let str_04013 = "variable-04013";
    let c_str_04013 = c"c-string-04013";
    let returned_str_04013 = returned_text();
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-04 instance-013: str={}, cstr={}, returned={}",
        str_04013,
        c_str_04013,
        returned_str_04013,
    );
}

#[inline(never)]
fn emit_04014() {
    let str_04014 = "variable-04014";
    let c_str_04014 = c"c-string-04014";
    let returned_str_04014 = returned_text();
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-04 instance-014: str={}, cstr={}, returned={}",
        str_04014,
        c_str_04014,
        returned_str_04014,
    );
}

#[inline(never)]
fn emit_04015() {
    let str_04015 = "variable-04015";
    let c_str_04015 = c"c-string-04015";
    let returned_str_04015 = returned_text();
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-015: str={}, cstr={}, returned={}",
        str_04015,
        c_str_04015,
        returned_str_04015,
    );
}

#[inline(never)]
fn emit_04016() {
    let str_04016 = "variable-04016";
    let c_str_04016 = c"c-string-04016";
    let returned_str_04016 = returned_text();
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-016: str={}, cstr={}, returned={}",
        str_04016,
        c_str_04016,
        returned_str_04016,
    );
}

#[inline(never)]
fn emit_04017() {
    let str_04017 = "variable-04017";
    let c_str_04017 = c"c-string-04017";
    let returned_str_04017 = returned_text();
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-017: str={}, cstr={}, returned={}",
        str_04017,
        c_str_04017,
        returned_str_04017,
    );
}

#[inline(never)]
fn emit_04018() {
    let str_04018 = "variable-04018";
    let c_str_04018 = c"c-string-04018";
    let returned_str_04018 = returned_text();
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-018: str={}, cstr={}, returned={}",
        str_04018,
        c_str_04018,
        returned_str_04018,
    );
}

#[inline(never)]
fn emit_04019() {
    let str_04019 = "variable-04019";
    let c_str_04019 = c"c-string-04019";
    let returned_str_04019 = returned_text();
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-019: str={}, cstr={}, returned={}",
        str_04019,
        c_str_04019,
        returned_str_04019,
    );
}

pub(crate) fn emit() {
    emit_04010();
    emit_04011();
    emit_04012();
    emit_04013();
    emit_04014();
    emit_04015();
    emit_04016();
    emit_04017();
    emit_04018();
    emit_04019();
}

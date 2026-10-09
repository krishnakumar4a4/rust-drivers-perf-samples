fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04050() {
    let str_04050 = "variable-04050";
    let c_str_04050 = c"c-string-04050";
    let returned_str_04050 = returned_text();
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-04 instance-050: str={}, cstr={}, returned={}",
        str_04050,
        c_str_04050,
        returned_str_04050
    );
}

#[inline(never)]
fn emit_04051() {
    let str_04051 = "variable-04051";
    let c_str_04051 = c"c-string-04051";
    let returned_str_04051 = returned_text();
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-04 instance-051: str={}, cstr={}, returned={}",
        str_04051,
        c_str_04051,
        returned_str_04051
    );
}

#[inline(never)]
fn emit_04052() {
    let str_04052 = "variable-04052";
    let c_str_04052 = c"c-string-04052";
    let returned_str_04052 = returned_text();
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-04 instance-052: str={}, cstr={}, returned={}",
        str_04052,
        c_str_04052,
        returned_str_04052
    );
}

#[inline(never)]
fn emit_04053() {
    let str_04053 = "variable-04053";
    let c_str_04053 = c"c-string-04053";
    let returned_str_04053 = returned_text();
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-04 instance-053: str={}, cstr={}, returned={}",
        str_04053,
        c_str_04053,
        returned_str_04053
    );
}

#[inline(never)]
fn emit_04054() {
    let str_04054 = "variable-04054";
    let c_str_04054 = c"c-string-04054";
    let returned_str_04054 = returned_text();
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-04 instance-054: str={}, cstr={}, returned={}",
        str_04054,
        c_str_04054,
        returned_str_04054
    );
}

#[inline(never)]
fn emit_04055() {
    let str_04055 = "variable-04055";
    let c_str_04055 = c"c-string-04055";
    let returned_str_04055 = returned_text();
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-04 instance-055: str={}, cstr={}, returned={}",
        str_04055,
        c_str_04055,
        returned_str_04055
    );
}

#[inline(never)]
fn emit_04056() {
    let str_04056 = "variable-04056";
    let c_str_04056 = c"c-string-04056";
    let returned_str_04056 = returned_text();
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-04 instance-056: str={}, cstr={}, returned={}",
        str_04056,
        c_str_04056,
        returned_str_04056
    );
}

#[inline(never)]
fn emit_04057() {
    let str_04057 = "variable-04057";
    let c_str_04057 = c"c-string-04057";
    let returned_str_04057 = returned_text();
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-04 instance-057: str={}, cstr={}, returned={}",
        str_04057,
        c_str_04057,
        returned_str_04057
    );
}

#[inline(never)]
fn emit_04058() {
    let str_04058 = "variable-04058";
    let c_str_04058 = c"c-string-04058";
    let returned_str_04058 = returned_text();
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-04 instance-058: str={}, cstr={}, returned={}",
        str_04058,
        c_str_04058,
        returned_str_04058
    );
}

#[inline(never)]
fn emit_04059() {
    let str_04059 = "variable-04059";
    let c_str_04059 = c"c-string-04059";
    let returned_str_04059 = returned_text();
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-04 instance-059: str={}, cstr={}, returned={}",
        str_04059,
        c_str_04059,
        returned_str_04059
    );
}

pub(crate) fn emit() {
    emit_04050();
    emit_04051();
    emit_04052();
    emit_04053();
    emit_04054();
    emit_04055();
    emit_04056();
    emit_04057();
    emit_04058();
    emit_04059();
}

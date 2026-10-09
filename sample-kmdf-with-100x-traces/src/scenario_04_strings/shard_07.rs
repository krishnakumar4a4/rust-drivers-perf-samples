fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04070() {
    let str_04070 = "variable-04070";
    let c_str_04070 = c"c-string-04070";
    let returned_str_04070 = returned_text();
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-04 instance-070: str={}, cstr={}, returned={}",
        str_04070,
        c_str_04070,
        returned_str_04070
    );
}

#[inline(never)]
fn emit_04071() {
    let str_04071 = "variable-04071";
    let c_str_04071 = c"c-string-04071";
    let returned_str_04071 = returned_text();
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-04 instance-071: str={}, cstr={}, returned={}",
        str_04071,
        c_str_04071,
        returned_str_04071
    );
}

#[inline(never)]
fn emit_04072() {
    let str_04072 = "variable-04072";
    let c_str_04072 = c"c-string-04072";
    let returned_str_04072 = returned_text();
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-04 instance-072: str={}, cstr={}, returned={}",
        str_04072,
        c_str_04072,
        returned_str_04072
    );
}

#[inline(never)]
fn emit_04073() {
    let str_04073 = "variable-04073";
    let c_str_04073 = c"c-string-04073";
    let returned_str_04073 = returned_text();
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-04 instance-073: str={}, cstr={}, returned={}",
        str_04073,
        c_str_04073,
        returned_str_04073
    );
}

#[inline(never)]
fn emit_04074() {
    let str_04074 = "variable-04074";
    let c_str_04074 = c"c-string-04074";
    let returned_str_04074 = returned_text();
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-04 instance-074: str={}, cstr={}, returned={}",
        str_04074,
        c_str_04074,
        returned_str_04074
    );
}

#[inline(never)]
fn emit_04075() {
    let str_04075 = "variable-04075";
    let c_str_04075 = c"c-string-04075";
    let returned_str_04075 = returned_text();
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-04 instance-075: str={}, cstr={}, returned={}",
        str_04075,
        c_str_04075,
        returned_str_04075
    );
}

#[inline(never)]
fn emit_04076() {
    let str_04076 = "variable-04076";
    let c_str_04076 = c"c-string-04076";
    let returned_str_04076 = returned_text();
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-04 instance-076: str={}, cstr={}, returned={}",
        str_04076,
        c_str_04076,
        returned_str_04076
    );
}

#[inline(never)]
fn emit_04077() {
    let str_04077 = "variable-04077";
    let c_str_04077 = c"c-string-04077";
    let returned_str_04077 = returned_text();
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-04 instance-077: str={}, cstr={}, returned={}",
        str_04077,
        c_str_04077,
        returned_str_04077
    );
}

#[inline(never)]
fn emit_04078() {
    let str_04078 = "variable-04078";
    let c_str_04078 = c"c-string-04078";
    let returned_str_04078 = returned_text();
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-04 instance-078: str={}, cstr={}, returned={}",
        str_04078,
        c_str_04078,
        returned_str_04078
    );
}

#[inline(never)]
fn emit_04079() {
    let str_04079 = "variable-04079";
    let c_str_04079 = c"c-string-04079";
    let returned_str_04079 = returned_text();
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-04 instance-079: str={}, cstr={}, returned={}",
        str_04079,
        c_str_04079,
        returned_str_04079
    );
}

pub(crate) fn emit() {
    emit_04070();
    emit_04071();
    emit_04072();
    emit_04073();
    emit_04074();
    emit_04075();
    emit_04076();
    emit_04077();
    emit_04078();
    emit_04079();
}

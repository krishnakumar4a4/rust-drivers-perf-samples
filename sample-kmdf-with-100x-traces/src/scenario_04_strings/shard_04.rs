fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04040() {
    let str_04040 = "variable-04040";
    let c_str_04040 = c"c-string-04040";
    let returned_str_04040 = returned_text();
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-040: str={}, cstr={}, returned={}",
        str_04040,
        c_str_04040,
        returned_str_04040
    );
}

#[inline(never)]
fn emit_04041() {
    let str_04041 = "variable-04041";
    let c_str_04041 = c"c-string-04041";
    let returned_str_04041 = returned_text();
    trace!(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-041: str={}, cstr={}, returned={}",
        str_04041,
        c_str_04041,
        returned_str_04041
    );
}

#[inline(never)]
fn emit_04042() {
    let str_04042 = "variable-04042";
    let c_str_04042 = c"c-string-04042";
    let returned_str_04042 = returned_text();
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-042: str={}, cstr={}, returned={}",
        str_04042,
        c_str_04042,
        returned_str_04042
    );
}

#[inline(never)]
fn emit_04043() {
    let str_04043 = "variable-04043";
    let c_str_04043 = c"c-string-04043";
    let returned_str_04043 = returned_text();
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-043: str={}, cstr={}, returned={}",
        str_04043,
        c_str_04043,
        returned_str_04043
    );
}

#[inline(never)]
fn emit_04044() {
    let str_04044 = "variable-04044";
    let c_str_04044 = c"c-string-04044";
    let returned_str_04044 = returned_text();
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-04 instance-044: str={}, cstr={}, returned={}",
        str_04044,
        c_str_04044,
        returned_str_04044
    );
}

#[inline(never)]
fn emit_04045() {
    let str_04045 = "variable-04045";
    let c_str_04045 = c"c-string-04045";
    let returned_str_04045 = returned_text();
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-04 instance-045: str={}, cstr={}, returned={}",
        str_04045,
        c_str_04045,
        returned_str_04045
    );
}

#[inline(never)]
fn emit_04046() {
    let str_04046 = "variable-04046";
    let c_str_04046 = c"c-string-04046";
    let returned_str_04046 = returned_text();
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-04 instance-046: str={}, cstr={}, returned={}",
        str_04046,
        c_str_04046,
        returned_str_04046
    );
}

#[inline(never)]
fn emit_04047() {
    let str_04047 = "variable-04047";
    let c_str_04047 = c"c-string-04047";
    let returned_str_04047 = returned_text();
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-04 instance-047: str={}, cstr={}, returned={}",
        str_04047,
        c_str_04047,
        returned_str_04047
    );
}

#[inline(never)]
fn emit_04048() {
    let str_04048 = "variable-04048";
    let c_str_04048 = c"c-string-04048";
    let returned_str_04048 = returned_text();
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-04 instance-048: str={}, cstr={}, returned={}",
        str_04048,
        c_str_04048,
        returned_str_04048
    );
}

#[inline(never)]
fn emit_04049() {
    let str_04049 = "variable-04049";
    let c_str_04049 = c"c-string-04049";
    let returned_str_04049 = returned_text();
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-04 instance-049: str={}, cstr={}, returned={}",
        str_04049,
        c_str_04049,
        returned_str_04049
    );
}

pub(crate) fn emit() {
    emit_04040();
    emit_04041();
    emit_04042();
    emit_04043();
    emit_04044();
    emit_04045();
    emit_04046();
    emit_04047();
    emit_04048();
    emit_04049();
}

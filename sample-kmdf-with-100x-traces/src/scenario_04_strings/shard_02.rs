fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04020() {
    let str_04020 = "variable-04020";
    let c_str_04020 = c"c-string-04020";
    let returned_str_04020 = returned_text();
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-04 instance-020: str={}, cstr={}, returned={}",
        str_04020,
        c_str_04020,
        returned_str_04020
    );
}

#[inline(never)]
fn emit_04021() {
    let str_04021 = "variable-04021";
    let c_str_04021 = c"c-string-04021";
    let returned_str_04021 = returned_text();
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-04 instance-021: str={}, cstr={}, returned={}",
        str_04021,
        c_str_04021,
        returned_str_04021
    );
}

#[inline(never)]
fn emit_04022() {
    let str_04022 = "variable-04022";
    let c_str_04022 = c"c-string-04022";
    let returned_str_04022 = returned_text();
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-04 instance-022: str={}, cstr={}, returned={}",
        str_04022,
        c_str_04022,
        returned_str_04022
    );
}

#[inline(never)]
fn emit_04023() {
    let str_04023 = "variable-04023";
    let c_str_04023 = c"c-string-04023";
    let returned_str_04023 = returned_text();
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-04 instance-023: str={}, cstr={}, returned={}",
        str_04023,
        c_str_04023,
        returned_str_04023
    );
}

#[inline(never)]
fn emit_04024() {
    let str_04024 = "variable-04024";
    let c_str_04024 = c"c-string-04024";
    let returned_str_04024 = returned_text();
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-04 instance-024: str={}, cstr={}, returned={}",
        str_04024,
        c_str_04024,
        returned_str_04024
    );
}

#[inline(never)]
fn emit_04025() {
    let str_04025 = "variable-04025";
    let c_str_04025 = c"c-string-04025";
    let returned_str_04025 = returned_text();
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-04 instance-025: str={}, cstr={}, returned={}",
        str_04025,
        c_str_04025,
        returned_str_04025
    );
}

#[inline(never)]
fn emit_04026() {
    let str_04026 = "variable-04026";
    let c_str_04026 = c"c-string-04026";
    let returned_str_04026 = returned_text();
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-04 instance-026: str={}, cstr={}, returned={}",
        str_04026,
        c_str_04026,
        returned_str_04026
    );
}

#[inline(never)]
fn emit_04027() {
    let str_04027 = "variable-04027";
    let c_str_04027 = c"c-string-04027";
    let returned_str_04027 = returned_text();
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-04 instance-027: str={}, cstr={}, returned={}",
        str_04027,
        c_str_04027,
        returned_str_04027
    );
}

#[inline(never)]
fn emit_04028() {
    let str_04028 = "variable-04028";
    let c_str_04028 = c"c-string-04028";
    let returned_str_04028 = returned_text();
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-04 instance-028: str={}, cstr={}, returned={}",
        str_04028,
        c_str_04028,
        returned_str_04028
    );
}

#[inline(never)]
fn emit_04029() {
    let str_04029 = "variable-04029";
    let c_str_04029 = c"c-string-04029";
    let returned_str_04029 = returned_text();
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-04 instance-029: str={}, cstr={}, returned={}",
        str_04029,
        c_str_04029,
        returned_str_04029
    );
}

pub(crate) fn emit() {
    emit_04020();
    emit_04021();
    emit_04022();
    emit_04023();
    emit_04024();
    emit_04025();
    emit_04026();
    emit_04027();
    emit_04028();
    emit_04029();
}

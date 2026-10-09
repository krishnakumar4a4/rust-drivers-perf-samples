fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04000() {
    let str_04000 = "variable-04000";
    let c_str_04000 = c"c-string-04000";
    let returned_str_04000 = returned_text();
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-04 instance-000: str={}, cstr={}, returned={}",
        str_04000,
        c_str_04000,
        returned_str_04000
    );
}

#[inline(never)]
fn emit_04001() {
    let str_04001 = "variable-04001";
    let c_str_04001 = c"c-string-04001";
    let returned_str_04001 = returned_text();
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-04 instance-001: str={}, cstr={}, returned={}",
        str_04001,
        c_str_04001,
        returned_str_04001
    );
}

#[inline(never)]
fn emit_04002() {
    let str_04002 = "variable-04002";
    let c_str_04002 = c"c-string-04002";
    let returned_str_04002 = returned_text();
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-04 instance-002: str={}, cstr={}, returned={}",
        str_04002,
        c_str_04002,
        returned_str_04002
    );
}

#[inline(never)]
fn emit_04003() {
    let str_04003 = "variable-04003";
    let c_str_04003 = c"c-string-04003";
    let returned_str_04003 = returned_text();
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-04 instance-003: str={}, cstr={}, returned={}",
        str_04003,
        c_str_04003,
        returned_str_04003
    );
}

#[inline(never)]
fn emit_04004() {
    let str_04004 = "variable-04004";
    let c_str_04004 = c"c-string-04004";
    let returned_str_04004 = returned_text();
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-04 instance-004: str={}, cstr={}, returned={}",
        str_04004,
        c_str_04004,
        returned_str_04004
    );
}

#[inline(never)]
fn emit_04005() {
    let str_04005 = "variable-04005";
    let c_str_04005 = c"c-string-04005";
    let returned_str_04005 = returned_text();
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-04 instance-005: str={}, cstr={}, returned={}",
        str_04005,
        c_str_04005,
        returned_str_04005
    );
}

#[inline(never)]
fn emit_04006() {
    let str_04006 = "variable-04006";
    let c_str_04006 = c"c-string-04006";
    let returned_str_04006 = returned_text();
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-04 instance-006: str={}, cstr={}, returned={}",
        str_04006,
        c_str_04006,
        returned_str_04006
    );
}

#[inline(never)]
fn emit_04007() {
    let str_04007 = "variable-04007";
    let c_str_04007 = c"c-string-04007";
    let returned_str_04007 = returned_text();
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-04 instance-007: str={}, cstr={}, returned={}",
        str_04007,
        c_str_04007,
        returned_str_04007
    );
}

#[inline(never)]
fn emit_04008() {
    let str_04008 = "variable-04008";
    let c_str_04008 = c"c-string-04008";
    let returned_str_04008 = returned_text();
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-04 instance-008: str={}, cstr={}, returned={}",
        str_04008,
        c_str_04008,
        returned_str_04008
    );
}

#[inline(never)]
fn emit_04009() {
    let str_04009 = "variable-04009";
    let c_str_04009 = c"c-string-04009";
    let returned_str_04009 = returned_text();
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-04 instance-009: str={}, cstr={}, returned={}",
        str_04009,
        c_str_04009,
        returned_str_04009
    );
}

pub(crate) fn emit() {
    emit_04000();
    emit_04001();
    emit_04002();
    emit_04003();
    emit_04004();
    emit_04005();
    emit_04006();
    emit_04007();
    emit_04008();
    emit_04009();
}

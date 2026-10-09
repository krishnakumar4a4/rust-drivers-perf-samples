fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04030() {
    let str_04030 = "variable-04030";
    let c_str_04030 = c"c-string-04030";
    let returned_str_04030 = returned_text();
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-04 instance-030: str={}, cstr={}, returned={}",
        str_04030,
        c_str_04030,
        returned_str_04030
    );
}

#[inline(never)]
fn emit_04031() {
    let str_04031 = "variable-04031";
    let c_str_04031 = c"c-string-04031";
    let returned_str_04031 = returned_text();
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-04 instance-031: str={}, cstr={}, returned={}",
        str_04031,
        c_str_04031,
        returned_str_04031
    );
}

#[inline(never)]
fn emit_04032() {
    let str_04032 = "variable-04032";
    let c_str_04032 = c"c-string-04032";
    let returned_str_04032 = returned_text();
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-04 instance-032: str={}, cstr={}, returned={}",
        str_04032,
        c_str_04032,
        returned_str_04032
    );
}

#[inline(never)]
fn emit_04033() {
    let str_04033 = "variable-04033";
    let c_str_04033 = c"c-string-04033";
    let returned_str_04033 = returned_text();
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-04 instance-033: str={}, cstr={}, returned={}",
        str_04033,
        c_str_04033,
        returned_str_04033
    );
}

#[inline(never)]
fn emit_04034() {
    let str_04034 = "variable-04034";
    let c_str_04034 = c"c-string-04034";
    let returned_str_04034 = returned_text();
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-04 instance-034: str={}, cstr={}, returned={}",
        str_04034,
        c_str_04034,
        returned_str_04034
    );
}

#[inline(never)]
fn emit_04035() {
    let str_04035 = "variable-04035";
    let c_str_04035 = c"c-string-04035";
    let returned_str_04035 = returned_text();
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-04 instance-035: str={}, cstr={}, returned={}",
        str_04035,
        c_str_04035,
        returned_str_04035
    );
}

#[inline(never)]
fn emit_04036() {
    let str_04036 = "variable-04036";
    let c_str_04036 = c"c-string-04036";
    let returned_str_04036 = returned_text();
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-04 instance-036: str={}, cstr={}, returned={}",
        str_04036,
        c_str_04036,
        returned_str_04036
    );
}

#[inline(never)]
fn emit_04037() {
    let str_04037 = "variable-04037";
    let c_str_04037 = c"c-string-04037";
    let returned_str_04037 = returned_text();
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-04 instance-037: str={}, cstr={}, returned={}",
        str_04037,
        c_str_04037,
        returned_str_04037
    );
}

#[inline(never)]
fn emit_04038() {
    let str_04038 = "variable-04038";
    let c_str_04038 = c"c-string-04038";
    let returned_str_04038 = returned_text();
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-04 instance-038: str={}, cstr={}, returned={}",
        str_04038,
        c_str_04038,
        returned_str_04038
    );
}

#[inline(never)]
fn emit_04039() {
    let str_04039 = "variable-04039";
    let c_str_04039 = c"c-string-04039";
    let returned_str_04039 = returned_text();
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-04 instance-039: str={}, cstr={}, returned={}",
        str_04039,
        c_str_04039,
        returned_str_04039
    );
}

pub(crate) fn emit() {
    emit_04030();
    emit_04031();
    emit_04032();
    emit_04033();
    emit_04034();
    emit_04035();
    emit_04036();
    emit_04037();
    emit_04038();
    emit_04039();
}

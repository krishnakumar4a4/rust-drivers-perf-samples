fn returned_text() -> &'static str {
    "returned-text"
}

#[inline(never)]
fn emit_04080() {
    let str_04080 = "variable-04080";
    let c_str_04080 = c"c-string-04080";
    let returned_str_04080 = returned_text();
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-04 instance-080: str={}, cstr={}, returned={}",
        str_04080,
        c_str_04080,
        returned_str_04080
    );
}

#[inline(never)]
fn emit_04081() {
    let str_04081 = "variable-04081";
    let c_str_04081 = c"c-string-04081";
    let returned_str_04081 = returned_text();
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-04 instance-081: str={}, cstr={}, returned={}",
        str_04081,
        c_str_04081,
        returned_str_04081
    );
}

#[inline(never)]
fn emit_04082() {
    let str_04082 = "variable-04082";
    let c_str_04082 = c"c-string-04082";
    let returned_str_04082 = returned_text();
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-04 instance-082: str={}, cstr={}, returned={}",
        str_04082,
        c_str_04082,
        returned_str_04082
    );
}

#[inline(never)]
fn emit_04083() {
    let str_04083 = "variable-04083";
    let c_str_04083 = c"c-string-04083";
    let returned_str_04083 = returned_text();
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-04 instance-083: str={}, cstr={}, returned={}",
        str_04083,
        c_str_04083,
        returned_str_04083
    );
}

#[inline(never)]
fn emit_04084() {
    let str_04084 = "variable-04084";
    let c_str_04084 = c"c-string-04084";
    let returned_str_04084 = returned_text();
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-04 instance-084: str={}, cstr={}, returned={}",
        str_04084,
        c_str_04084,
        returned_str_04084
    );
}

#[inline(never)]
fn emit_04085() {
    let str_04085 = "variable-04085";
    let c_str_04085 = c"c-string-04085";
    let returned_str_04085 = returned_text();
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-04 instance-085: str={}, cstr={}, returned={}",
        str_04085,
        c_str_04085,
        returned_str_04085
    );
}

#[inline(never)]
fn emit_04086() {
    let str_04086 = "variable-04086";
    let c_str_04086 = c"c-string-04086";
    let returned_str_04086 = returned_text();
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-04 instance-086: str={}, cstr={}, returned={}",
        str_04086,
        c_str_04086,
        returned_str_04086
    );
}

#[inline(never)]
fn emit_04087() {
    let str_04087 = "variable-04087";
    let c_str_04087 = c"c-string-04087";
    let returned_str_04087 = returned_text();
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-04 instance-087: str={}, cstr={}, returned={}",
        str_04087,
        c_str_04087,
        returned_str_04087
    );
}

#[inline(never)]
fn emit_04088() {
    let str_04088 = "variable-04088";
    let c_str_04088 = c"c-string-04088";
    let returned_str_04088 = returned_text();
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-04 instance-088: str={}, cstr={}, returned={}",
        str_04088,
        c_str_04088,
        returned_str_04088
    );
}

#[inline(never)]
fn emit_04089() {
    let str_04089 = "variable-04089";
    let c_str_04089 = c"c-string-04089";
    let returned_str_04089 = returned_text();
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-04 instance-089: str={}, cstr={}, returned={}",
        str_04089,
        c_str_04089,
        returned_str_04089
    );
}

pub(crate) fn emit() {
    emit_04080();
    emit_04081();
    emit_04082();
    emit_04083();
    emit_04084();
    emit_04085();
    emit_04086();
    emit_04087();
    emit_04088();
    emit_04089();
}

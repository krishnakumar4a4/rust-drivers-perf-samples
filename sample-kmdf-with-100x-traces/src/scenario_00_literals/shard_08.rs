#[inline(never)]
fn emit_00080() {
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-00 instance-080: signed={}, unsigned={}, bool={}",
        -81_i8,
        60080_u16,
        true
    );
}

#[inline(never)]
fn emit_00081() {
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-00 instance-081: signed={}, unsigned={}, bool={}",
        -82_i8,
        60081_u16,
        false
    );
}

#[inline(never)]
fn emit_00082() {
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-00 instance-082: signed={}, unsigned={}, bool={}",
        -83_i8,
        60082_u16,
        true
    );
}

#[inline(never)]
fn emit_00083() {
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-00 instance-083: signed={}, unsigned={}, bool={}",
        -84_i8,
        60083_u16,
        false
    );
}

#[inline(never)]
fn emit_00084() {
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-00 instance-084: signed={}, unsigned={}, bool={}",
        -85_i8,
        60084_u16,
        true
    );
}

#[inline(never)]
fn emit_00085() {
    trace!(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-085: signed={}, unsigned={}, bool={}",
        -86_i8,
        60085_u16,
        false
    );
}

#[inline(never)]
fn emit_00086() {
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-086: signed={}, unsigned={}, bool={}",
        -87_i8,
        60086_u16,
        true
    );
}

#[inline(never)]
fn emit_00087() {
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-087: signed={}, unsigned={}, bool={}",
        -88_i8,
        60087_u16,
        false
    );
}

#[inline(never)]
fn emit_00088() {
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-088: signed={}, unsigned={}, bool={}",
        -89_i8,
        60088_u16,
        true
    );
}

#[inline(never)]
fn emit_00089() {
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-089: signed={}, unsigned={}, bool={}",
        -90_i8,
        60089_u16,
        false
    );
}

pub(crate) fn emit() {
    emit_00080();
    emit_00081();
    emit_00082();
    emit_00083();
    emit_00084();
    emit_00085();
    emit_00086();
    emit_00087();
    emit_00088();
    emit_00089();
}

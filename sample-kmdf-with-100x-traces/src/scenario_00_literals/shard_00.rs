#[inline(never)]
fn emit_00000() {
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-00 instance-000: signed={}, unsigned={}, bool={}",
        -1_i8,
        60000_u16,
        true
    );
}

#[inline(never)]
fn emit_00001() {
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-00 instance-001: signed={}, unsigned={}, bool={}",
        -2_i8,
        60001_u16,
        false
    );
}

#[inline(never)]
fn emit_00002() {
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-00 instance-002: signed={}, unsigned={}, bool={}",
        -3_i8,
        60002_u16,
        true
    );
}

#[inline(never)]
fn emit_00003() {
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-00 instance-003: signed={}, unsigned={}, bool={}",
        -4_i8,
        60003_u16,
        false
    );
}

#[inline(never)]
fn emit_00004() {
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-00 instance-004: signed={}, unsigned={}, bool={}",
        -5_i8,
        60004_u16,
        true
    );
}

#[inline(never)]
fn emit_00005() {
    trace!(
        INFO,
        SIGNED,
        "fixture-1000 scenario-00 instance-005: signed={}, unsigned={}, bool={}",
        -6_i8,
        60005_u16,
        false
    );
}

#[inline(never)]
fn emit_00006() {
    trace!(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-00 instance-006: signed={}, unsigned={}, bool={}",
        -7_i8,
        60006_u16,
        true
    );
}

#[inline(never)]
fn emit_00007() {
    trace!(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-00 instance-007: signed={}, unsigned={}, bool={}",
        -8_i8,
        60007_u16,
        false
    );
}

#[inline(never)]
fn emit_00008() {
    trace!(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-00 instance-008: signed={}, unsigned={}, bool={}",
        -9_i8,
        60008_u16,
        true
    );
}

#[inline(never)]
fn emit_00009() {
    trace!(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-00 instance-009: signed={}, unsigned={}, bool={}",
        -10_i8,
        60009_u16,
        false
    );
}

pub(crate) fn emit() {
    emit_00000();
    emit_00001();
    emit_00002();
    emit_00003();
    emit_00004();
    emit_00005();
    emit_00006();
    emit_00007();
    emit_00008();
    emit_00009();
}

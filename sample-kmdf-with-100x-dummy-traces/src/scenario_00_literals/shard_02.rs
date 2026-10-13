use crate::dummy_trace::*;

#[inline(never)]
fn emit_00020() {
    dummy_trace(
        INFO,
        STRINGS,
        "fixture-1000 scenario-00 instance-020: signed={}, unsigned={}, bool={}",
        -21_i8,
        60020_u16,
        true,
    );
}

#[inline(never)]
fn emit_00021() {
    dummy_trace(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-00 instance-021: signed={}, unsigned={}, bool={}",
        -22_i8,
        60021_u16,
        false,
    );
}

#[inline(never)]
fn emit_00022() {
    dummy_trace(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-00 instance-022: signed={}, unsigned={}, bool={}",
        -23_i8,
        60022_u16,
        true,
    );
}

#[inline(never)]
fn emit_00023() {
    dummy_trace(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-00 instance-023: signed={}, unsigned={}, bool={}",
        -24_i8,
        60023_u16,
        false,
    );
}

#[inline(never)]
fn emit_00024() {
    dummy_trace(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-00 instance-024: signed={}, unsigned={}, bool={}",
        -25_i8,
        60024_u16,
        true,
    );
}

#[inline(never)]
fn emit_00025() {
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-00 instance-025: signed={}, unsigned={}, bool={}",
        -26_i8,
        60025_u16,
        false,
    );
}

#[inline(never)]
fn emit_00026() {
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-00 instance-026: signed={}, unsigned={}, bool={}",
        -27_i8,
        60026_u16,
        true,
    );
}

#[inline(never)]
fn emit_00027() {
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-00 instance-027: signed={}, unsigned={}, bool={}",
        -28_i8,
        60027_u16,
        false,
    );
}

#[inline(never)]
fn emit_00028() {
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-00 instance-028: signed={}, unsigned={}, bool={}",
        -29_i8,
        60028_u16,
        true,
    );
}

#[inline(never)]
fn emit_00029() {
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-00 instance-029: signed={}, unsigned={}, bool={}",
        -30_i8,
        60029_u16,
        false,
    );
}

pub(crate) fn emit() {
    emit_00020();
    emit_00021();
    emit_00022();
    emit_00023();
    emit_00024();
    emit_00025();
    emit_00026();
    emit_00027();
    emit_00028();
    emit_00029();
}

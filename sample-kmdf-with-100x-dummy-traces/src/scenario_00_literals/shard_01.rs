use crate::dummy_trace::*;

#[inline(never)]
fn emit_00010() {
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-010: signed={}, unsigned={}, bool={}",
        -11_i8,
        60010_u16,
        true,
    );
}

#[inline(never)]
fn emit_00011() {
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-011: signed={}, unsigned={}, bool={}",
        -12_i8,
        60011_u16,
        false,
    );
}

#[inline(never)]
fn emit_00012() {
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-012: signed={}, unsigned={}, bool={}",
        -13_i8,
        60012_u16,
        true,
    );
}

#[inline(never)]
fn emit_00013() {
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-013: signed={}, unsigned={}, bool={}",
        -14_i8,
        60013_u16,
        false,
    );
}

#[inline(never)]
fn emit_00014() {
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-014: signed={}, unsigned={}, bool={}",
        -15_i8,
        60014_u16,
        true,
    );
}

#[inline(never)]
fn emit_00015() {
    dummy_trace(
        INFO,
        FLOATS,
        "fixture-1000 scenario-00 instance-015: signed={}, unsigned={}, bool={}",
        -16_i8,
        60015_u16,
        false,
    );
}

#[inline(never)]
fn emit_00016() {
    dummy_trace(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-00 instance-016: signed={}, unsigned={}, bool={}",
        -17_i8,
        60016_u16,
        true,
    );
}

#[inline(never)]
fn emit_00017() {
    dummy_trace(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-00 instance-017: signed={}, unsigned={}, bool={}",
        -18_i8,
        60017_u16,
        false,
    );
}

#[inline(never)]
fn emit_00018() {
    dummy_trace(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-00 instance-018: signed={}, unsigned={}, bool={}",
        -19_i8,
        60018_u16,
        true,
    );
}

#[inline(never)]
fn emit_00019() {
    dummy_trace(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-00 instance-019: signed={}, unsigned={}, bool={}",
        -20_i8,
        60019_u16,
        false,
    );
}

pub(crate) fn emit() {
    emit_00010();
    emit_00011();
    emit_00012();
    emit_00013();
    emit_00014();
    emit_00015();
    emit_00016();
    emit_00017();
    emit_00018();
    emit_00019();
}

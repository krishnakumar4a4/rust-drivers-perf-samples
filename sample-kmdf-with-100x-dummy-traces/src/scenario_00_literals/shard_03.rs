use crate::dummy_trace::*;

#[inline(never)]
fn emit_00030() {
    dummy_trace(
        INFO,
        SIGNED,
        "fixture-1000 scenario-00 instance-030: signed={}, unsigned={}, bool={}",
        -31_i8,
        60030_u16,
        true,
    );
}

#[inline(never)]
fn emit_00031() {
    dummy_trace(
        VERBOSE,
        SIGNED,
        "fixture-1000 scenario-00 instance-031: signed={}, unsigned={}, bool={}",
        -32_i8,
        60031_u16,
        false,
    );
}

#[inline(never)]
fn emit_00032() {
    dummy_trace(
        WARNING,
        SIGNED,
        "fixture-1000 scenario-00 instance-032: signed={}, unsigned={}, bool={}",
        -33_i8,
        60032_u16,
        true,
    );
}

#[inline(never)]
fn emit_00033() {
    dummy_trace(
        ERROR,
        SIGNED,
        "fixture-1000 scenario-00 instance-033: signed={}, unsigned={}, bool={}",
        -34_i8,
        60033_u16,
        false,
    );
}

#[inline(never)]
fn emit_00034() {
    dummy_trace(
        CRITICAL,
        SIGNED,
        "fixture-1000 scenario-00 instance-034: signed={}, unsigned={}, bool={}",
        -35_i8,
        60034_u16,
        true,
    );
}

#[inline(never)]
fn emit_00035() {
    dummy_trace(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-035: signed={}, unsigned={}, bool={}",
        -36_i8,
        60035_u16,
        false,
    );
}

#[inline(never)]
fn emit_00036() {
    dummy_trace(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-036: signed={}, unsigned={}, bool={}",
        -37_i8,
        60036_u16,
        true,
    );
}

#[inline(never)]
fn emit_00037() {
    dummy_trace(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-037: signed={}, unsigned={}, bool={}",
        -38_i8,
        60037_u16,
        false,
    );
}

#[inline(never)]
fn emit_00038() {
    dummy_trace(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-038: signed={}, unsigned={}, bool={}",
        -39_i8,
        60038_u16,
        true,
    );
}

#[inline(never)]
fn emit_00039() {
    dummy_trace(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-00 instance-039: signed={}, unsigned={}, bool={}",
        -40_i8,
        60039_u16,
        false,
    );
}

pub(crate) fn emit() {
    emit_00030();
    emit_00031();
    emit_00032();
    emit_00033();
    emit_00034();
    emit_00035();
    emit_00036();
    emit_00037();
    emit_00038();
    emit_00039();
}

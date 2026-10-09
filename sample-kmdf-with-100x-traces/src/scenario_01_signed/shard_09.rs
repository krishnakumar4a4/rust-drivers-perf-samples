#[inline(never)]
fn emit_01090(signed_i16_01090: i16, signed_i32_01090: i32, signed_i64_01090: i64) {
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-01 instance-090: i16={}, i32={}, i64={}",
        signed_i16_01090,
        signed_i32_01090,
        signed_i64_01090
    );
}

#[inline(never)]
fn emit_01091(signed_i16_01091: i16, signed_i32_01091: i32, signed_i64_01091: i64) {
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-01 instance-091: i16={}, i32={}, i64={}",
        signed_i16_01091,
        signed_i32_01091,
        signed_i64_01091
    );
}

#[inline(never)]
fn emit_01092(signed_i16_01092: i16, signed_i32_01092: i32, signed_i64_01092: i64) {
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-01 instance-092: i16={}, i32={}, i64={}",
        signed_i16_01092,
        signed_i32_01092,
        signed_i64_01092
    );
}

#[inline(never)]
fn emit_01093(signed_i16_01093: i16, signed_i32_01093: i32, signed_i64_01093: i64) {
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-01 instance-093: i16={}, i32={}, i64={}",
        signed_i16_01093,
        signed_i32_01093,
        signed_i64_01093
    );
}

#[inline(never)]
fn emit_01094(signed_i16_01094: i16, signed_i32_01094: i32, signed_i64_01094: i64) {
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-01 instance-094: i16={}, i32={}, i64={}",
        signed_i16_01094,
        signed_i32_01094,
        signed_i64_01094
    );
}

#[inline(never)]
fn emit_01095(signed_i16_01095: i16, signed_i32_01095: i32, signed_i64_01095: i64) {
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-01 instance-095: i16={}, i32={}, i64={}",
        signed_i16_01095,
        signed_i32_01095,
        signed_i64_01095
    );
}

#[inline(never)]
fn emit_01096(signed_i16_01096: i16, signed_i32_01096: i32, signed_i64_01096: i64) {
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-01 instance-096: i16={}, i32={}, i64={}",
        signed_i16_01096,
        signed_i32_01096,
        signed_i64_01096
    );
}

#[inline(never)]
fn emit_01097(signed_i16_01097: i16, signed_i32_01097: i32, signed_i64_01097: i64) {
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-01 instance-097: i16={}, i32={}, i64={}",
        signed_i16_01097,
        signed_i32_01097,
        signed_i64_01097
    );
}

#[inline(never)]
fn emit_01098(signed_i16_01098: i16, signed_i32_01098: i32, signed_i64_01098: i64) {
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-01 instance-098: i16={}, i32={}, i64={}",
        signed_i16_01098,
        signed_i32_01098,
        signed_i64_01098
    );
}

#[inline(never)]
fn emit_01099(signed_i16_01099: i16, signed_i32_01099: i32, signed_i64_01099: i64) {
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-01 instance-099: i16={}, i32={}, i64={}",
        signed_i16_01099,
        signed_i32_01099,
        signed_i64_01099
    );
}

pub(crate) fn emit() {
    emit_01090(-1690, -32090, -64090);
    emit_01091(-1691, -32091, -64091);
    emit_01092(-1692, -32092, -64092);
    emit_01093(-1693, -32093, -64093);
    emit_01094(-1694, -32094, -64094);
    emit_01095(-1695, -32095, -64095);
    emit_01096(-1696, -32096, -64096);
    emit_01097(-1697, -32097, -64097);
    emit_01098(-1698, -32098, -64098);
    emit_01099(-1699, -32099, -64099);
}

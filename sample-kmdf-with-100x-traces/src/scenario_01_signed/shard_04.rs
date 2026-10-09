#[inline(never)]
fn emit_01040(signed_i16_01040: i16, signed_i32_01040: i32, signed_i64_01040: i64) {
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-01 instance-040: i16={}, i32={}, i64={}",
        signed_i16_01040,
        signed_i32_01040,
        signed_i64_01040
    );
}

#[inline(never)]
fn emit_01041(signed_i16_01041: i16, signed_i32_01041: i32, signed_i64_01041: i64) {
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-01 instance-041: i16={}, i32={}, i64={}",
        signed_i16_01041,
        signed_i32_01041,
        signed_i64_01041
    );
}

#[inline(never)]
fn emit_01042(signed_i16_01042: i16, signed_i32_01042: i32, signed_i64_01042: i64) {
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-01 instance-042: i16={}, i32={}, i64={}",
        signed_i16_01042,
        signed_i32_01042,
        signed_i64_01042
    );
}

#[inline(never)]
fn emit_01043(signed_i16_01043: i16, signed_i32_01043: i32, signed_i64_01043: i64) {
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-01 instance-043: i16={}, i32={}, i64={}",
        signed_i16_01043,
        signed_i32_01043,
        signed_i64_01043
    );
}

#[inline(never)]
fn emit_01044(signed_i16_01044: i16, signed_i32_01044: i32, signed_i64_01044: i64) {
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-01 instance-044: i16={}, i32={}, i64={}",
        signed_i16_01044,
        signed_i32_01044,
        signed_i64_01044
    );
}

#[inline(never)]
fn emit_01045(signed_i16_01045: i16, signed_i32_01045: i32, signed_i64_01045: i64) {
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-01 instance-045: i16={}, i32={}, i64={}",
        signed_i16_01045,
        signed_i32_01045,
        signed_i64_01045
    );
}

#[inline(never)]
fn emit_01046(signed_i16_01046: i16, signed_i32_01046: i32, signed_i64_01046: i64) {
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-01 instance-046: i16={}, i32={}, i64={}",
        signed_i16_01046,
        signed_i32_01046,
        signed_i64_01046
    );
}

#[inline(never)]
fn emit_01047(signed_i16_01047: i16, signed_i32_01047: i32, signed_i64_01047: i64) {
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-01 instance-047: i16={}, i32={}, i64={}",
        signed_i16_01047,
        signed_i32_01047,
        signed_i64_01047
    );
}

#[inline(never)]
fn emit_01048(signed_i16_01048: i16, signed_i32_01048: i32, signed_i64_01048: i64) {
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-01 instance-048: i16={}, i32={}, i64={}",
        signed_i16_01048,
        signed_i32_01048,
        signed_i64_01048
    );
}

#[inline(never)]
fn emit_01049(signed_i16_01049: i16, signed_i32_01049: i32, signed_i64_01049: i64) {
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-01 instance-049: i16={}, i32={}, i64={}",
        signed_i16_01049,
        signed_i32_01049,
        signed_i64_01049
    );
}

pub(crate) fn emit() {
    emit_01040(-1640, -32040, -64040);
    emit_01041(-1641, -32041, -64041);
    emit_01042(-1642, -32042, -64042);
    emit_01043(-1643, -32043, -64043);
    emit_01044(-1644, -32044, -64044);
    emit_01045(-1645, -32045, -64045);
    emit_01046(-1646, -32046, -64046);
    emit_01047(-1647, -32047, -64047);
    emit_01048(-1648, -32048, -64048);
    emit_01049(-1649, -32049, -64049);
}

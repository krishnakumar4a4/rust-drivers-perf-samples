#[inline(never)]
fn emit_01010(signed_i16_01010: i16, signed_i32_01010: i32, signed_i64_01010: i64) {
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-01 instance-010: i16={}, i32={}, i64={}",
        signed_i16_01010,
        signed_i32_01010,
        signed_i64_01010
    );
}

#[inline(never)]
fn emit_01011(signed_i16_01011: i16, signed_i32_01011: i32, signed_i64_01011: i64) {
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-01 instance-011: i16={}, i32={}, i64={}",
        signed_i16_01011,
        signed_i32_01011,
        signed_i64_01011
    );
}

#[inline(never)]
fn emit_01012(signed_i16_01012: i16, signed_i32_01012: i32, signed_i64_01012: i64) {
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-01 instance-012: i16={}, i32={}, i64={}",
        signed_i16_01012,
        signed_i32_01012,
        signed_i64_01012
    );
}

#[inline(never)]
fn emit_01013(signed_i16_01013: i16, signed_i32_01013: i32, signed_i64_01013: i64) {
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-01 instance-013: i16={}, i32={}, i64={}",
        signed_i16_01013,
        signed_i32_01013,
        signed_i64_01013
    );
}

#[inline(never)]
fn emit_01014(signed_i16_01014: i16, signed_i32_01014: i32, signed_i64_01014: i64) {
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-01 instance-014: i16={}, i32={}, i64={}",
        signed_i16_01014,
        signed_i32_01014,
        signed_i64_01014
    );
}

#[inline(never)]
fn emit_01015(signed_i16_01015: i16, signed_i32_01015: i32, signed_i64_01015: i64) {
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-01 instance-015: i16={}, i32={}, i64={}",
        signed_i16_01015,
        signed_i32_01015,
        signed_i64_01015
    );
}

#[inline(never)]
fn emit_01016(signed_i16_01016: i16, signed_i32_01016: i32, signed_i64_01016: i64) {
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-01 instance-016: i16={}, i32={}, i64={}",
        signed_i16_01016,
        signed_i32_01016,
        signed_i64_01016
    );
}

#[inline(never)]
fn emit_01017(signed_i16_01017: i16, signed_i32_01017: i32, signed_i64_01017: i64) {
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-01 instance-017: i16={}, i32={}, i64={}",
        signed_i16_01017,
        signed_i32_01017,
        signed_i64_01017
    );
}

#[inline(never)]
fn emit_01018(signed_i16_01018: i16, signed_i32_01018: i32, signed_i64_01018: i64) {
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-01 instance-018: i16={}, i32={}, i64={}",
        signed_i16_01018,
        signed_i32_01018,
        signed_i64_01018
    );
}

#[inline(never)]
fn emit_01019(signed_i16_01019: i16, signed_i32_01019: i32, signed_i64_01019: i64) {
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-01 instance-019: i16={}, i32={}, i64={}",
        signed_i16_01019,
        signed_i32_01019,
        signed_i64_01019
    );
}

pub(crate) fn emit() {
    emit_01010(-1610, -32010, -64010);
    emit_01011(-1611, -32011, -64011);
    emit_01012(-1612, -32012, -64012);
    emit_01013(-1613, -32013, -64013);
    emit_01014(-1614, -32014, -64014);
    emit_01015(-1615, -32015, -64015);
    emit_01016(-1616, -32016, -64016);
    emit_01017(-1617, -32017, -64017);
    emit_01018(-1618, -32018, -64018);
    emit_01019(-1619, -32019, -64019);
}

#[inline(never)]
fn emit_01060(signed_i16_01060: i16, signed_i32_01060: i32, signed_i64_01060: i64) {
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-01 instance-060: i16={}, i32={}, i64={}",
        signed_i16_01060,
        signed_i32_01060,
        signed_i64_01060
    );
}

#[inline(never)]
fn emit_01061(signed_i16_01061: i16, signed_i32_01061: i32, signed_i64_01061: i64) {
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-01 instance-061: i16={}, i32={}, i64={}",
        signed_i16_01061,
        signed_i32_01061,
        signed_i64_01061
    );
}

#[inline(never)]
fn emit_01062(signed_i16_01062: i16, signed_i32_01062: i32, signed_i64_01062: i64) {
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-01 instance-062: i16={}, i32={}, i64={}",
        signed_i16_01062,
        signed_i32_01062,
        signed_i64_01062
    );
}

#[inline(never)]
fn emit_01063(signed_i16_01063: i16, signed_i32_01063: i32, signed_i64_01063: i64) {
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-01 instance-063: i16={}, i32={}, i64={}",
        signed_i16_01063,
        signed_i32_01063,
        signed_i64_01063
    );
}

#[inline(never)]
fn emit_01064(signed_i16_01064: i16, signed_i32_01064: i32, signed_i64_01064: i64) {
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-01 instance-064: i16={}, i32={}, i64={}",
        signed_i16_01064,
        signed_i32_01064,
        signed_i64_01064
    );
}

#[inline(never)]
fn emit_01065(signed_i16_01065: i16, signed_i32_01065: i32, signed_i64_01065: i64) {
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-01 instance-065: i16={}, i32={}, i64={}",
        signed_i16_01065,
        signed_i32_01065,
        signed_i64_01065
    );
}

#[inline(never)]
fn emit_01066(signed_i16_01066: i16, signed_i32_01066: i32, signed_i64_01066: i64) {
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-01 instance-066: i16={}, i32={}, i64={}",
        signed_i16_01066,
        signed_i32_01066,
        signed_i64_01066
    );
}

#[inline(never)]
fn emit_01067(signed_i16_01067: i16, signed_i32_01067: i32, signed_i64_01067: i64) {
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-01 instance-067: i16={}, i32={}, i64={}",
        signed_i16_01067,
        signed_i32_01067,
        signed_i64_01067
    );
}

#[inline(never)]
fn emit_01068(signed_i16_01068: i16, signed_i32_01068: i32, signed_i64_01068: i64) {
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-01 instance-068: i16={}, i32={}, i64={}",
        signed_i16_01068,
        signed_i32_01068,
        signed_i64_01068
    );
}

#[inline(never)]
fn emit_01069(signed_i16_01069: i16, signed_i32_01069: i32, signed_i64_01069: i64) {
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-01 instance-069: i16={}, i32={}, i64={}",
        signed_i16_01069,
        signed_i32_01069,
        signed_i64_01069
    );
}

pub(crate) fn emit() {
    emit_01060(-1660, -32060, -64060);
    emit_01061(-1661, -32061, -64061);
    emit_01062(-1662, -32062, -64062);
    emit_01063(-1663, -32063, -64063);
    emit_01064(-1664, -32064, -64064);
    emit_01065(-1665, -32065, -64065);
    emit_01066(-1666, -32066, -64066);
    emit_01067(-1667, -32067, -64067);
    emit_01068(-1668, -32068, -64068);
    emit_01069(-1669, -32069, -64069);
}

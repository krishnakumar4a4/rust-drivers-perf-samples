#[inline(never)]
fn emit_02000() {
    let unsigned_u8_02000 = 0_u8;
    let unsigned_u32_02000 = 4000000000_u32;
    let unsigned_u64_02000 = 8000000000_u64;
    trace!(
        WARNING,
        UNSIGNED,
        "fixture-1000 scenario-02 instance-000: u8={}, u32={}, u64={}",
        unsigned_u8_02000,
        unsigned_u32_02000,
        unsigned_u64_02000
    );
}

#[inline(never)]
fn emit_02001() {
    let unsigned_u8_02001 = 1_u8;
    let unsigned_u32_02001 = 4000000001_u32;
    let unsigned_u64_02001 = 8000000001_u64;
    trace!(
        ERROR,
        UNSIGNED,
        "fixture-1000 scenario-02 instance-001: u8={}, u32={}, u64={}",
        unsigned_u8_02001,
        unsigned_u32_02001,
        unsigned_u64_02001
    );
}

#[inline(never)]
fn emit_02002() {
    let unsigned_u8_02002 = 2_u8;
    let unsigned_u32_02002 = 4000000002_u32;
    let unsigned_u64_02002 = 8000000002_u64;
    trace!(
        CRITICAL,
        UNSIGNED,
        "fixture-1000 scenario-02 instance-002: u8={}, u32={}, u64={}",
        unsigned_u8_02002,
        unsigned_u32_02002,
        unsigned_u64_02002
    );
}

#[inline(never)]
fn emit_02003() {
    let unsigned_u8_02003 = 3_u8;
    let unsigned_u32_02003 = 4000000003_u32;
    let unsigned_u64_02003 = 8000000003_u64;
    trace!(
        INFO,
        UNSIGNED,
        "fixture-1000 scenario-02 instance-003: u8={}, u32={}, u64={}",
        unsigned_u8_02003,
        unsigned_u32_02003,
        unsigned_u64_02003
    );
}

#[inline(never)]
fn emit_02004() {
    let unsigned_u8_02004 = 4_u8;
    let unsigned_u32_02004 = 4000000004_u32;
    let unsigned_u64_02004 = 8000000004_u64;
    trace!(
        VERBOSE,
        UNSIGNED,
        "fixture-1000 scenario-02 instance-004: u8={}, u32={}, u64={}",
        unsigned_u8_02004,
        unsigned_u32_02004,
        unsigned_u64_02004
    );
}

#[inline(never)]
fn emit_02005() {
    let unsigned_u8_02005 = 5_u8;
    let unsigned_u32_02005 = 4000000005_u32;
    let unsigned_u64_02005 = 8000000005_u64;
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-02 instance-005: u8={}, u32={}, u64={}",
        unsigned_u8_02005,
        unsigned_u32_02005,
        unsigned_u64_02005
    );
}

#[inline(never)]
fn emit_02006() {
    let unsigned_u8_02006 = 6_u8;
    let unsigned_u32_02006 = 4000000006_u32;
    let unsigned_u64_02006 = 8000000006_u64;
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-02 instance-006: u8={}, u32={}, u64={}",
        unsigned_u8_02006,
        unsigned_u32_02006,
        unsigned_u64_02006
    );
}

#[inline(never)]
fn emit_02007() {
    let unsigned_u8_02007 = 7_u8;
    let unsigned_u32_02007 = 4000000007_u32;
    let unsigned_u64_02007 = 8000000007_u64;
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-02 instance-007: u8={}, u32={}, u64={}",
        unsigned_u8_02007,
        unsigned_u32_02007,
        unsigned_u64_02007
    );
}

#[inline(never)]
fn emit_02008() {
    let unsigned_u8_02008 = 8_u8;
    let unsigned_u32_02008 = 4000000008_u32;
    let unsigned_u64_02008 = 8000000008_u64;
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-02 instance-008: u8={}, u32={}, u64={}",
        unsigned_u8_02008,
        unsigned_u32_02008,
        unsigned_u64_02008
    );
}

#[inline(never)]
fn emit_02009() {
    let unsigned_u8_02009 = 9_u8;
    let unsigned_u32_02009 = 4000000009_u32;
    let unsigned_u64_02009 = 8000000009_u64;
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-02 instance-009: u8={}, u32={}, u64={}",
        unsigned_u8_02009,
        unsigned_u32_02009,
        unsigned_u64_02009
    );
}

pub(crate) fn emit() {
    emit_02000();
    emit_02001();
    emit_02002();
    emit_02003();
    emit_02004();
    emit_02005();
    emit_02006();
    emit_02007();
    emit_02008();
    emit_02009();
}

use crate::dummy_trace::*;

#[inline(never)]
fn emit_02010() {
    let unsigned_u8_02010 = 10_u8;
    let unsigned_u32_02010 = 4000000010_u32;
    let unsigned_u64_02010 = 8000000010_u64;
    dummy_trace(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-02 instance-010: u8={}, u32={}, u64={}",
        unsigned_u8_02010,
        unsigned_u32_02010,
        unsigned_u64_02010,
    );
}

#[inline(never)]
fn emit_02011() {
    let unsigned_u8_02011 = 11_u8;
    let unsigned_u32_02011 = 4000000011_u32;
    let unsigned_u64_02011 = 8000000011_u64;
    dummy_trace(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-02 instance-011: u8={}, u32={}, u64={}",
        unsigned_u8_02011,
        unsigned_u32_02011,
        unsigned_u64_02011,
    );
}

#[inline(never)]
fn emit_02012() {
    let unsigned_u8_02012 = 12_u8;
    let unsigned_u32_02012 = 4000000012_u32;
    let unsigned_u64_02012 = 8000000012_u64;
    dummy_trace(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-02 instance-012: u8={}, u32={}, u64={}",
        unsigned_u8_02012,
        unsigned_u32_02012,
        unsigned_u64_02012,
    );
}

#[inline(never)]
fn emit_02013() {
    let unsigned_u8_02013 = 13_u8;
    let unsigned_u32_02013 = 4000000013_u32;
    let unsigned_u64_02013 = 8000000013_u64;
    dummy_trace(
        INFO,
        STRINGS,
        "fixture-1000 scenario-02 instance-013: u8={}, u32={}, u64={}",
        unsigned_u8_02013,
        unsigned_u32_02013,
        unsigned_u64_02013,
    );
}

#[inline(never)]
fn emit_02014() {
    let unsigned_u8_02014 = 14_u8;
    let unsigned_u32_02014 = 4000000014_u32;
    let unsigned_u64_02014 = 8000000014_u64;
    dummy_trace(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-02 instance-014: u8={}, u32={}, u64={}",
        unsigned_u8_02014,
        unsigned_u32_02014,
        unsigned_u64_02014,
    );
}

#[inline(never)]
fn emit_02015() {
    let unsigned_u8_02015 = 15_u8;
    let unsigned_u32_02015 = 4000000015_u32;
    let unsigned_u64_02015 = 8000000015_u64;
    dummy_trace(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-02 instance-015: u8={}, u32={}, u64={}",
        unsigned_u8_02015,
        unsigned_u32_02015,
        unsigned_u64_02015,
    );
}

#[inline(never)]
fn emit_02016() {
    let unsigned_u8_02016 = 16_u8;
    let unsigned_u32_02016 = 4000000016_u32;
    let unsigned_u64_02016 = 8000000016_u64;
    dummy_trace(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-02 instance-016: u8={}, u32={}, u64={}",
        unsigned_u8_02016,
        unsigned_u32_02016,
        unsigned_u64_02016,
    );
}

#[inline(never)]
fn emit_02017() {
    let unsigned_u8_02017 = 17_u8;
    let unsigned_u32_02017 = 4000000017_u32;
    let unsigned_u64_02017 = 8000000017_u64;
    dummy_trace(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-02 instance-017: u8={}, u32={}, u64={}",
        unsigned_u8_02017,
        unsigned_u32_02017,
        unsigned_u64_02017,
    );
}

#[inline(never)]
fn emit_02018() {
    let unsigned_u8_02018 = 18_u8;
    let unsigned_u32_02018 = 4000000018_u32;
    let unsigned_u64_02018 = 8000000018_u64;
    dummy_trace(
        INFO,
        LITERALS,
        "fixture-1000 scenario-02 instance-018: u8={}, u32={}, u64={}",
        unsigned_u8_02018,
        unsigned_u32_02018,
        unsigned_u64_02018,
    );
}

#[inline(never)]
fn emit_02019() {
    let unsigned_u8_02019 = 19_u8;
    let unsigned_u32_02019 = 4000000019_u32;
    let unsigned_u64_02019 = 8000000019_u64;
    dummy_trace(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-02 instance-019: u8={}, u32={}, u64={}",
        unsigned_u8_02019,
        unsigned_u32_02019,
        unsigned_u64_02019,
    );
}

pub(crate) fn emit() {
    emit_02010();
    emit_02011();
    emit_02012();
    emit_02013();
    emit_02014();
    emit_02015();
    emit_02016();
    emit_02017();
    emit_02018();
    emit_02019();
}

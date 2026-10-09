#[inline(never)]
fn emit_02060() {
    let unsigned_u8_02060 = 60_u8;
    let unsigned_u32_02060 = 4000000060_u32;
    let unsigned_u64_02060 = 8000000060_u64;
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-02 instance-060: u8={}, u32={}, u64={}",
        unsigned_u8_02060,
        unsigned_u32_02060,
        unsigned_u64_02060
    );
}

#[inline(never)]
fn emit_02061() {
    let unsigned_u8_02061 = 61_u8;
    let unsigned_u32_02061 = 4000000061_u32;
    let unsigned_u64_02061 = 8000000061_u64;
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-02 instance-061: u8={}, u32={}, u64={}",
        unsigned_u8_02061,
        unsigned_u32_02061,
        unsigned_u64_02061
    );
}

#[inline(never)]
fn emit_02062() {
    let unsigned_u8_02062 = 62_u8;
    let unsigned_u32_02062 = 4000000062_u32;
    let unsigned_u64_02062 = 8000000062_u64;
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-02 instance-062: u8={}, u32={}, u64={}",
        unsigned_u8_02062,
        unsigned_u32_02062,
        unsigned_u64_02062
    );
}

#[inline(never)]
fn emit_02063() {
    let unsigned_u8_02063 = 63_u8;
    let unsigned_u32_02063 = 4000000063_u32;
    let unsigned_u64_02063 = 8000000063_u64;
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-02 instance-063: u8={}, u32={}, u64={}",
        unsigned_u8_02063,
        unsigned_u32_02063,
        unsigned_u64_02063
    );
}

#[inline(never)]
fn emit_02064() {
    let unsigned_u8_02064 = 64_u8;
    let unsigned_u32_02064 = 4000000064_u32;
    let unsigned_u64_02064 = 8000000064_u64;
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-02 instance-064: u8={}, u32={}, u64={}",
        unsigned_u8_02064,
        unsigned_u32_02064,
        unsigned_u64_02064
    );
}

#[inline(never)]
fn emit_02065() {
    let unsigned_u8_02065 = 65_u8;
    let unsigned_u32_02065 = 4000000065_u32;
    let unsigned_u64_02065 = 8000000065_u64;
    trace!(
        WARNING,
        LITERALS,
        "fixture-1000 scenario-02 instance-065: u8={}, u32={}, u64={}",
        unsigned_u8_02065,
        unsigned_u32_02065,
        unsigned_u64_02065
    );
}

#[inline(never)]
fn emit_02066() {
    let unsigned_u8_02066 = 66_u8;
    let unsigned_u32_02066 = 4000000066_u32;
    let unsigned_u64_02066 = 8000000066_u64;
    trace!(
        ERROR,
        LITERALS,
        "fixture-1000 scenario-02 instance-066: u8={}, u32={}, u64={}",
        unsigned_u8_02066,
        unsigned_u32_02066,
        unsigned_u64_02066
    );
}

#[inline(never)]
fn emit_02067() {
    let unsigned_u8_02067 = 67_u8;
    let unsigned_u32_02067 = 4000000067_u32;
    let unsigned_u64_02067 = 8000000067_u64;
    trace!(
        CRITICAL,
        LITERALS,
        "fixture-1000 scenario-02 instance-067: u8={}, u32={}, u64={}",
        unsigned_u8_02067,
        unsigned_u32_02067,
        unsigned_u64_02067
    );
}

#[inline(never)]
fn emit_02068() {
    let unsigned_u8_02068 = 68_u8;
    let unsigned_u32_02068 = 4000000068_u32;
    let unsigned_u64_02068 = 8000000068_u64;
    trace!(
        INFO,
        LITERALS,
        "fixture-1000 scenario-02 instance-068: u8={}, u32={}, u64={}",
        unsigned_u8_02068,
        unsigned_u32_02068,
        unsigned_u64_02068
    );
}

#[inline(never)]
fn emit_02069() {
    let unsigned_u8_02069 = 69_u8;
    let unsigned_u32_02069 = 4000000069_u32;
    let unsigned_u64_02069 = 8000000069_u64;
    trace!(
        VERBOSE,
        LITERALS,
        "fixture-1000 scenario-02 instance-069: u8={}, u32={}, u64={}",
        unsigned_u8_02069,
        unsigned_u32_02069,
        unsigned_u64_02069
    );
}

pub(crate) fn emit() {
    emit_02060();
    emit_02061();
    emit_02062();
    emit_02063();
    emit_02064();
    emit_02065();
    emit_02066();
    emit_02067();
    emit_02068();
    emit_02069();
}

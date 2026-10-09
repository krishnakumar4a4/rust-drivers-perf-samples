#[inline(never)]
fn emit_02080() {
    let unsigned_u8_02080 = 80_u8;
    let unsigned_u32_02080 = 4000000080_u32;
    let unsigned_u64_02080 = 8000000080_u64;
    trace!(
        WARNING,
        FLOATS,
        "fixture-1000 scenario-02 instance-080: u8={}, u32={}, u64={}",
        unsigned_u8_02080,
        unsigned_u32_02080,
        unsigned_u64_02080
    );
}

#[inline(never)]
fn emit_02081() {
    let unsigned_u8_02081 = 81_u8;
    let unsigned_u32_02081 = 4000000081_u32;
    let unsigned_u64_02081 = 8000000081_u64;
    trace!(
        ERROR,
        FLOATS,
        "fixture-1000 scenario-02 instance-081: u8={}, u32={}, u64={}",
        unsigned_u8_02081,
        unsigned_u32_02081,
        unsigned_u64_02081
    );
}

#[inline(never)]
fn emit_02082() {
    let unsigned_u8_02082 = 82_u8;
    let unsigned_u32_02082 = 4000000082_u32;
    let unsigned_u64_02082 = 8000000082_u64;
    trace!(
        CRITICAL,
        FLOATS,
        "fixture-1000 scenario-02 instance-082: u8={}, u32={}, u64={}",
        unsigned_u8_02082,
        unsigned_u32_02082,
        unsigned_u64_02082
    );
}

#[inline(never)]
fn emit_02083() {
    let unsigned_u8_02083 = 83_u8;
    let unsigned_u32_02083 = 4000000083_u32;
    let unsigned_u64_02083 = 8000000083_u64;
    trace!(
        INFO,
        FLOATS,
        "fixture-1000 scenario-02 instance-083: u8={}, u32={}, u64={}",
        unsigned_u8_02083,
        unsigned_u32_02083,
        unsigned_u64_02083
    );
}

#[inline(never)]
fn emit_02084() {
    let unsigned_u8_02084 = 84_u8;
    let unsigned_u32_02084 = 4000000084_u32;
    let unsigned_u64_02084 = 8000000084_u64;
    trace!(
        VERBOSE,
        FLOATS,
        "fixture-1000 scenario-02 instance-084: u8={}, u32={}, u64={}",
        unsigned_u8_02084,
        unsigned_u32_02084,
        unsigned_u64_02084
    );
}

#[inline(never)]
fn emit_02085() {
    let unsigned_u8_02085 = 85_u8;
    let unsigned_u32_02085 = 4000000085_u32;
    let unsigned_u64_02085 = 8000000085_u64;
    trace!(
        WARNING,
        STRINGS,
        "fixture-1000 scenario-02 instance-085: u8={}, u32={}, u64={}",
        unsigned_u8_02085,
        unsigned_u32_02085,
        unsigned_u64_02085
    );
}

#[inline(never)]
fn emit_02086() {
    let unsigned_u8_02086 = 86_u8;
    let unsigned_u32_02086 = 4000000086_u32;
    let unsigned_u64_02086 = 8000000086_u64;
    trace!(
        ERROR,
        STRINGS,
        "fixture-1000 scenario-02 instance-086: u8={}, u32={}, u64={}",
        unsigned_u8_02086,
        unsigned_u32_02086,
        unsigned_u64_02086
    );
}

#[inline(never)]
fn emit_02087() {
    let unsigned_u8_02087 = 87_u8;
    let unsigned_u32_02087 = 4000000087_u32;
    let unsigned_u64_02087 = 8000000087_u64;
    trace!(
        CRITICAL,
        STRINGS,
        "fixture-1000 scenario-02 instance-087: u8={}, u32={}, u64={}",
        unsigned_u8_02087,
        unsigned_u32_02087,
        unsigned_u64_02087
    );
}

#[inline(never)]
fn emit_02088() {
    let unsigned_u8_02088 = 88_u8;
    let unsigned_u32_02088 = 4000000088_u32;
    let unsigned_u64_02088 = 8000000088_u64;
    trace!(
        INFO,
        STRINGS,
        "fixture-1000 scenario-02 instance-088: u8={}, u32={}, u64={}",
        unsigned_u8_02088,
        unsigned_u32_02088,
        unsigned_u64_02088
    );
}

#[inline(never)]
fn emit_02089() {
    let unsigned_u8_02089 = 89_u8;
    let unsigned_u32_02089 = 4000000089_u32;
    let unsigned_u64_02089 = 8000000089_u64;
    trace!(
        VERBOSE,
        STRINGS,
        "fixture-1000 scenario-02 instance-089: u8={}, u32={}, u64={}",
        unsigned_u8_02089,
        unsigned_u32_02089,
        unsigned_u64_02089
    );
}

pub(crate) fn emit() {
    emit_02080();
    emit_02081();
    emit_02082();
    emit_02083();
    emit_02084();
    emit_02085();
    emit_02086();
    emit_02087();
    emit_02088();
    emit_02089();
}

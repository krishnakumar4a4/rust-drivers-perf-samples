use crate::dummy_trace::*;

use crate::{ImplicitState, MixedState};

fn returned_i32(value: i32) -> i32 {
    value
}

#[inline(never)]
fn emit_08080() {
    let implicit_state_08080 = ImplicitState::Ready;
    let mixed_state_08080 = MixedState::Failed;
    let returned_i32_08080 = returned_i32(-8080);
    dummy_trace(
        ERROR,
        MIXED,
        "fixture-1000 scenario-08 instance-080: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08080,
        mixed_state_08080,
        returned_i32_08080,
    );
}

#[inline(never)]
fn emit_08081() {
    let implicit_state_08081 = ImplicitState::Unknown;
    let mixed_state_08081 = MixedState::Unknown;
    let returned_i32_08081 = returned_i32(-8081);
    dummy_trace(
        CRITICAL,
        MIXED,
        "fixture-1000 scenario-08 instance-081: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08081,
        mixed_state_08081,
        returned_i32_08081,
    );
}

#[inline(never)]
fn emit_08082() {
    let implicit_state_08082 = ImplicitState::Starting;
    let mixed_state_08082 = MixedState::Warning;
    let returned_i32_08082 = returned_i32(-8082);
    dummy_trace(
        INFO,
        MIXED,
        "fixture-1000 scenario-08 instance-082: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08082,
        mixed_state_08082,
        returned_i32_08082,
    );
}

#[inline(never)]
fn emit_08083() {
    let implicit_state_08083 = ImplicitState::Ready;
    let mixed_state_08083 = MixedState::Failed;
    let returned_i32_08083 = returned_i32(-8083);
    dummy_trace(
        VERBOSE,
        MIXED,
        "fixture-1000 scenario-08 instance-083: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08083,
        mixed_state_08083,
        returned_i32_08083,
    );
}

#[inline(never)]
fn emit_08084() {
    let implicit_state_08084 = ImplicitState::Unknown;
    let mixed_state_08084 = MixedState::Unknown;
    let returned_i32_08084 = returned_i32(-8084);
    dummy_trace(
        WARNING,
        MIXED,
        "fixture-1000 scenario-08 instance-084: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08084,
        mixed_state_08084,
        returned_i32_08084,
    );
}

#[inline(never)]
fn emit_08085() {
    let implicit_state_08085 = ImplicitState::Starting;
    let mixed_state_08085 = MixedState::Warning;
    let returned_i32_08085 = returned_i32(-8085);
    dummy_trace(
        ERROR,
        STATUS,
        "fixture-1000 scenario-08 instance-085: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08085,
        mixed_state_08085,
        returned_i32_08085,
    );
}

#[inline(never)]
fn emit_08086() {
    let implicit_state_08086 = ImplicitState::Ready;
    let mixed_state_08086 = MixedState::Failed;
    let returned_i32_08086 = returned_i32(-8086);
    dummy_trace(
        CRITICAL,
        STATUS,
        "fixture-1000 scenario-08 instance-086: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08086,
        mixed_state_08086,
        returned_i32_08086,
    );
}

#[inline(never)]
fn emit_08087() {
    let implicit_state_08087 = ImplicitState::Unknown;
    let mixed_state_08087 = MixedState::Unknown;
    let returned_i32_08087 = returned_i32(-8087);
    dummy_trace(
        INFO,
        STATUS,
        "fixture-1000 scenario-08 instance-087: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08087,
        mixed_state_08087,
        returned_i32_08087,
    );
}

#[inline(never)]
fn emit_08088() {
    let implicit_state_08088 = ImplicitState::Starting;
    let mixed_state_08088 = MixedState::Warning;
    let returned_i32_08088 = returned_i32(-8088);
    dummy_trace(
        VERBOSE,
        STATUS,
        "fixture-1000 scenario-08 instance-088: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08088,
        mixed_state_08088,
        returned_i32_08088,
    );
}

#[inline(never)]
fn emit_08089() {
    let implicit_state_08089 = ImplicitState::Ready;
    let mixed_state_08089 = MixedState::Failed;
    let returned_i32_08089 = returned_i32(-8089);
    dummy_trace(
        WARNING,
        STATUS,
        "fixture-1000 scenario-08 instance-089: implicit={}, mixed={}, returned-i32={}",
        implicit_state_08089,
        mixed_state_08089,
        returned_i32_08089,
    );
}

pub(crate) fn emit() {
    emit_08080();
    emit_08081();
    emit_08082();
    emit_08083();
    emit_08084();
    emit_08085();
    emit_08086();
    emit_08087();
    emit_08088();
    emit_08089();
}

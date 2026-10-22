#![no_std]
#![feature(codeview_annotation)]

use wdf::{
    Device, DeviceInit, Driver, DriverConfig, DriverObject, IRQL, NtResult, NtStatus,
    UnicodeString, driver_entry, status_codes, wpp_control_guids,
};

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u8)]
pub(crate) enum ByteState {
    Unknown = 0,
    Ready = 2,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u16)]
pub(crate) enum ShortState {
    Unknown = 0,
    Warning = 10,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u32)]
pub(crate) enum LongState {
    Offline = 0,
    Online = 1,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u8)]
pub(crate) enum ImplicitState {
    Unknown,
    Ready,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u16)]
pub(crate) enum MixedState {
    Unknown,
    Warning = 10,
}

wpp_control_guids!(
    CoreProvider 4fdb2201-5a10-4c01-9001-000000000020 {
        LITERALS,
        SIGNED,
        UNSIGNED,
        FLOATS,
        STRINGS,
    }
    DetailProvider 4fdb2202-5a10-4c01-9002-000000000020 {
        STATUS,
        ENUM_PRIMARY,
        GENERIC,
        ENUM_SECONDARY,
        MIXED,
    }
);

#[driver_entry(trace_providers(CoreProvider, DetailProvider))]
fn driver_entry(driver_object: &mut DriverObject, registry_path: &UnicodeString) -> NtResult<()> {
    trace!(
        INFO,
        LITERALS, "direct-entry-00: signed={}, unsigned={}, bool={}", -1_i8, 60000_u16, true
    );
    trace!(
        VERBOSE,
        SIGNED, "direct-entry-01: i16={}, i32={}, i64={}", -1600_i16, -32000_i32, -64000_i64
    );
    trace!(
        WARNING,
        UNSIGNED, "direct-entry-02: u8={}, u32={}, u64={}", 0_u8, 4000000000_u32, 8000000000_u64
    );
    trace!(
        ERROR,
        FLOATS, "direct-entry-03: f32={}, f64={}, bool={}", 3.25_f32, 30.125_f64, false
    );
    trace!(
        CRITICAL,
        STRINGS,
        "direct-entry-04: str={}, cstr={}, text={}",
        "entry-text",
        c"entry-c-string",
        "registry-received"
    );
    trace!(
        INFO,
        STATUS,
        "direct-entry-05: expected-irql={}, status={}, usize={}",
        IRQL::Passive,
        NtStatus::from(status_codes::STATUS_SUCCESS),
        5000_usize
    );
    trace!(
        VERBOSE,
        ENUM_PRIMARY,
        "direct-entry-06: byte={}, short={}, long={}",
        ByteState::Unknown,
        ShortState::Unknown,
        LongState::Offline
    );
    trace!(
        WARNING,
        GENERIC, "direct-entry-07: isize={}, str={}, u32={}", -7_isize, "typed-entry", 70_u32
    );
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "direct-entry-08: implicit={}, mixed={}, i32={}",
        ImplicitState::Unknown,
        MixedState::Unknown,
        -8000_i32
    );
    trace!(
        CRITICAL,
        MIXED, "direct-entry-09: u8={}, i16={}, usize={}", 90_u8, -900_i16, 9000_usize
    );

    let config = DriverConfig::new(event_device_add);
    let _driver = Driver::create(driver_object, registry_path, config)?;
    Ok(())
}

fn event_device_add(device_init: &mut DeviceInit) -> NtResult<()> {
    trace!(
        INFO,
        LITERALS, "direct-device-add-00: signed={}, unsigned={}, bool={}", -2_i8, 60001_u16, false
    );
    trace!(
        VERBOSE,
        SIGNED, "direct-device-add-01: i16={}, i32={}, i64={}", -1601_i16, -32001_i32, -64001_i64
    );
    trace!(
        WARNING,
        UNSIGNED,
        "direct-device-add-02: u8={}, u32={}, u64={}",
        1_u8,
        4000000001_u32,
        8000000001_u64
    );
    trace!(
        ERROR,
        FLOATS, "direct-device-add-03: f32={}, f64={}, bool={}", 4.25_f32, 31.125_f64, true
    );
    trace!(
        CRITICAL,
        STRINGS,
        "direct-device-add-04: str={}, cstr={}, text={}",
        "device-text",
        c"device-c-string",
        "device-init-received"
    );
    trace!(
        INFO,
        STATUS,
        "direct-device-add-05: expected-irql={}, status={}, usize={}",
        IRQL::Passive,
        NtStatus::from(status_codes::STATUS_SUCCESS),
        5001_usize
    );
    trace!(
        VERBOSE,
        ENUM_PRIMARY,
        "direct-device-add-06: byte={}, short={}, long={}",
        ByteState::Ready,
        ShortState::Warning,
        LongState::Online
    );
    trace!(
        WARNING,
        GENERIC, "direct-device-add-07: isize={}, str={}, u32={}", -8_isize, "typed-device", 71_u32
    );
    trace!(
        ERROR,
        ENUM_SECONDARY,
        "direct-device-add-08: implicit={}, mixed={}, i32={}",
        ImplicitState::Ready,
        MixedState::Warning,
        -8001_i32
    );
    trace!(
        CRITICAL,
        MIXED, "direct-device-add-09: u8={}, i16={}, usize={}", 91_u8, -901_i16, 9001_usize
    );

    let _device = Device::create(device_init, None)?;
    Ok(())
}

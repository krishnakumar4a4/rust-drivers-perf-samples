#![no_std]
#![feature(codeview_annotation)]

use wdf::{
    driver_entry, wpp_control_guids, Device, DeviceInit, Driver, DriverConfig, DriverObject,
    NtResult, UnicodeString,
};

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u8)]
pub(crate) enum ByteState {
    Unknown = 0,
    Starting = 1,
    Ready = 2,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u16)]
pub(crate) enum ShortState {
    Unknown = 0,
    Warning = 10,
    Failed = 100,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u32)]
pub(crate) enum LongState {
    Offline = 0,
    Online = 1,
    Recovering = 2,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u8)]
pub(crate) enum ImplicitState {
    Unknown,
    Starting,
    Ready,
}

#[derive(Clone, Copy, wpp::WppField)]
#[repr(u16)]
pub(crate) enum MixedState {
    Unknown,
    Warning = 10,
    Failed,
}

wpp_control_guids!(
    CoreProvider 4fdb1001-5a10-4c01-9001-000000000010 {
        LITERALS,
        SIGNED,
        UNSIGNED,
        FLOATS,
        STRINGS,
    }
    DetailProvider 4fdb1002-5a10-4c01-9002-000000000010 {
        STATUS,
        ENUM_PRIMARY,
        GENERIC,
        ENUM_SECONDARY,
        MIXED,
    }
);

mod scenario_00_literals;
mod scenario_01_signed;
mod scenario_02_unsigned;
mod scenario_03_floats;
mod scenario_04_strings;
mod scenario_05_status;
mod scenario_06_enum_primary;
mod scenario_07_generic;
mod scenario_08_enum_secondary;
mod scenario_09_mixed;

#[driver_entry(trace_providers(CoreProvider, DetailProvider))]
fn driver_entry(driver_object: &mut DriverObject, registry_path: &UnicodeString) -> NtResult<()> {
    scenario_00_literals::emit();
    scenario_01_signed::emit();

    let config = DriverConfig::new(evt_device_add);
    let _driver = Driver::create(driver_object, registry_path, config)?;
    Ok(())
}

fn evt_device_add(device_init: &mut DeviceInit) -> NtResult<()> {
    scenario_02_unsigned::emit();
    scenario_03_floats::emit();
    scenario_04_strings::emit();
    scenario_05_status::emit();
    scenario_06_enum_primary::emit();
    scenario_07_generic::emit();
    scenario_08_enum_secondary::emit();
    scenario_09_mixed::emit();

    let _device = Device::create(device_init, None)?;
    Ok(())
}

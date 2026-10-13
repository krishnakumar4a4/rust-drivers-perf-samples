#![no_std]

use wdf::{
    driver_entry, Device, DeviceInit, Driver, DriverConfig, DriverObject, NtResult, UnicodeString,
};

// The 1x fixture constructs only the first variant of each enum.
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum ByteState {
    Unknown = 0,
    Starting = 1,
    Ready = 2,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u16)]
pub(crate) enum ShortState {
    Unknown = 0,
    Warning = 10,
    Failed = 100,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub(crate) enum LongState {
    Offline = 0,
    Online = 1,
    Recovering = 2,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum ImplicitState {
    Unknown,
    Starting,
    Ready,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u16)]
pub(crate) enum MixedState {
    Unknown,
    Warning = 10,
    Failed,
}

mod dummy_trace;

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

#[driver_entry]
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

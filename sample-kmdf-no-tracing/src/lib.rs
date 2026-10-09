#![no_std]

extern crate alloc;

use wdf::{
    driver_entry, println, wpp_control_guids, DeviceInit, Driver, DriverConfig, DriverObject,
    status_codes, IRQL, NtResult, NtStatus, UnicodeString,
};

mod device;

#[driver_entry]
fn driver_entry(driver_object: &mut DriverObject, registry_path: &UnicodeString) -> NtResult<()> {
    let config = DriverConfig::new(evt_device_add);
    let driver = Driver::create(driver_object, registry_path, config)?;

    Ok(())
}

fn evt_device_add(device_init: &mut DeviceInit) -> NtResult<()> {
    device::device_create(device_init)
}
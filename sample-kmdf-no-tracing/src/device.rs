//! Device creation and PnP/power callbacks.

use core::time::Duration;

use wdf::{
    object_context, println, Arc, Device, DeviceInit, Guid, IoQueue, NtResult,
    PnpPowerEventCallbacks, Timer,
};


pub(crate) fn device_create(device_init: &mut DeviceInit) -> NtResult<()> {
    let mut pnp_power_callbacks = PnpPowerEventCallbacks::default();
    pnp_power_callbacks.evt_device_self_managed_io_init = Some(evt_device_self_managed_io_start);
    pnp_power_callbacks.evt_device_self_managed_io_suspend =
        Some(evt_device_self_managed_io_suspend);
    pnp_power_callbacks.evt_device_self_managed_io_restart = Some(evt_device_self_managed_io_start);

    let device = Device::create(device_init, Some(pnp_power_callbacks))?;

    let _ = device.create_device_interface(
        &Guid::parse("2aa02ab1-c26e-431b-8efe-85ee8de102e4").expect("GUID is valid"),
        None,
    )?;
    Ok(())
}

fn evt_device_self_managed_io_start(device: &Device) -> NtResult<()> {
    println!("Self-managed I/O start called: {:?}", device);

    Ok(())
}

fn evt_device_self_managed_io_suspend(device: &Device) -> NtResult<()> {
    println!("Self-managed I/O suspend called: {:?}", device);

    Ok(())
}

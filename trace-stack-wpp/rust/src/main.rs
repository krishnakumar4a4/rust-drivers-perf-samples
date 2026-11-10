#![feature(codeview_annotation)]
#![feature(core_os)]

wpp::wpp_control_guids!(
    TraceProvider "4fdb3301-5a10-4c01-9001-000000000001" {
        SAMPLE,
    }
);

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn trace_sample(value: u64) {
    trace!(INFO, SAMPLE, "value={}", value);
}

/// Retains the native tracing lifecycle for assembly analysis, not execution.
///
/// # Safety
/// Requires a kernel-mode caller with valid driver and registry-path pointers.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn trace_entry(
    driver_object: *mut core::ffi::c_void,
    registry_path: *const core::ffi::c_void,
) {
    // SAFETY: The caller must supply the kernel pointers required by WPP.
    unsafe {
        __wpp_ifr::init(driver_object, registry_path);
        TraceProvider::init();
    }
    trace_sample(42);
    TraceProvider::clean_up();
    __wpp_ifr::cleanup();
}

fn main() {
    trace_sample(42);
}

// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

use std::{
    ffi::c_void,
    sync::{Mutex, MutexGuard},
};

wpp::wpp_control_guids!(
    BenchProvider 01234567-89ab-cdef-0123-456789abcdef {
        BENCHMARK,
        FILTERED,
    }
);

#[repr(C)]
#[derive(Debug, Default)]
struct MockStats {
    registrations: u64,
    unregistrations: u64,
    set_information: u64,
    starts: u64,
    stops: u64,
    etw_writes: u64,
    ifr_writes: u64,
}

unsafe extern "C" {
    fn wpp_bench_reset(enable_ifr: u32, capture_payloads: u32);
    fn wpp_bench_enable(level: u8, keywords: u64);
    fn wpp_bench_stats(out: *mut MockStats);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn wpp_bench_set_ifr_context(
    blocks: *mut wpp::ifr::WPP_PROJECT_CONTROL_BLOCK,
    context: *mut c_void,
) {
    let mut block = blocks.cast::<wpp::ifr::WPP_TRACE_CONTROL_BLOCK>();
    while !block.is_null() {
        // SAFETY: Only the start/stop mocks call this, with the generated,
        // initialized control-block list and while the fixture owns SESSION.
        let control = unsafe { &mut *block };
        control.AutoLogContext = context;
        block = control.Next.cast_mut();
    }
}

static SESSION: Mutex<()> = Mutex::new(());
static DRIVER: u8 = 0;
static REGISTRY_PATH: u8 = 0;

struct Session {
    _guard: MutexGuard<'static, ()>,
}

impl Session {
    fn new(etw: bool, ifr: bool, capture: bool) -> Self {
        let guard = SESSION.lock().expect("a previous mock session failed");
        // SAFETY: The session lock serializes all mock and generated lifecycle
        // state. The native mocks treat driver/registry pointers as opaque,
        // never dereference them, and the stand-ins outlive cleanup.
        unsafe {
            wpp_bench_reset(u32::from(ifr), u32::from(capture));
            __wpp_ifr::init(
                (&raw const DRIVER).cast_mut().cast(),
                (&raw const REGISTRY_PATH).cast(),
            );
            BenchProvider::init();
        }
        let session = Self { _guard: guard };
        if !etw {
            session.enable(0, 0);
        }
        assert_ne!(BenchProvider::STATE.reg_handle(), 0);
        assert_eq!(
            BenchProvider::STATE.is_enabled(4, BenchProvider::BENCHMARK),
            etw
        );
        assert_eq!(!__wpp_ifr::STATE.auto_log_context().is_null(), ifr);
        session
    }

    fn enable(&self, level: u8, keywords: u64) {
        // SAFETY: This session owns the registered provider and callback.
        unsafe { wpp_bench_enable(level, keywords) };
    }

    fn stats(&self) -> MockStats {
        let mut stats = MockStats::default();
        // SAFETY: The output is writable and this session excludes other calls.
        unsafe { wpp_bench_stats(&mut stats) };
        stats
    }

    fn assert_activity_since(&self, before: MockStats, etw: bool, ifr: bool) {
        let after = self.stats();
        let etw_calls = after.etw_writes - before.etw_writes;
        let ifr_calls = after.ifr_writes - before.ifr_writes;
        assert_eq!(etw_calls > 0, etw);
        assert_eq!(ifr_calls > 0, ifr);
        if etw && ifr {
            assert_eq!(etw_calls, ifr_calls);
        }
        assert_eq!(after.registrations, before.registrations);
        assert_eq!(after.starts, before.starts);
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        BenchProvider::clean_up();
        __wpp_ifr::cleanup();
    }
}

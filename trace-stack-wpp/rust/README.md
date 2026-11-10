# Rust WPP assembly sample
Build-only kernel-mode WPP sample.
From the repository root in PowerShell:
```powershell
# Debug: opt-level=0; emit assembly and LLVM IR.
cargo +nightly rustc --locked --profile dev -- --emit=asm,llvm-ir
# Release: opt-level=3; emit assembly and LLVM IR.
cargo +nightly rustc --locked --profile release -- --emit=asm,llvm-ir
```
Assembly: `target\debug\deps\trace_stack_app.s` / `target\release\deps\trace_stack_app.s`;

## Expanded trace statement
```rust
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn trace_sample(value: u64) {
    {
        const __WPP_LEVEL: u8 = 4u8;
        {
            const __WPP_DECODE_GUID: ::wpp::GUID =
                ::wpp::GUID {
                    data1: 791229211u32,
                    data2: 49301u16,
                    data3: 17421u16,
                    data4: [151u8, 226u8, 228u8, 229u8, 3u8, 184u8, 28u8, 215u8],
                };
            let __a0 = value;
            let __f0 = ::wpp::IntoWppField::into_wpp_field(__a0);
            let __wpp_event_id =
                {
                    fn __wpp_emit_annotation<__WppT0: ::wpp::WppField>(__f0:
                            &__WppT0) -> u16 {
                        struct Args<__WppT0>(core::marker::PhantomData<(__WppT0,)>);
                        impl<__WppT0: ::wpp::WppField> Args<__WppT0> {
                            const EVENT_ID: u16 =
                                ::wpp::field::compute_event_id("value={}",
                                    &[__WppT0::TYPE_NAME]);
                            const FORMAT_LINE: ::wpp::field::AnnotationString<512> =
                                ::wpp::field::AnnotationString::event_format_line("TraceProvider",
                                    Self::EVENT_ID,
                                    &["%0value=%10", __WppT0::FORMAT_SPEC, "",
                                                "\" //   LEVEL=TRACE_LEVEL_INFORMATION FLAGS=SAMPLE"]);
                            const ARG_0: ::wpp::field::AnnotationString<150> =
                                ::wpp::field::AnnotationString::concat("value, ",
                                    __WppT0::TYPE_NAME, " -- 10");
                        }
                        impl<__WppT0: ::wpp::WppField>
                            core::os::windows::CodeViewAnnotationArgs for Args<__WppT0>
                            {
                            const ARGS: &'static [&'static str] =
                                &["TMF:",
                                            "2f29331b-c095-440d-97e2-e4e503b81cd7 TraceProvider // SRC=TraceProvider.rs MJ= MN=",
                                            Self::FORMAT_LINE.as_str(), "{", Self::ARG_0.as_str(), "}"];
                        }
                        core::os::windows::codeview_annotation::<Args<__WppT0>>();
                        Args::<__WppT0>::EVENT_ID
                    }
                    __wpp_emit_annotation(&__f0)
                };
            let __b0 = ::wpp::WppField::as_bytes(&__f0);
            {
                let __wpp_kw_val: u64 = crate::TraceProvider::SAMPLE;
                if crate::TraceProvider::STATE.is_enabled(__WPP_LEVEL,
                        __wpp_kw_val) {
                    let __wpp_evt_desc: ::wpp::etw::EVENT_DESCRIPTOR =
                        ::wpp::etw::EVENT_DESCRIPTOR {
                            Id: __wpp_event_id,
                            Version: 0,
                            Channel: 0,
                            Level: __WPP_LEVEL,
                            Opcode: 0,
                            Task: 0,
                            Keyword: crate::TraceProvider::SAMPLE,
                        };
                    let __wpp_data:
                            [::wpp::etw::EVENT_DATA_DESCRIPTOR; 2u32 as usize] =
                        [::wpp::etw::EVENT_DATA_DESCRIPTOR {
                                    Ptr: &__WPP_DECODE_GUID as *const ::wpp::GUID as u64,
                                    Size: core::mem::size_of::<::wpp::GUID>() as u32,
                                    Reserved: ::wpp::etw::EVENT_DATA_DESCRIPTOR_TYPE_RESERVED1,
                                },
                                ::wpp::etw::EVENT_DATA_DESCRIPTOR {
                                    Ptr: __b0.as_ptr() as u64,
                                    Size: __b0.len() as u32,
                                    Reserved: 0,
                                }];
                    unsafe {
                        ::wpp::etw::write(crate::TraceProvider::STATE.reg_handle(),
                            &__wpp_evt_desc, 2u32, __wpp_data.as_ptr());
                    }
                }
            }
            {
                let __wpp_auto_ctx =
                    crate::__wpp_ifr::STATE.auto_log_context();
                if !__wpp_auto_ctx.is_null() {
                    let mut __wpp_ifr_guid = __WPP_DECODE_GUID;
                    unsafe {
                        ::wpp::ifr::WppAutoLogTrace(__wpp_auto_ctx, __WPP_LEVEL,
                            (crate::TraceProvider::SAMPLE & 0xFFFF_FFFF) as u32,
                            &mut __wpp_ifr_guid as *mut ::wpp::GUID as
                                *mut core::ffi::c_void, __wpp_event_id,
                            __b0.as_ptr() as *const core::ffi::c_void, __b0.len(),
                            core::ptr::null::<core::ffi::c_void>())
                    };
                }
            }
        };
    };
}
```

## Stack analysis for the trace frame
## 1. Total stack utilization

| Measurement                                                   |         Debug |       Release |
| ------------------------------------------------------------- | ------------: | ------------: |
| `trace_sample`’s own frame                                    | **296 bytes** | **136 bytes** |
| Deepest normal generated-code path: field conversion/checking | **696 bytes** | **136 bytes** |
| Deepest ETW enable-check path                                 | **600 bytes** | **136 bytes** |
| Deepest IFR-context / registration-handle read path           | **520 bytes** | **136 bytes** |
| Depth at native ETW callee entry                              | **400 bytes** | **144 bytes** |
| Depth at native IFR callee entry                              | **304 bytes** | **144 bytes** |

## 2. Split across helper functions

| Helper                                            | Debug own frame | Release trace path                      |
| ------------------------------------------------- | --------------: | --------------------------------------- |
| `IntoWppField::into_wpp_field`                    |               8 | Inlined                                 |
| Annotation helper                                 |               8 | Inlined; annotation metadata remains    |
| `WppField::as_bytes`                              |              40 | Inlined                                 |
| `slice::from_raw_parts::<u8>`                     |              88 | No out-of-line call                     |
| Slice precondition check                          |             152 | Eliminated for this known-valid field   |
| Pointer alignment check                           |              88 | Eliminated for this known-valid field   |
| `ProviderState::is_enabled`                       |             120 | Inlined                                 |
| Atomic load wrappers: `u8`, `u64`, pointer        |     56 **each** | Inlined                                 |
| Atomic load implementations: `u8`, `u64`, pointer |    104 **each** | Inlined                                 |
| `IFRState::auto_log_context`                      |              40 | Inlined                                 |
| `ProviderState::reg_handle`                       |              40 | Inlined                                 |
| Pointer null check                                |               8 | Inlined                                 |
| `wpp::etw::write`                                 |              88 | Inlined; direct `EtwWriteTransfer` call |

### Simultaneously active call chains

| Path | Debug calculation | Release calculation |
|---|---|---|
| Payload byte-view checking | `296 + (8+40) + (8+88) + (8+152) + (8+88) = 696` | Helpers eliminated; frame **136** |
| ETW gate through atomic load | `296 + (8+120) + (8+56) + (8+104) = 600` | Inline loads; frame **136** |
| IFR context or registration handle through atomic load | `296 + (8+40) + (8+56) + (8+104) = 520` | Inline loads; frame **136** |
| ETW native entry | `296 + 8 + 88 + 8 = 400` | `136 + 8 = 144` |
| IFR native entry | `296 + 8 = 304` | `136 + 8 = 144` |


## 3. Complete `trace_sample` frame accounting

| Non-overlapping category | Debug | Release |
|---|---:|---:|
| Outgoing register-argument home area | 32 | 32 |
| Outgoing stack-argument area | 32 | 32 |
| Payload plus primary byte-view/event-ID/context spills | 34 | 8 |
| Event/data descriptors, descriptor-construction temporaries, separate IFR GUID | 96 | 48 |
| Additional Debug/compiler bookkeeping | 90 | 0 |
| Unused/slack space outside the outgoing argument area | 12 | 16 |
| **Total** | **296** | **136** |


# C-KM assembly build
Build-only kernel-mode WPP sample; 
MSBuild below generates `main.tmh` and assembly; the `cl` commands show the explicit assembly-generation steps.
```powershell
$project = '.\trace-stack-wpp\c-km'
$kit = 'C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0'
$flags = @('/c', '/Zi', '/nologo', '/W4', '/WX', '/diagnostics:column', '/D_AMD64_', '/DWINNT', '/Gm-', '/EHsc', '/MT', '/GS', '/Gy-', '/fp:precise', '/Zc:wchar_t', '/Zc:forScope', '/Zc:inline', '/external:W4', '/Gd', '/TC', '/FC', '/kernel', '/FAs', "/I$kit\km", "/I$kit\shared")
MSBuild "$project\trace-stack-c-km.vcxproj" /t:Build /p:Configuration=Debug /p:Platform=x64
MSBuild "$project\trace-stack-c-km.vcxproj" /t:Build /p:Configuration=Release /p:Platform=x64
cl @flags /Od /Ob0 "/I$project\build\debug" "/Fa$project\build\debug\main.asm" "/Fo$project\build\debug\obj\main.obj" "/Fd$project\build\debug\compiler.pdb" "$project\main.c"
cl @flags /O2 "/I$project\build\release" "/Fa$project\build\release\main.asm" "/Fo$project\build\release\obj\main.obj" "/Fd$project\build\release\compiler.pdb" "$project\main.c"
```
Inspect `build\debug\main.asm` (`/Od /Ob0`) or `build\release\main.asm` (`/O2`). `/FAs` includes source annotations; `/Fa` sets the output path. Use `/t:Rebuild` instead of `/t:Build` to force MSBuild regeneration.

## Expanded trace statement
```c++
__declspec(noinline) void trace_sample(unsigned __int64 value)
{
    do
    {
        __annotation(L"TMF:",
                     L"5ac7f988-1b8d-38dc-544f-8bc053dfb650 c-km // SRC="
                     L"main.c"
                     L" MJ= MN=",
                     L"#typev "
                     L"main_c8"
                     L" 10 \"%0value=%10!I64u!\" //   LEVEL=TRACE_LEVEL_INFORMATION FLAGS=SAMPLE",
                     L"{", L"value, ItemULongLong -- 10", L"}"),
            0;
        BOOLEAN wppEnabled =
            (WPP_GLOBAL_Control != (WPP_PROJECT_CONTROL_BLOCK *)&WPP_GLOBAL_Control) &&
            (((WPP_GLOBAL_Control[((WPP_BIT_SAMPLE) >> 16)].Control)
                  .Flags[((0xFFFF & ((WPP_BIT_SAMPLE)-1)) / 32)] &
              (1 << (((WPP_BIT_SAMPLE)-1) & 31))) &&
             (WPP_GLOBAL_Control[((WPP_BIT_SAMPLE) >> 16)].Control).Level >= (4));
        BOOLEAN recorderEnabled =
            (WPP_RECORDER_INITIALIZED != (WPP_PROJECT_CONTROL_BLOCK *)&WPP_RECORDER_INITIALIZED) &&
            ((WPP_GLOBAL_Control[((WPP_BIT_SAMPLE) >> 16)].Control).AutoLogContext != ((void *)0));
        if (wppEnabled || recorderEnabled)
        {
            (void)0;
            WPP_RECORDER_AND_TRACE_SF_I(
                ((WPP_GLOBAL_Control[((WPP_BIT_SAMPLE) >> 16)].Control).Logger), wppEnabled,
                recorderEnabled,
                (WPP_GLOBAL_Control[((WPP_BIT_SAMPLE) >> 16)].Control).AutoLogContext, 4,
                WPP_BIT_SAMPLE, 10, WPP_5ac7f9881b8d38dc544f8bc053dfb650_Traceguids + 0, value);
        }
    } while (0);
}

__declspec(noinline) __inline void WPP_RECORDER_AND_TRACE_SF_I(
    TRACEHANDLE Logger, BOOLEAN wppEnabled, BOOLEAN recorderEnabled, PVOID AutoLogContext,
    UCHAR level, ULONG flags, USHORT id, LPCGUID traceGuid, unsigned __int64 _a1)
{
    if (wppEnabled)
    {
        pfnWppTraceMessage(Logger, WPP_TRACE_OPTIONS, (LPGUID)traceGuid, id, (&(_a1)),
                           ((SIZE_T)(sizeof(unsigned __int64))), (void *)0);
    }

    if (recorderEnabled)
    {
        WppAutoLogTrace(AutoLogContext, level, flags, (LPGUID)traceGuid, id, (&(_a1)),
                        ((SIZE_T)(sizeof(unsigned __int64))), (void *)0);
    }
}
```

## Stack analysis for the trace frame
### 1. Total stack utilization

Measured downward from `RSP` at entry to `trace_sample`, excluding its incoming return address and caller’s frame:

| Measurement                                                                           |         Debug |       Release |
| ------------------------------------------------------------------------------------- | ------------: | ------------: |
| `trace_sample`’s own frame                                                            | **104 bytes** |  **88 bytes** |
| WPP helper’s own frame, including register pushes                                     |  **72 bytes** |  **72 bytes** |
| Sum of both frame reservations                                                        | **176 bytes** | **160 bytes** |
| Depth while executing helper, including its return address                            | **184 bytes** | **168 bytes** |
| Depth at either native ETW/IFR callee’s entry, including both nested return addresses | **192 bytes** | **176 bytes** |
| If both sinks are disabled: helper is skipped                                         | **104 bytes** |  **88 bytes** |


### 2. Split by function and call path

| Function/path                          | Debug calculation        | Release calculation                |
| -------------------------------------- | ------------------------ | ---------------------------------- |
| `trace_sample`                         | `sub rsp,104`            | `sub rsp,88`                       |
| `WPP_RECORDER_AND_TRACE_SF_I`          | `sub rsp,72`             | `sub rsp,64` + `push rdi` = **72** |
| ETW entry through `pfnWppTraceMessage` | `104 + 8 + 72 + 8 = 192` | `88 + 8 + 72 + 8 = 176`            |
| IFR entry through `WppAutoLogTrace`    | `104 + 8 + 72 + 8 = 192` | `88 + 8 + 72 + 8 = 176`            |


### 3. Where the reserved stack bytes go

| Function                 | Category                                            |   Debug | Release |
| ------------------------ | --------------------------------------------------- | ------: | ------: |
| `trace_sample`           | Helper’s register-argument home area                |      32 |      32 |
| `trace_sample`           | Five outgoing stack-argument slots                  |      40 |      40 |
| `trace_sample`           | Two boolean variables plus two compiler temporaries |      10 |       0 |
| `trace_sample`           | Remaining unused/slack space                        |      22 |      16 |
| **Caller subtotal**      |                                                     | **104** |  **88** |
| WPP helper               | Native sink register-argument home area             |      32 |      32 |
| WPP helper               | Reused outgoing ETW/IFR stack-argument slots        |      32 |      32 |
| WPP helper               | Unused space                                        |       8 |       0 |
| WPP helper               | Saved RDI                                           |       0 |       8 |
| **Helper subtotal**      |                                                     |  **72** |  **72** |
| **Combined reservation** |                                                     | **176** | **160** |


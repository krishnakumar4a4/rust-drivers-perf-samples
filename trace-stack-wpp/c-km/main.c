#include <ntddk.h>

#include "trace.h"
#include "main.tmh"

__declspec(noinline) void trace_sample(unsigned __int64 value)
{
    Trace(TRACE_LEVEL_INFORMATION, SAMPLE, "value=%I64u", value);
}

DRIVER_INITIALIZE DriverEntry;

NTSTATUS DriverEntry(PDRIVER_OBJECT driver_object, PUNICODE_STRING registry_path)
{
    WPP_INIT_TRACING(driver_object, registry_path);
    trace_sample(42ULL);
    WPP_CLEANUP(driver_object);
    return STATUS_SUCCESS;
}

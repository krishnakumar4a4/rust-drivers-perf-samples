#include <windows.h>
#include <wmistr.h>
#include <evntrace.h>
#include <winternl.h>

#include "trace.h"
#include "main.tmh"

__declspec(noinline) void trace_sample(unsigned __int64 value)
{
    Trace(TRACE_LEVEL_INFORMATION, SAMPLE, "value=%I64u", value);
}

int main(void)
{
    WPP_INIT_TRACING(L"trace-stack-c-um");
    trace_sample(42ULL);
    WPP_CLEANUP();
    return 0;
}

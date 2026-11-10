#pragma once

#define WPP_CONTROL_GUIDS \
    WPP_DEFINE_CONTROL_GUID(TraceProvider, \
        (4fdb3301,5a10,4c01,9001,000000000001), \
        WPP_DEFINE_BIT(SAMPLE))

#define ENABLE_WPP_RECORDER 1
#define ENABLE_WPP_TRACE_FILTERING_WITH_WPP_RECORDER 1

#define WPP_LEVEL_FLAGS_LOGGER(level, flags) WPP_LEVEL_LOGGER(flags)
#define WPP_LEVEL_FLAGS_ENABLED(level, flags) \
    (WPP_LEVEL_ENABLED(flags) && WPP_CONTROL(WPP_BIT_ ## flags).Level >= (level))
#define WPP_RECORDER_LEVEL_FLAGS_ARGS(level, flags) \
    WPP_CONTROL(WPP_BIT_ ## flags).AutoLogContext, level, WPP_BIT_ ## flags
#define WPP_RECORDER_LEVEL_FLAGS_FILTER(level, flags) \
    (WPP_CONTROL(WPP_BIT_ ## flags).AutoLogContext != NULL)

// begin_wpp config
// FUNC Trace(LEVEL, FLAGS, MSG, ...);
// end_wpp

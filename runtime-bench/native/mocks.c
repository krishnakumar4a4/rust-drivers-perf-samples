// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REQUIRE(condition) do { \
    if (!(condition)) { \
        fputs("WPP mock contract violation: " #condition "\n", stderr); \
        abort(); \
    } \
} while (0)

#define MAX_FIELDS 8
#define MAX_PAYLOAD 256
#define REG_HANDLE UINT64_C(1)

typedef struct {
    uint16_t id;
    uint8_t version, channel, level, opcode;
    uint16_t task;
    uint64_t keyword;
} EventDescriptor;

typedef struct {
    uint64_t ptr;
    uint32_t size, reserved;
} DataDescriptor;

typedef void (__stdcall *EnableCallback)(
    const void *, uint32_t, uint8_t, uint64_t, uint64_t, const void *, void *);

typedef struct {
    uint64_t registrations, unregistrations, set_information;
    uint64_t starts, stops, etw_writes, ifr_writes;
} MockStats;

typedef struct {
    uint32_t event_id, level;
    uint64_t keyword;
    uint32_t field_count;
    uint32_t sizes[MAX_FIELDS];
    uint8_t decode_guid[16];
    uint32_t payload_len;
    uint8_t payload[MAX_PAYLOAD];
} MockRecord;

/* The fixture serializes access. These are intentionally not atomic counters. */
static MockStats stats;
static MockRecord etw_record, ifr_record;
static EnableCallback enable_callback;
static void *enable_context;
static uint8_t provider_guid[16], ifr_context;
static uint32_t registered, started, ifr_enabled, capture;

/* Use the actual Rust control-block type rather than copying its layout here. */
extern void wpp_bench_set_ifr_context(void *blocks, void *context);

void wpp_bench_reset(uint32_t enable_ifr, uint32_t capture_payloads)
{
    REQUIRE(!registered && !started);
    memset(&stats, 0, sizeof(stats));
    memset(&etw_record, 0, sizeof(etw_record));
    memset(&ifr_record, 0, sizeof(ifr_record));
    ifr_enabled = enable_ifr;
    capture = capture_payloads;
}

void wpp_bench_enable(uint8_t level, uint64_t keywords)
{
    REQUIRE(registered && enable_callback != NULL);
    enable_callback(provider_guid, level == 0 ? 0 : 1, level,
                    keywords, 0, NULL, enable_context);
}

void wpp_bench_stats(MockStats *out) { *out = stats; }
void wpp_bench_records(MockRecord *etw, MockRecord *ifr)
{
    *etw = etw_record;
    *ifr = ifr_record;
}

uint32_t __stdcall EtwRegister(
    const void *guid, EnableCallback callback, void *context, uint64_t *handle)
{
    REQUIRE(!registered && started);
    REQUIRE(guid != NULL && callback != NULL && context != NULL && handle != NULL);
    memcpy(provider_guid, guid, sizeof(provider_guid));
    enable_callback = callback;
    enable_context = context;
    registered = 1;
    *handle = REG_HANDLE;
    ++stats.registrations;
    wpp_bench_enable(5, UINT64_MAX);
    return 0;
}

uint32_t __stdcall EtwSetInformation(
    uint64_t handle, uint32_t info_class, const void *info, uint32_t length)
{
    REQUIRE(registered && handle == REG_HANDLE);
    if (info_class == 4) {
        REQUIRE(info == NULL && length == 0);
    } else {
        REQUIRE(info_class == 3 && info != NULL && length == 1);
        REQUIRE(*(const uint8_t *)info == 1);
    }
    ++stats.set_information;
    return 0;
}

uint32_t __stdcall EtwUnregister(uint64_t handle)
{
    REQUIRE(registered && handle == REG_HANDLE);
    wpp_bench_enable(0, 0);
    registered = 0;
    enable_callback = NULL;
    enable_context = NULL;
    ++stats.unregistrations;
    return 0;
}

static void capture_field(MockRecord *record, const void *data, size_t length)
{
    REQUIRE(record->field_count < MAX_FIELDS);
    REQUIRE(length <= MAX_PAYLOAD - record->payload_len);
    REQUIRE(data != NULL);
    record->sizes[record->field_count++] = (uint32_t)length;
    memcpy(record->payload + record->payload_len, data, length);
    record->payload_len += (uint32_t)length;
}

__declspec(noinline) uint32_t __stdcall EtwWriteTransfer(
    uint64_t handle, const EventDescriptor *event, const void *activity,
    const void *related_activity, uint32_t count, const DataDescriptor *data)
{
    REQUIRE(registered && handle == REG_HANDLE);
    REQUIRE(event != NULL && data != NULL && count >= 1);
    REQUIRE(activity == NULL && related_activity == NULL);
    ++stats.etw_writes;
    if (capture) {
        REQUIRE(count <= MAX_FIELDS + 1);
        REQUIRE(data[0].reserved == 4 && data[0].size == 16 && data[0].ptr != 0);
        memset(&etw_record, 0, sizeof(etw_record));
        etw_record.event_id = event->id;
        etw_record.level = event->level;
        etw_record.keyword = event->keyword;
        memcpy(etw_record.decode_guid, (const void *)(uintptr_t)data[0].ptr, 16);
        for (uint32_t i = 1; i < count; ++i) {
            REQUIRE(data[i].reserved == 0);
            capture_field(&etw_record, (const void *)(uintptr_t)data[i].ptr, data[i].size);
        }
    }
    return 0;
}

void WppAutoLogStart(void *blocks, void *driver, const void *registry_path)
{
    REQUIRE(!started && blocks != NULL && driver != NULL && registry_path != NULL);
    started = 1;
    wpp_bench_set_ifr_context(blocks, ifr_enabled ? &ifr_context : NULL);
    ++stats.starts;
}

void WppAutoLogStop(void *blocks, void *driver)
{
    REQUIRE(started && !registered && blocks != NULL && driver != NULL);
    wpp_bench_set_ifr_context(blocks, NULL);
    started = 0;
    ++stats.stops;
}

__declspec(noinline) int32_t WppAutoLogTrace(
    void *context, uint8_t level, uint32_t flags, void *guid,
    uint16_t message_number, ...)
{
    REQUIRE(started && ifr_enabled && context == &ifr_context && guid != NULL);
    ++stats.ifr_writes;
    if (capture) {
        memset(&ifr_record, 0, sizeof(ifr_record));
        ifr_record.event_id = message_number;
        ifr_record.level = level;
        ifr_record.keyword = flags;
        memcpy(ifr_record.decode_guid, guid, 16);
        va_list args;
        va_start(args, message_number);
        const void *data;
        while ((data = va_arg(args, const void *)) != NULL) {
            size_t length = va_arg(args, size_t);
            capture_field(&ifr_record, data, length);
        }
        va_end(args);
    }
    return 0;
}

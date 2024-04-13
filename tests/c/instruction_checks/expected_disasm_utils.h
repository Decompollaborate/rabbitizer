/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef EXPECTED_DISASM_UTILS_H
#define EXPECTED_DISASM_UTILS_H
#pragma once

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#ifdef NDEBUG
#error "Do not define NDEBUG"
#endif
#include <assert.h>

#define BOOL_STR(x) ((x) ? "true" : "false")

#define LOG(...)                                \
    do {                                        \
        fprintf(stderr, "%s: ", __BASE_FILE__); \
        fprintf(stderr, __VA_ARGS__);           \
    } while (0)

#define LOG_ENTRY_DATA(entry, instr)                                                                  \
    do {                                                                                              \
        fprintf(stderr, "        InstrIdType: '%s'\n", RabInstrIdType_getName((instr)->instrIdType)); \
        fprintf(stderr, "        gnuMode '%s'\n", BOOL_STR((entry)->gnuMode));                        \
        fprintf(stderr, "\n");                                                                        \
    } while (0)

size_t strlen_null(const char *string) {
    if (string == NULL) {
        return 0;
    }
    return strlen(string);
}

int strcmp_null(const char *s0, const char *s1) {
    if (s0 == s1) {
        return 0;
    }

    if (s0 == NULL) {
        return 1;
    }

    if (s1 == NULL) {
        return -1;
    }

    return strcmp(s0, s1);
}

typedef struct TestEntry {
    RabbitizerInstrCategory category;
    uint32_t word;
    const char *immOverride;
    const char *expectedStr;
    bool gnuMode;
} TestEntry;

#define TEST_ENTRY(cat, w, imm, expected, ...) \
    { .category = cat, .word = w, .immOverride = imm, .expectedStr = expected, .gnuMode = true, __VA_ARGS__ }

typedef struct InstrInitInfo {
    void (*init)(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
    void (*processUniqueId)(RabbitizerInstruction *self);
    void (*destroy)(RabbitizerInstruction *self);
} InstrInitInfo;

#define INIT_INFOS(catSuffix, plainSuffix)                                       \
    [RABBITIZER_INSTRCAT_##catSuffix] = {                                        \
        .init = RabbitizerInstruction##plainSuffix##_init,                       \
        .processUniqueId = RabbitizerInstruction##plainSuffix##_processUniqueId, \
        .destroy = RabbitizerInstruction##plainSuffix##_destroy,                 \
    }

const InstrInitInfo initInfos[] = {
    INIT_INFOS(CPU, ),
    INIT_INFOS(RSP, Rsp),
    INIT_INFOS(R3000GTE, R3000GTE),
    INIT_INFOS(R4000ALLEGREX, R4000Allegrex),
    INIT_INFOS(R5900, R5900),
};

static_assert(ARRAY_COUNT(initInfos) == RABBITIZER_INSTRCAT_MAX, "oy noy, the tests are borken");

void check_infos_validity(void) {
    static bool checked = false;
    size_t i;

    if (checked) {
        return;
    }

    for (i = 0; i < RABBITIZER_INSTRCAT_MAX; i++) {
        const InstrInitInfo *info = &initInfos[i];

        assert(info->init != NULL);
        assert(info->processUniqueId != NULL);
        assert(info->destroy != NULL);
    }

    checked = true;
}

int check_duplicated_entries(size_t entries_len, const TestEntry entries_arr[]) {
    assert(entries_arr != NULL);

    int errorCount = 0;
    size_t i;

    for (i = 0; i < entries_len; i++) {
        size_t j;

        for (j = i + 1; j < entries_len; j++) {
            if ((entries_arr[i].word == entries_arr[j].word) &&
                (strcmp_null(entries_arr[i].immOverride, entries_arr[j].immOverride) == 0)) {
                if (entries_arr[i].gnuMode == entries_arr[j].gnuMode) {
                    LOG("Duplicated entry. Word: '0x%08X'. immOverride: '%s'\n", entries_arr[i].word,
                        entries_arr[i].immOverride);
                    errorCount++;
                }
            }
        }
    }

    return errorCount;
}

bool check_expected_output(const TestEntry *entry) {
    assert(entry != NULL);
    assert(entry->category < RABBITIZER_INSTRCAT_MAX);

    bool expected = true;
    RabbitizerInstruction instr;
    char *buffer;
    size_t bufferSize;
    size_t immOverrideLength = strlen_null(entry->immOverride);
    const InstrInitInfo *info = &initInfos[entry->category];

    RabbitizerConfig_Cfg.toolchainTweaks.gnuMode = entry->gnuMode;

    check_infos_validity();

    info->init(&instr, entry->word, 0);
    info->processUniqueId(&instr);

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, immOverrideLength, 0);
    buffer = malloc(bufferSize + 1);
    assert(buffer != NULL);

    RabbitizerInstruction_disassemble(&instr, buffer, entry->immOverride, immOverrideLength, 0);

    if (entry->expectedStr == NULL) {
        LOG("Word '0x%08X' doesn't have a expected str, got '%s'\n", entry->word, buffer);
        LOG_ENTRY_DATA(entry, &instr);
        expected = false;
    } else if (strcmp(buffer, entry->expectedStr) != 0) {
        LOG("Error on word '0x%08X'. Expected '%s', got '%s'\n", entry->word, entry->expectedStr, buffer);
        LOG_ENTRY_DATA(entry, &instr);
        expected = false;
    }

    free(buffer);
    info->destroy(&instr);

    return expected;
}

// Must be defined by the test
const TestEntry test_entries[];
size_t test_entries_len;

#ifndef AVOID_DEF_MAIN
int main() {
    int errorCount = check_duplicated_entries(test_entries_len, test_entries);
    size_t i;

    if (errorCount != 0) {
        LOG("Found %i duplicated entries. Stopping\n", errorCount);

        return errorCount;
    }

    for (i = 0; i < test_entries_len; i++) {
        if (!check_expected_output(&test_entries[i])) {
            errorCount++;
        }
    }

    LOG("%i errors out of %zu entries\n\n", errorCount, test_entries_len);

    return errorCount;
}
#endif

#endif

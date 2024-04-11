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

size_t strlen_null(const char *string) {
    if (string == NULL) {
        return 0;
    }
    return strlen(string);
}

typedef struct TestEntry {
    RabbitizerInstrCategory category;
    uint32_t word;
    const char *immOverride;
    const char *expectedStr;
} TestEntry;

#define TEST_ENTRY(cat, w, imm, expected) \
    { .category = cat, .word = w, .immOverride = imm, .expectedStr = expected, }

typedef struct InstrInitInfo {
    void (*init)(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
    void (*processUniqueId)(RabbitizerInstruction *self);
    void (*destroy)(RabbitizerInstruction *self);
} InstrInitInfo;

const InstrInitInfo initInfos[] = {
    [RABBITIZER_INSTRCAT_CPU] = {
        .init = RabbitizerInstruction_init,
        .processUniqueId = RabbitizerInstruction_processUniqueId,
        .destroy = RabbitizerInstruction_destroy,
    },
    [RABBITIZER_INSTRCAT_RSP] = {
        .init = RabbitizerInstructionRsp_init,
        .processUniqueId = RabbitizerInstructionRsp_processUniqueId,
        .destroy = RabbitizerInstructionRsp_destroy,
    },
    [RABBITIZER_INSTRCAT_R3000GTE] = {
        .init = RabbitizerInstructionR3000GTE_init,
        .processUniqueId = RabbitizerInstructionR3000GTE_processUniqueId,
        .destroy = RabbitizerInstructionR3000GTE_destroy,
    },
    [RABBITIZER_INSTRCAT_R5900] = {
        .init = RabbitizerInstructionR5900_init,
        .processUniqueId = RabbitizerInstructionR5900_processUniqueId,
        .destroy = RabbitizerInstructionR5900_destroy,
    },
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

bool check_expected_output(const TestEntry *entry) {
    assert(entry != NULL);
    assert(entry->category < RABBITIZER_INSTRCAT_MAX);

    bool expected = true;
    RabbitizerInstruction instr;
    char *buffer;
    size_t bufferSize;
    size_t immOverrideLength = strlen_null(entry->immOverride);
    const InstrInitInfo *info = &initInfos[entry->category];

    check_infos_validity();

    info->init(&instr, entry->word, 0);
    info->processUniqueId(&instr);

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, immOverrideLength, 0);
    buffer = malloc(bufferSize + 1);
    assert(buffer != NULL);

    RabbitizerInstruction_disassemble(&instr, buffer, entry->immOverride, immOverrideLength, 0);

    if (entry->expectedStr == NULL) {
        printf("Word '0x%08X' doesn't have a expected str, got '%s'\n", entry->word, buffer);
        expected = false;
    } else if (strcmp(buffer, entry->expectedStr) != 0) {
        fprintf(stderr, "Error on word '0x%08X'. Expected '%s', got '%s'\n", entry->word, entry->expectedStr, buffer);
        expected = false;
    }

    free(buffer);
    info->destroy(&instr);

    return expected;
}

#endif

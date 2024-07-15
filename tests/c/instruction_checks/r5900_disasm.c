/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R5900, word, imm, expected,)

// TODO: fill
const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x70001030, NULL, "pmfhl.lw    $v0"),
    TEST_ENTRY_C(0x70001070, NULL, "pmfhl.uw    $v0"),
    TEST_ENTRY_C(0x700010B0, NULL, "pmfhl.slw   $v0"),
    TEST_ENTRY_C(0x700010F0, NULL, "pmfhl.lh    $v0"),
    TEST_ENTRY_C(0x70001130, NULL, "pmfhl.sh    $v0"),
    TEST_ENTRY_C(0x70000031, NULL, "pmthl.lw    $zero"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);

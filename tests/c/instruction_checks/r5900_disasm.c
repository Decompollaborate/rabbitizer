/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R5900, word, imm, expected,)

// TODO: fill
const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x4A000038, NULL, "vcallms     0x0"),
    TEST_ENTRY_C(0x4A004038, NULL, "vcallms     0x800"),
    TEST_ENTRY_C(0x4A008038, NULL, "vcallms     0x1000"),
    TEST_ENTRY_C(0x4A008838, NULL, "vcallms     0x1100"),
    TEST_ENTRY_C(0x4A009038, NULL, "vcallms     0x1200"),
    TEST_ENTRY_C(0x4A009838, NULL, "vcallms     0x1300"),
    TEST_ENTRY_C(0x4A00a038, NULL, "vcallms     0x1400"),
    TEST_ENTRY_C(0x4A07FFF8, NULL, "vcallms     0xFFF8"),
    TEST_ENTRY_C(0x4A080038, NULL, "vcallms     0x10000"),
    TEST_ENTRY_C(0x4A1F8038, NULL, "vcallms     0x3F000"),
    TEST_ENTRY_C(0x4A1FFFB8, NULL, "vcallms     0x3FFF0"),

    TEST_ENTRY_C(0x70001030, NULL, "pmfhl.lw    $v0"),
    TEST_ENTRY_C(0x70001070, NULL, "pmfhl.uw    $v0"),
    TEST_ENTRY_C(0x700010B0, NULL, "pmfhl.slw   $v0"),
    TEST_ENTRY_C(0x700010F0, NULL, "pmfhl.lh    $v0"),
    TEST_ENTRY_C(0x70001130, NULL, "pmfhl.sh    $v0"),
    TEST_ENTRY_C(0x70000031, NULL, "pmthl.lw    $zero"),

    TEST_ENTRY_C(0x4B020BFE, NULL, "vilwr.x     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A820BFE, NULL, "vilwr.y     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A420BFE, NULL, "vilwr.z     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A220BFE, NULL, "vilwr.w     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4B020BFF, NULL, "viswr.x     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A820BFF, NULL, "viswr.y     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A420BFF, NULL, "viswr.z     $vi2, ($vi1)"),
    TEST_ENTRY_C(0x4A220BFF, NULL, "viswr.w     $vi2, ($vi1)"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);

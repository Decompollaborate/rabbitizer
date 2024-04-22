/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, gnu, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R5900, word, imm, expected, .gnuMode=gnu,)

const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x4600600D, true,  NULL, ".word       0x4600600D                   # trunc.w.s   $f0, $f12 # 00000000 <InstrIdType: CPU_COP1_FPUS>"),
    TEST_ENTRY_C(0x46006024, true,  NULL, ".word       0x46006024                   # cvt.w.s     $f0, $f12 # 00000000 <InstrIdType: CPU_COP1_FPUS>"),
    TEST_ENTRY_C(0x4600600D, false, NULL, "trunc.w.s   $f0, $f12"),
    TEST_ENTRY_C(0x46006024, false, NULL, "cvt.w.s     $f0, $f12"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);

/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R3000GTE, word, imm, expected,)

const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x4A180001, NULL, "rtps"),
    TEST_ENTRY_C(0x4A280030, NULL, "rtpt"),
    TEST_ENTRY_C(0x4A680029, NULL, "dpcl"),
    TEST_ENTRY_C(0x4A780010, NULL, "dpcs"),
    TEST_ENTRY_C(0x4AF8002A, NULL, "dpct"),
    TEST_ENTRY_C(0x4A980011, NULL, "intpl"),
    TEST_ENTRY_C(0x4AC8041E, NULL, "ncs"),
    TEST_ENTRY_C(0x4AD80420, NULL, "nct"),
    TEST_ENTRY_C(0x4AE80413, NULL, "ncds"),
    TEST_ENTRY_C(0x4AF80416, NULL, "ncdt"),
    TEST_ENTRY_C(0x4B08041B, NULL, "nccs"),
    TEST_ENTRY_C(0x4B18043F, NULL, "ncct"),
    TEST_ENTRY_C(0x4B280414, NULL, "cdp"),
    TEST_ENTRY_C(0x4B38041C, NULL, "cc"),
    TEST_ENTRY_C(0x4B400006, NULL, "nclip"),
    TEST_ENTRY_C(0x4B58002D, NULL, "avsz3"),
    TEST_ENTRY_C(0x4B68002E, NULL, "avsz4"),

    TEST_ENTRY_C(0x4A400012, NULL, "mvmva       0, 0, 0, 0, 0"),
    TEST_ENTRY_C(0x4AA00428, NULL, "sqr         0"),
    TEST_ENTRY_C(0x4B70000C, NULL, "op          0"),
    TEST_ENTRY_C(0x4B90003D, NULL, "gpf         0"),
    TEST_ENTRY_C(0x4BA0003E, NULL, "gpl         0"),

    TEST_ENTRY_C(0x4A486012, NULL, "mvmva       1, 0, 0, 3, 0"),
    TEST_ENTRY_C(0x4A48E012, NULL, "mvmva       1, 0, 1, 3, 0"),
    TEST_ENTRY_C(0x4A496012, NULL, "mvmva       1, 0, 2, 3, 0"),
    TEST_ENTRY_C(0x4A49E012, NULL, "mvmva       1, 0, 3, 3, 0"),
    TEST_ENTRY_C(0x4A41E012, NULL, "mvmva       0, 0, 3, 3, 0"),
    TEST_ENTRY_C(0x4A480012, NULL, "mvmva       1, 0, 0, 0, 0"),
    TEST_ENTRY_C(0x4A488012, NULL, "mvmva       1, 0, 1, 0, 0"),
    TEST_ENTRY_C(0x4A490012, NULL, "mvmva       1, 0, 2, 0, 0"),
    TEST_ENTRY_C(0x4A498012, NULL, "mvmva       1, 0, 3, 0, 0"),
    TEST_ENTRY_C(0x4A482012, NULL, "mvmva       1, 0, 0, 1, 0"),
    TEST_ENTRY_C(0x4A48A012, NULL, "mvmva       1, 0, 1, 1, 0"),
    TEST_ENTRY_C(0x4A492012, NULL, "mvmva       1, 0, 2, 1, 0"),
    TEST_ENTRY_C(0x4A49A012, NULL, "mvmva       1, 0, 3, 1, 0"),
    TEST_ENTRY_C(0x4A4A6412, NULL, "mvmva       1, 1, 0, 3, 1"),
    TEST_ENTRY_C(0x4A4A6012, NULL, "mvmva       1, 1, 0, 3, 0"),
    TEST_ENTRY_C(0x4A4AE012, NULL, "mvmva       1, 1, 1, 3, 0"),
    TEST_ENTRY_C(0x4A4B6012, NULL, "mvmva       1, 1, 2, 3, 0"),
    TEST_ENTRY_C(0x4A4BE012, NULL, "mvmva       1, 1, 3, 3, 0"),
    TEST_ENTRY_C(0x4A4A0012, NULL, "mvmva       1, 1, 0, 0, 0"),
    TEST_ENTRY_C(0x4A4A8012, NULL, "mvmva       1, 1, 1, 0, 0"),
    TEST_ENTRY_C(0x4A4B0012, NULL, "mvmva       1, 1, 2, 0, 0"),
    TEST_ENTRY_C(0x4A4B8012, NULL, "mvmva       1, 1, 3, 0, 0"),
    TEST_ENTRY_C(0x4A4A2012, NULL, "mvmva       1, 1, 0, 1, 0"),
    TEST_ENTRY_C(0x4A4AA012, NULL, "mvmva       1, 1, 1, 1, 0"),
    TEST_ENTRY_C(0x4A4B2012, NULL, "mvmva       1, 1, 2, 1, 0"),
    TEST_ENTRY_C(0x4A4BA012, NULL, "mvmva       1, 1, 3, 1, 0"),
    TEST_ENTRY_C(0x4A4DA412, NULL, "mvmva       1, 2, 3, 1, 1"),
    TEST_ENTRY_C(0x4A4C6012, NULL, "mvmva       1, 2, 0, 3, 0"),
    TEST_ENTRY_C(0x4A4CE012, NULL, "mvmva       1, 2, 1, 3, 0"),
    TEST_ENTRY_C(0x4A4D6012, NULL, "mvmva       1, 2, 2, 3, 0"),
    TEST_ENTRY_C(0x4A4DE012, NULL, "mvmva       1, 2, 3, 3, 0"),
    TEST_ENTRY_C(0x4A4C0012, NULL, "mvmva       1, 2, 0, 0, 0"),
    TEST_ENTRY_C(0x4A4C8012, NULL, "mvmva       1, 2, 1, 0, 0"),
    TEST_ENTRY_C(0x4A4D0012, NULL, "mvmva       1, 2, 2, 0, 0"),
    TEST_ENTRY_C(0x4A4D8012, NULL, "mvmva       1, 2, 3, 0, 0"),
    TEST_ENTRY_C(0x4A4C2012, NULL, "mvmva       1, 2, 0, 1, 0"),
    TEST_ENTRY_C(0x4A4CA012, NULL, "mvmva       1, 2, 1, 1, 0"),
    TEST_ENTRY_C(0x4A4D2012, NULL, "mvmva       1, 2, 2, 1, 0"),
    TEST_ENTRY_C(0x4A4DA012, NULL, "mvmva       1, 2, 3, 1, 0"),
    TEST_ENTRY_C(0x4AA80428, NULL, "sqr         1"),
    TEST_ENTRY_C(0x4B78000C, NULL, "op          1"),
    TEST_ENTRY_C(0x4B98003D, NULL, "gpf         1"),
    TEST_ENTRY_C(0x4BA8003E, NULL, "gpl         1"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);

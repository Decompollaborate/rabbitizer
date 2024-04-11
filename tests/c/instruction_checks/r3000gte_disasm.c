/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R3000GTE, word, imm, expected)

const TestEntry entries[] = {
    TEST_ENTRY_C(0x4A180001, NULL, "RTPS"),
    TEST_ENTRY_C(0x4A280030, NULL, "RTPT"),
    TEST_ENTRY_C(0x4A680029, NULL, "DPCL"),
    TEST_ENTRY_C(0x4A780010, NULL, "DPCS"),
    TEST_ENTRY_C(0x4AF8002A, NULL, "DPCT"),
    TEST_ENTRY_C(0x4A980011, NULL, "INTPL"),
    TEST_ENTRY_C(0x4AC8041E, NULL, "NCS"),
    TEST_ENTRY_C(0x4AD80420, NULL, "NCT"),
    TEST_ENTRY_C(0x4AE80413, NULL, "NCDS"),
    TEST_ENTRY_C(0x4AF80416, NULL, "NCDT"),
    TEST_ENTRY_C(0x4B08041B, NULL, "NCCS"),
    TEST_ENTRY_C(0x4B18043F, NULL, "NCCT"),
    TEST_ENTRY_C(0x4B280414, NULL, "CDP"),
    TEST_ENTRY_C(0x4B38041C, NULL, "CC"),
    TEST_ENTRY_C(0x4B400006, NULL, "NCLIP"),
    TEST_ENTRY_C(0x4B58002D, NULL, "AVSZ3"),
    TEST_ENTRY_C(0x4B68002E, NULL, "AVSZ4"),

    TEST_ENTRY_C(0x4A400012, NULL, "MVMVA       0, 0, 0, 0, 0"),
    TEST_ENTRY_C(0x4AA00428, NULL, "SQR         0"),
    TEST_ENTRY_C(0x4B70000C, NULL, "OP          0"),
    TEST_ENTRY_C(0x4B90003D, NULL, "GPF         0"),
    TEST_ENTRY_C(0x4BA0003E, NULL, "GPL         0"),

    TEST_ENTRY_C(0x4A486012, NULL, "MVMVA       1, 0, 0, 3, 0"),
    TEST_ENTRY_C(0x4A48E012, NULL, "MVMVA       1, 0, 1, 3, 0"),
    TEST_ENTRY_C(0x4A496012, NULL, "MVMVA       1, 0, 2, 3, 0"),
    TEST_ENTRY_C(0x4A49E012, NULL, "MVMVA       1, 0, 3, 3, 0"),
    TEST_ENTRY_C(0x4A41E012, NULL, "MVMVA       0, 0, 3, 3, 0"),
    TEST_ENTRY_C(0x4A480012, NULL, "MVMVA       1, 0, 0, 0, 0"),
    TEST_ENTRY_C(0x4A488012, NULL, "MVMVA       1, 0, 1, 0, 0"),
    TEST_ENTRY_C(0x4A490012, NULL, "MVMVA       1, 0, 2, 0, 0"),
    TEST_ENTRY_C(0x4A498012, NULL, "MVMVA       1, 0, 3, 0, 0"),
    TEST_ENTRY_C(0x4A482012, NULL, "MVMVA       1, 0, 0, 1, 0"),
    TEST_ENTRY_C(0x4A48A012, NULL, "MVMVA       1, 0, 1, 1, 0"),
    TEST_ENTRY_C(0x4A492012, NULL, "MVMVA       1, 0, 2, 1, 0"),
    TEST_ENTRY_C(0x4A49A012, NULL, "MVMVA       1, 0, 3, 1, 0"),
    TEST_ENTRY_C(0x4A4A6412, NULL, "MVMVA       1, 1, 0, 3, 1"),
    TEST_ENTRY_C(0x4A4A6012, NULL, "MVMVA       1, 1, 0, 3, 0"),
    TEST_ENTRY_C(0x4A4AE012, NULL, "MVMVA       1, 1, 1, 3, 0"),
    TEST_ENTRY_C(0x4A4B6012, NULL, "MVMVA       1, 1, 2, 3, 0"),
    TEST_ENTRY_C(0x4A4BE012, NULL, "MVMVA       1, 1, 3, 3, 0"),
    TEST_ENTRY_C(0x4A4A0012, NULL, "MVMVA       1, 1, 0, 0, 0"),
    TEST_ENTRY_C(0x4A4A8012, NULL, "MVMVA       1, 1, 1, 0, 0"),
    TEST_ENTRY_C(0x4A4B0012, NULL, "MVMVA       1, 1, 2, 0, 0"),
    TEST_ENTRY_C(0x4A4B8012, NULL, "MVMVA       1, 1, 3, 0, 0"),
    TEST_ENTRY_C(0x4A4A2012, NULL, "MVMVA       1, 1, 0, 1, 0"),
    TEST_ENTRY_C(0x4A4AA012, NULL, "MVMVA       1, 1, 1, 1, 0"),
    TEST_ENTRY_C(0x4A4B2012, NULL, "MVMVA       1, 1, 2, 1, 0"),
    TEST_ENTRY_C(0x4A4BA012, NULL, "MVMVA       1, 1, 3, 1, 0"),
    TEST_ENTRY_C(0x4A4DA412, NULL, "MVMVA       1, 2, 3, 1, 1"),
    TEST_ENTRY_C(0x4A4C6012, NULL, "MVMVA       1, 2, 0, 3, 0"),
    TEST_ENTRY_C(0x4A4CE012, NULL, "MVMVA       1, 2, 1, 3, 0"),
    TEST_ENTRY_C(0x4A4D6012, NULL, "MVMVA       1, 2, 2, 3, 0"),
    TEST_ENTRY_C(0x4A4DE012, NULL, "MVMVA       1, 2, 3, 3, 0"),
    TEST_ENTRY_C(0x4A4C0012, NULL, "MVMVA       1, 2, 0, 0, 0"),
    TEST_ENTRY_C(0x4A4C8012, NULL, "MVMVA       1, 2, 1, 0, 0"),
    TEST_ENTRY_C(0x4A4D0012, NULL, "MVMVA       1, 2, 2, 0, 0"),
    TEST_ENTRY_C(0x4A4D8012, NULL, "MVMVA       1, 2, 3, 0, 0"),
    TEST_ENTRY_C(0x4A4C2012, NULL, "MVMVA       1, 2, 0, 1, 0"),
    TEST_ENTRY_C(0x4A4CA012, NULL, "MVMVA       1, 2, 1, 1, 0"),
    TEST_ENTRY_C(0x4A4D2012, NULL, "MVMVA       1, 2, 2, 1, 0"),
    TEST_ENTRY_C(0x4A4DA012, NULL, "MVMVA       1, 2, 3, 1, 0"),
    TEST_ENTRY_C(0x4AA80428, NULL, "SQR         1"),
    TEST_ENTRY_C(0x4AA80428, NULL, "SQR         1"),
    TEST_ENTRY_C(0x4B78000C, NULL, "OP          1"),
    TEST_ENTRY_C(0x4B70000C, NULL, "OP          0"),
    TEST_ENTRY_C(0x4B98003D, NULL, "GPF         1"),
    TEST_ENTRY_C(0x4B90003D, NULL, "GPF         0"),
    TEST_ENTRY_C(0x4BA8003E, NULL, "GPL         1"),
    TEST_ENTRY_C(0x4BA0003E, NULL, "GPL         0"),
};

int main() {
    int errorCount = 0;
    size_t i;

    for (i = 0; i < ARRAY_COUNT(entries); i++) {
        if (!check_expected_output(&entries[i])) {
            errorCount++;
        }
    }

    return errorCount;
}

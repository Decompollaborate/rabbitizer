/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R5900, word, imm, expected,)

const TestEntry entries[] = {
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

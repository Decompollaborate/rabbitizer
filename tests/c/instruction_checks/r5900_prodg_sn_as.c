/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#ifdef NDEBUG
#error "Do not define NDEBUG"
#endif

#include <assert.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>


typedef struct TestEntry {
    uint32_t word;
    const char *expectedStr;
    RabTrinaryValue r5900ProdgSnAsInvertedRegs;
} TestEntry;

const TestEntry entries[] = {
    { 0x4BE21ABC, "vadda.xyzw  $ACC, $vf2, $vf3", RAB_TRINARY_VAL_NONE },
    { 0x4BE21ABC, "vadda.xyzw  $ACC, $vf2, $vf3", RAB_TRINARY_VAL_FALSE },
    { 0x4BE21ABC, "vadda.xyzw  $ACC, $vf3, $vf2", RAB_TRINARY_VAL_TRUE },
    { 0x4BE21AFD, "vmsuba.xyzw $ACC, $vf2, $vf3", RAB_TRINARY_VAL_NONE },
    { 0x4BE21AFD, "vmsuba.xyzw $ACC, $vf2, $vf3", RAB_TRINARY_VAL_FALSE },
    { 0x4BE21AFD, "vmsuba.xyzw $ACC, $vf3, $vf2", RAB_TRINARY_VAL_TRUE },
};

int main() {
    int errorCount = 0;
    RabbitizerInstruction instr;
    size_t test_entries_len = ARRAY_COUNT(entries);

    for (size_t i = 0; i < test_entries_len; i++) {
        const TestEntry *entry = &entries[i];
        char buffer[0x100];

        memset(buffer, 0, sizeof(buffer));

        RabbitizerInstructionR5900_init(&instr, entry->word, 0x80000000);
        RAB_INSTR_FLAGS_SET_r5900ProdgSnAsInvertedRegs(&instr, entry->r5900ProdgSnAsInvertedRegs);
        RabbitizerInstructionR5900_processUniqueId(&instr);

        RabbitizerInstruction_disassemble(&instr, buffer, NULL, 0, 0);

        if (strcmp(buffer, entry->expectedStr) != 0) {
            errorCount++;
            fprintf(stderr, "Failed to disassemble 0x%u.\n", entry->word);
            fprintf(stderr, "    Expected: %s\n", entry->expectedStr);
            fprintf(stderr, "    Got:      %s\n", buffer);
        }
    }

    fprintf(stderr, "%s: ", __BASE_FILE__);
    fprintf(
        stderr,
        "%i errors out of %zu entries. %.2f%% correct.\n\n",
        errorCount,
        test_entries_len,
        (double)((test_entries_len - errorCount) / (float)test_entries_len * 100.0f)
    );

    return errorCount;
}

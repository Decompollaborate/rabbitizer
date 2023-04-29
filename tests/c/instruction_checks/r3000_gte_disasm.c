/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

size_t strlen_null(const char *string) {
    if (string == NULL) {
        return 0;
    }
    return strlen(string);
}

typedef struct TestEntry {
    uint32_t word;
    const char *immOverride;
    const char *expectedStr;
} TestEntry;

const TestEntry entries[] = {
    { 0x4A180001, NULL, "RTPS" },
    { 0x4A280030, NULL, "RTPT" },
    { 0x4A680029, NULL, "DPCL" },
    { 0x4A780010, NULL, "DPCS" },
    { 0x4AF8002A, NULL, "DPCT" },
    { 0x4A980011, NULL, "INTPL" },
    { 0x4AC8041E, NULL, "NCS" },
    { 0x4AD80420, NULL, "NCT" },
    { 0x4AE80413, NULL, "NCDS" },
    { 0x4AF80416, NULL, "NCDT" },
    { 0x4B08041B, NULL, "NCCS" },
    { 0x4B18043F, NULL, "NCCT" },
    { 0x4B280414, NULL, "CDP" },
    { 0x4B38041C, NULL, "CC" },
    { 0x4B400006, NULL, "NCLIP" },
    { 0x4B58002D, NULL, "AVSZ3" },

    { 0x4B68002E, NULL, "AVSZ4" },
    { 0x4A400012, NULL, "MVMVA sf, mx, v, cv, lm" },
    { 0x4AA00428, NULL, "SQR sf" },
    { 0x4B70000C, NULL, "OP sf" },
    { 0x4B90003D, NULL, "GPF sf" },
    { 0x4BA0003E, NULL, "GPL sf" },

    { 0x4A486012, NULL, "rtv0" },
    { 0x4A48E012, NULL, "rtv1" },
    { 0x4A496012, NULL, "rtv2" },
    { 0x4A49E012, NULL, "rtir12" },
    { 0x4A41E012, NULL, "rtir0" },
    { 0x4A480012, NULL, "rtv0tr" },
    { 0x4A488012, NULL, "rtv1tr" },
    { 0x4A490012, NULL, "rtv2tr" },
    { 0x4A498012, NULL, "rtirtr" },
    { 0x4A482012, NULL, "rtv0bk" },
    { 0x4A48A012, NULL, "rtv1bk" },
    { 0x4A492012, NULL, "rtv2bk" },
    { 0x4A49A012, NULL, "rtirbk" },
    { 0x4A4A6412, NULL, "ll" },
    { 0x4A4A6012, NULL, "llv0" },
    { 0x4A4AE012, NULL, "llv1" },
    { 0x4A4B6012, NULL, "llv2" },
    { 0x4A4BE012, NULL, "llvir" },
    { 0x4A4A0012, NULL, "llv0tr" },
    { 0x4A4A8012, NULL, "llv1tr" },
    { 0x4A4B0012, NULL, "llv2tr" },
    { 0x4A4B8012, NULL, "llirtr" },
    { 0x4A4A2012, NULL, "llv0bk" },
    { 0x4A4AA012, NULL, "llv1bk" },
    { 0x4A4B2012, NULL, "llv2bk" },
    { 0x4A4BA012, NULL, "llirbk" },
    { 0x4A4DA412, NULL, "lc" },
    { 0x4A4C6012, NULL, "lcv0" },
    { 0x4A4CE012, NULL, "lcv1" },
    { 0x4A4D6012, NULL, "lcv2" },
    { 0x4A4DE012, NULL, "lcvir" },
    { 0x4A4C0012, NULL, "lcv0tr" },
    { 0x4A4C8012, NULL, "lcv1tr" },
    { 0x4A4D0012, NULL, "lcv2tr" },
    { 0x4A4D8012, NULL, "lcirtr" },
    { 0x4A4C2012, NULL, "lev0bk" },
    { 0x4A4CA012, NULL, "lev1bk" },
    { 0x4A4D2012, NULL, "lev2bk" },
    { 0x4A4DA012, NULL, "leirbk" },
    { 0x4AA80428, NULL, "sqr12" },
    { 0x4AA80428, NULL, "sqr0" },
    { 0x4B78000C, NULL, "op12" },
    { 0x4B70000C, NULL, "op0" },
    { 0x4B98003D, NULL, "gpf12" },
    { 0x4B90003D, NULL, "gpf0" },
    { 0x4BA8003E, NULL, "gpl12" },
    { 0x4BA0003E, NULL, "gpl0" },

};

int main() {
    int errorCount = 0;
    size_t i;

    for (i = 0; i < ARRAY_COUNT(entries); i++) {
        const TestEntry *entry = &entries[i];
        RabbitizerInstruction instr;
        char *buffer;
        size_t bufferSize;
        size_t immOverrideLength = strlen_null(entry->immOverride);

        RabbitizerInstructionR3000_GTE_init(&instr, entry->word, 0);
        RabbitizerInstructionR3000_GTE_processUniqueId(&instr);

        bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, immOverrideLength, 0);
        buffer = malloc(bufferSize + 1);
        assert(buffer != NULL);

        RabbitizerInstruction_disassemble(&instr, buffer, entry->immOverride, immOverrideLength, 0);

        if (entry->expectedStr == NULL) {
            printf("Word '0x%08X' doesn't have a expected str, got '%s'\n", entry->word, buffer);
            errorCount++;
        } else if (strcmp(buffer, entry->expectedStr) != 0) {
            fprintf(stderr, "Error on word '0x%08X'. Expected '%s', got '%s'\n", entry->word, entry->expectedStr, buffer);
            errorCount++;
        }

        free(buffer);
        RabbitizerInstructionR3000_GTE_destroy(&instr);
    }

    return errorCount;
}

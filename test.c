/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>


int main() {
    uint32_t word;
    RabbitizerInstruction instr;
    char *buffer;
    int extraLJust = 5;

    // word = 0x8D4A7E18; // lw
    word = 0x00004010; // mfhi

    RabbitizerInstruction_init(&instr, word);

    RabbitizerInstruction_processUniqueId(&instr);

    buffer = malloc(RabbitizerInstruction_getSizeForBuffer(&instr, 0, extraLJust) + 1);
    assert(buffer != NULL);

    RabbitizerInstruction_disassemble(&instr, buffer, NULL, 0, extraLJust);

    printf("%08X: %s\n", word, buffer);

    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}

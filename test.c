/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include "stdio.h"

int main() {
    uint32_t word = 0x8D4A7E18;
    RabbitizerInstr instr;
    char buffer[0x1000]; // huge buffer for testing purposes

    RabbitizerInstr_Init(&instr, word);

    RabbitizerInstr_ProcessUniqueId(&instr);

    instr.extraLjustWidthOpcode += 10;

    RabbitizerInstr_Disassemble(&instr, buffer, NULL, 0);

    printf("%08X: %s\n", word, buffer);

    RabbitizerInstr_Destroy(&instr);

    return 0;
}

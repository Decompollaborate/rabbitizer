/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    |           |                              |function   |  MMI2  |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 |PMADDW | ---   |PSLLVW |PSRLVW |
001 |PMSUBW | ---   | ---   | ---   |
010 |PMFHI  |PMFLO  |PINTH  | ---   |
011 |PMULTW |PDIVW  |PCPYLD | ---   |
100 |PMADDH |PHMADH | PAND  |  PXOR |
101 |PMSUBH |PHMSBH | ---   | ---   |
110 | ---   | ---   | PEXEH | PREVH |
111 |PMULTH |PDIVBW | PEXEW |PROT3W |
 hi |-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x00, pmaddw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x02, psllvw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Variable Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x03, psrlvw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Variable Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x04, pmsubw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x08, pmfhi, R5900EE,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From HI register
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x09, pmflo, R5900EE,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From LO register
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x0A, pinth, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel INTerleave Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x0C, pmultw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MULTiply Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x0D, pdivw, R5900EE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x0E, pcpyld, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel CoPY Lower Doubleword

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x10, pmaddh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x11, phmadh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-ADd Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x12, pand, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel AND
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x13, pxor, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel eXclusive OR

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x14, pmsubh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x15, phmsbh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-Subtract Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1A, pexeh, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1B, prevh, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel REVerse Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1C, pmulth, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1D, pdivbw, R5900EE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Broadcast Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1E, pexew, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, R5900EE_MMI_2, 0x1F, prot3w, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ROTate 3 Words left

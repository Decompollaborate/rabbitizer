/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x00, pmaddw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x02, psllvw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Variable Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, psrlvw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Variable Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x04, pmsubw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x08, pmfhi,
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true
    ) // Parallel Move From HI register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x09, pmflo,
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true
    ) // Parallel Move From LO register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0A, pinth,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel INTerleave Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0C, pmultw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MULTiply Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0D, pdivw,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0E, pcpyld,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel CoPY Lower Doubleword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x10, pmaddh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x11, phmadh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-ADd Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, pand,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel AND
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x13, pxor,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel eXclusive OR

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x14, pmsubh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x15, phmsbh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-Subtract Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, pexeh,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, prevh,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel REVerse Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C, pmulth,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1D, pdivbw,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Broadcast Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1E, pexew,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1F, prot3w,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ROTate 3 Words left

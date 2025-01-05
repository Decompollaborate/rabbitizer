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

    RAB_DEF_OPCODE(
        r5900, 0x00, pmaddw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Word
    RAB_DEF_OPCODE(
        r5900, 0x02, psllvw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Variable Word
    RAB_DEF_OPCODE(
        r5900, 0x03, psrlvw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Variable Word

    RAB_DEF_OPCODE(
        r5900, 0x04, pmsubw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Word

    RAB_DEF_OPCODE(
        r5900, 0x08, pmfhi, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From HI register
    RAB_DEF_OPCODE(
        r5900, 0x09, pmflo, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From LO register
    RAB_DEF_OPCODE(
        r5900, 0x0A, pinth, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel INTerleave Halfword

    RAB_DEF_OPCODE(
        r5900, 0x0C, pmultw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MULTiply Word
    RAB_DEF_OPCODE(
        r5900, 0x0D, pdivw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Word
    RAB_DEF_OPCODE(
        r5900, 0x0E, pcpyld, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel CoPY Lower Doubleword

    RAB_DEF_OPCODE(
        r5900, 0x10, pmaddh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Halfword
    RAB_DEF_OPCODE(
        r5900, 0x11, phmadh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-ADd Halfword
    RAB_DEF_OPCODE(
        r5900, 0x12, pand, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel AND
    RAB_DEF_OPCODE(
        r5900, 0x13, pxor, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel eXclusive OR

    RAB_DEF_OPCODE(
        r5900, 0x14, pmsubh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-SUBtract Halfword
    RAB_DEF_OPCODE(
        r5900, 0x15, phmsbh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Horizontal Multiply-Subtract Halfword

    RAB_DEF_OPCODE(
        r5900, 0x1A, pexeh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Halfword
    RAB_DEF_OPCODE(
        r5900, 0x1B, prevh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel REVerse Halfword

    RAB_DEF_OPCODE(
        r5900, 0x1C, pmulth, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply Halfword
    RAB_DEF_OPCODE(
        r5900, 0x1D, pdivbw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // Parallel DIVide Broadcast Word
    RAB_DEF_OPCODE(
        r5900, 0x1E, pexew, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Even Word
    RAB_DEF_OPCODE(
        r5900, 0x1F, prot3w, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ROTate 3 Words left

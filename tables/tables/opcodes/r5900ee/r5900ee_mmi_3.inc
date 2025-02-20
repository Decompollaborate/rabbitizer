/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    |           |                               |function  |  MMI3  |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 |PMADDUW| ---   | ---   |PSRAVW |
001 | ---   | ---   | ---   | ---   |
010 |PMTHI  | PMTLO |PINTEH | ---   |
011 |PMULTUW| PDIVUW|PCPYUD | ---   |
100 | ---   | ---   |  POR  | PNOR  |
101 | ---   | ---   | ---   | ---   |
110 | ---   | ---   | PEXCH | PCPYH |
111 | ---   | ---   | PEXCW | ---   |
 hi |-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE(
        r5900ee, 0x00, pmadduw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Unsigned Word
    RAB_DEF_OPCODE(
        r5900ee, 0x03, psravw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Variable Word

    RAB_DEF_OPCODE(
        r5900ee, 0x08, pmthi, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rs),
        reads_rs: true
    ) // Parallel Move To HI register
    RAB_DEF_OPCODE(
        r5900ee, 0x09, pmtlo, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rs),
        reads_rs: true
    ) // Parallel Move To LO register
    RAB_DEF_OPCODE(
        r5900ee, 0x0A, pinteh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel INTerleave Even Halfword

    RAB_DEF_OPCODE(
        r5900ee, 0x0C, pmultuw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MULTiply Unsigned Word
    RAB_DEF_OPCODE(
        r5900ee, 0x0D, pdivuw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // Prallel DIVide Unsigned Word
    RAB_DEF_OPCODE(
        r5900ee, 0x0E, pcpyud, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel CoPY Upper Doubleword

    RAB_DEF_OPCODE(
        r5900ee, 0x12, por, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel OR
    RAB_DEF_OPCODE(
        r5900ee, 0x13, pnor, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Not OR

    RAB_DEF_OPCODE(
        r5900ee, 0x1A, pexch, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Center Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x1B, pcpyh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel CoPY Halfword

    RAB_DEF_OPCODE(
        r5900ee, 0x1E, pexcw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Center Word

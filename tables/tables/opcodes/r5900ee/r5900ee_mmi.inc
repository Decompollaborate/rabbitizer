/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = MMI     |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | MADD  | MADDU | ---   | ---   | PLZCW | ---   | ---   | ---   |
001 |  *1   |  *2   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | MFHI1 | MTHI1 | MFLO1 | MTLO1 | ---   | ---   | ---   | ---   |
011 | MULT1 | MULTU1| DIV1  | DIVU1 | ---   | ---   | ---   | ---   |
100 | MADD1 | MADDU1| ---   | ---   | ---   | ---   | ---   | ---   |
101 |  *3   |  *4   | ---   | ---   | ---   | ---   | ---   | ---   |
110 |  *5   |  *6   | ---   | ---   | PSLLH | ---   | PSRLH | PSRAH |
111 | ---   | ---   | ---   | ---   | PSLLW | ---   | PSRLW | PSRAW |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = MMI0 list
     *2 = MMI2 list
     *3 = MMI1 list
     *4 = MMI3 list
     *5 = PMFHL list
     *6 = PMTHL list
*/

    RAB_DEF_OPCODE(
        r5900ee, 0x00, madd, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD word
    RAB_DEF_OPCODE(
        r5900ee, 0x01, maddu, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD Unsigned word
    RAB_DEF_OPCODE(
        r5900ee, 0x04, plzcw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        modifies_rd: true,
        reads_rs: true
    ) // Parallel Leading Zero or one Count Word

    RAB_DEF_OPCODE(
        r5900ee, 0x10, mfhi1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Move From HI1 register
    RAB_DEF_OPCODE(
        r5900ee, 0x11, mthi1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rs),
        reads_rs: true
    ) // Move To HI1 register
    RAB_DEF_OPCODE(
        r5900ee, 0x12, mflo1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rd),
        reads_rd: true
    ) // Move From LO1 register
    RAB_DEF_OPCODE(
        r5900ee, 0x13, mtlo1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr1(Operand::core_rs),
        reads_rs: true
    ) // Move To LO1 register

    RAB_DEF_OPCODE(
        r5900ee, 0x18, mult1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MULTiply word pipeline 1
    RAB_DEF_OPCODE(
        r5900ee, 0x19, multu1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MULTiply Unsigned word pipeline 1
    RAB_DEF_OPCODE(
        r5900ee, 0x1A, div1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // DIVide word pipeline 1
    RAB_DEF_OPCODE(
        r5900ee, 0x1B, divu1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
        reads_rs: true,
        reads_rt: true
    ) // DIVide Unsigned word pipeline 1

    RAB_DEF_OPCODE(
        r5900ee, 0x20, madd1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD word pipeline 1
    RAB_DEF_OPCODE(
        r5900ee, 0x21, maddu1, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD Unsigned word pipeline 1

    RAB_DEF_OPCODE(
        r5900ee, 0x34, psllh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x36, psrlh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x37, psrah, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Halfword

    RAB_DEF_OPCODE(
        r5900ee, 0x3C, psllw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Word
    RAB_DEF_OPCODE(
        r5900ee, 0x3E, psrlw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Word
    RAB_DEF_OPCODE(
        r5900ee, 0x3F, psraw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Word

    RAB_DEF_OPCODE_REDIRECT(0x08, r5900ee, mmi_0)
    RAB_DEF_OPCODE_REDIRECT(0x09, r5900ee, mmi_2)
    RAB_DEF_OPCODE_REDIRECT(0x28, r5900ee, mmi_1)
    RAB_DEF_OPCODE_REDIRECT(0x29, r5900ee, mmi_3)
    RAB_DEF_OPCODE_REDIRECT(0x30, r5900ee, mmi_pmfhl)
    RAB_DEF_OPCODE_REDIRECT(0x31, r5900ee, mmi_pmthl)

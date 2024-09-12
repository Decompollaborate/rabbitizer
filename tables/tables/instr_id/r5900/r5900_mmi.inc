/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x00, madd,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x01, maddu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x04, plzcw,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true
    ) // Parallel Leading Zero or one Count Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x10, mfhi1,
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true
    ) // Move From HI1 register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x11, mthi1,
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true
    ) // Move To HI1 register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, mflo1,
        operands: Operand::arr1(Operand::cpu_rd),
        reads_rd: true
    ) // Move From LO1 register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x13, mtlo1,
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true
    ) // Move To LO1 register

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x18, mult1,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MULTiply word pipeline 1
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x19, multu1,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MULTiply Unsigned word pipeline 1
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, div1,
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true
    ) // DIVide word pipeline 1
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, divu1,
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true
    ) // DIVide Unsigned word pipeline 1

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x20, madd1,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD word pipeline 1
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x21, maddu1,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Multiply-ADD Unsigned word pipeline 1

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x34, psllh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x36, psrlh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x37, psrah,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x3C, psllw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Left Logical Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x3E, psrlw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Logical Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x3F, psraw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Word

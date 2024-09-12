/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x00, pmadduw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Multiply-ADD Unsigned Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, psravw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Shift Right Arithmetic Variable Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x08, pmthi,
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true
    ) // Parallel Move To HI register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x09, pmtlo,
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true
    ) // Parallel Move To LO register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0A, pinteh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel INTerleave Even Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0C, pmultuw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MULTiply Unsigned Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0D, pdivuw,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true
    ) // Prallel DIVide Unsigned Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0E, pcpyud,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel CoPY Upper Doubleword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, por,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel OR
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x13, pnor,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Not OR

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, pexch,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Center Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, pcpyh,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel CoPY Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1E, pexcw,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXchange Center Word

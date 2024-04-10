/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = SPECIAL |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLL   | ---   | *1    | SRA   | SLLV  | ---   | *2    | SRAV  |
001 | JR    | JALR  | MOVZ  | MOVN  |SYSCALL| BREAK | ---   | SYNC  |
010 | MFHI  | MTHI  | MFLO  | MTLO  | ---   | ---   | CLZ   | CLO   |
011 | MULT  | MULTU | DIV   | DIVU  | MADD  | MADDU | ---   | ---   |
100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
101 | ---   | ---   | SLT   | SLTU  | MAX   | MIN   | MSUB  | MSUBU |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL rs, see SPECIAL rs list
     *2 = SPECIAL sa, see SPECIAL sa list
*/

    // The other instructions are implemented using the main CPU table

    // OP rd, rs

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x16, clz,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rs},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true
    ) // Count Leading Zero

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x17, clo,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rs},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true
    ) // Count Leading One

    // OP rs, rt

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x1C, madd,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRs=true,
        .readsRt=true,
        .modifiesHI=true,
        .modifiesLO=true
    ) // Multiply ADD

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x1D, maddu,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRs=true,
        .readsRt=true,
        .modifiesHI=true,
        .modifiesLO=true
    ) // Multiply ADD Unsigned

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x2E, msub,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRs=true,
        .readsRt=true,
        .modifiesHI=true,
        .modifiesLO=true
    ) // Multiply SUBtract

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x2F, msubu,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRs=true,
        .readsRt=true,
        .modifiesHI=true,
        .modifiesLO=true
    ) // Multiply SUBtract Unsigned

    // OP rd, rs, rt

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x2C, max,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true,
        .readsRt=true
    ) // select MAX

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x2D, min,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true,
        .readsRt=true
    ) // select MIN

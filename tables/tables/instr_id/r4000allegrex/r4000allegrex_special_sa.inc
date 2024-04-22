/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26--------------------------------10------6-5--------0
    | = SPECIAL |                               |   sa    |  0x06   |
    ------6------------------------------------------5---------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | srlv  | rotrv | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, srlv,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_rs},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true,
        .readsRt=true
    ) // Shift word Right Logical Variable

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, rotrv,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_rs},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true,
        .readsRt=true
    ) // ROTate word Right Variable

/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21---------------------------------5--------0
    | = SPECIAL |   rs    |                               |  0x02   |
    ------6----------5-----------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | srl   | rotr  | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, srl,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_sa},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // Shift word Right Logical

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, rotr,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_sa},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // ROTate word Right

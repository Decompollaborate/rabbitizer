/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | =SPECIAL3 |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | EXT   | ---   | ---   | ---   | INS   | ---   | ---   | ---   |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL3 BSHFL
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, ext,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_rs, RAB_OPERAND_r4000allegrex_pos, RAB_OPERAND_r4000allegrex_size},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRt=true,
        .readsRs=true
    ) // EXTract bit field

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x04, ins,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_rs, RAB_OPERAND_r4000allegrex_pos, RAB_OPERAND_r4000allegrex_size_plus_pos},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRt=true,
        .readsRs=true
    ) // INSert bit field

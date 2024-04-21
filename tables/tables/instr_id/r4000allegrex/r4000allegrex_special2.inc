/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | =SPECIAL2 |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLEEP | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | MFIE  | ---   | MTIE  | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, sleep,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_R
    )

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x24, mfie,
        .operands={RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRt=true
    )

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x26, mtie,
        .operands={RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRt=true
    )

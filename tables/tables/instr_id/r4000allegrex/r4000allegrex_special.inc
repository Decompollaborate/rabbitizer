/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = SPECIAL |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ---   | ---   | *1    | ---   | ---   | ---   | *2    | ---   |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | clo   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL rs, see SPECIAL rs list
     *2 = SPECIAL sa, see SPECIAL sa list
*/

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x17, clo,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rs},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRs=true
    ) // Count Leading One

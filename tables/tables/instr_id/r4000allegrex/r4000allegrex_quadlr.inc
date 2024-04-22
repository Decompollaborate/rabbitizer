/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-----------------------------------------------2-1-0
    | = QUADLR  |                                               |Z| |
    ------6------------------------------------------------------1---
    |---------------0---------------|---------------1---------------| lo
    |             SVL.Q             |             SVR.Q             |
    |-------------------------------|-------------------------------|
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0, svl_q, svl.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    ) // Store Quad word Left from Vfpu

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 1, svr_q, svr.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    ) // Store Quad word Right from Vfpu

/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
  LVL = 110101
    31--------26-------------------------------------------------1--0
    |    LVL    |                                               |Z| |
    ------6------------------------------------------------------1---
    |---------------0---------------|---------------1---------------| lo
    | LVL.Q                         | LVR.Q                         |
    |-------------------------------|-------------------------------|
*/

/*
lvl.q
    31--------26-25-----21-20-----16-15------------------------2-1--0
    |    LVL    |    RS   |    VT   |          OFFSET14         |0|VT|
    ------6----------5---------5------------------14-------------1-1-
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0, lvl_q, lvl.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    )

/*
lvr.q
    31--------26-25-----21-20-----16-15------------------------2-1--0
    |    LVL    |    RS   |    VT   |          OFFSET14         |1|VT|
    ------6----------5---------5------------------14-------------1-1-
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1, lvr_q, lvr.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    )

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

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_LVL, 0x0, lvl_q, lvl.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_rs),
        instr_type: InstrType::I,
        reads_rs: true
    )

/*
lvr.q
    31--------26-25-----21-20-----16-15------------------------2-1--0
    |    LVL    |    RS   |    VT   |          OFFSET14         |1|VT|
    ------6----------5---------5------------------14-------------1-1-
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_LVL, 0x1, lvr_q, lvr.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_rs),
        instr_type: InstrType::I,
        reads_rs: true
    )

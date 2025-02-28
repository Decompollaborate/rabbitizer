/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-----------------------------------------------2-1-0
    | = QUADLR  |                                               |Z| |
    ------6------------------------------------------------------1---
    |---------------0---------------|---------------1---------------| lo
    |             SVL.Q             |             SVR.Q             |
    |-------------------------------|-------------------------------|
*/

    RAB_DEF_OPCODE_EXTENSION_ALTNAME(
        r4000allegrex, 0x0, svl_q, svl.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Store Quad word Left from Vfpu

    RAB_DEF_OPCODE_EXTENSION_ALTNAME(
        r4000allegrex, 0x1, svr_q, svr.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Store Quad word Right from Vfpu

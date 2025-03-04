/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-----------------------------------------0
    |  = COP0   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC0  | ---   | ---   | ---   | MTC0  | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP rt, cop0d
    RAB_DEF_OPCODE_EXT(
        rsp, 0x00, mfc0, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Move word From CP0
    RAB_DEF_OPCODE_EXT(
        rsp, 0x04, mtc0, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Move word to CP0

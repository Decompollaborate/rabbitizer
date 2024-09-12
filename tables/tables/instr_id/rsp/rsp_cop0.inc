/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rt, cop0d
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, mfc0,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Move word From CP0
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, mtc0,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Move word to CP0

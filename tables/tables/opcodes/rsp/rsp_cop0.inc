/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rt, cop0d
    RAB_DEF_OPCODE(
        rsp, 0x00, mfc0, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Move word From CP0
    RAB_DEF_OPCODE(
        rsp, 0x04, mtc0, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Move word to CP0

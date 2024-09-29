/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP cop2t, vd[index]
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, mfc2, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, mtc2, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index)
    )

    // OP rt, cop2cd
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, cfc2, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
        modifies_rt: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, ctc2, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
        reads_rt: true
    )

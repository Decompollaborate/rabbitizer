/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-20------16------------------------------0
    |  = COP0   |   BC0   |   fmt   |                               |
    ------6----------5----------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BC0F  | BC0T  | BC0FL | BC0TL | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0x00, bc0f, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on FP False
    RABBITIZER_DEF_INSTR_ID(
        core, 0x01, bc0t, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on FP True
    RABBITIZER_DEF_INSTR_ID(
        core, 0x02, bc0fl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on FP False Likely
    RABBITIZER_DEF_INSTR_ID(
        core, 0x03, bc0tl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on FP True Likely

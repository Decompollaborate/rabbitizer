/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
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
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP0_BC0, 0x00, bc0f,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on FP False
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP0_BC0, 0x01, bc0t,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on FP True
    RAB_DEF_OPCODE(
        core, CORE_COP0_BC0, 0x02, bc0fl, MIPS_II,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on FP False Likely
    RAB_DEF_OPCODE(
        core, CORE_COP0_BC0, 0x03, bc0tl, MIPS_II,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on FP True Likely

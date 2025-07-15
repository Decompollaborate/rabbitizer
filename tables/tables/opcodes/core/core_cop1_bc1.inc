/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-20------16------------------------------0
    |  = COP1   |   BC1   |   fmt   |                               |
    ------6----------5----------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BC1F  | BC1T  | BC1FL | BC1TL | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP IMM
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP1_BC1, 0x00, bc1f,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    )
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP1_BC1, 0x01, bc1t,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    )
    RAB_DEF_OPCODE(
        core, CORE_COP1_BC1, 0x02, bc1fl, MIPS_II,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    )
    RAB_DEF_OPCODE(
        core, CORE_COP1_BC1, 0x03, bc1tl, MIPS_II,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    )

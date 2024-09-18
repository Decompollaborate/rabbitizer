/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x00, bc1f, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, bc1t, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, bc1fl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x03, bc1tl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    )

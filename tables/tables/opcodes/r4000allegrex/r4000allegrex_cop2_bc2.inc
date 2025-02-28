/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21------17-16-------------------------------0
    |  = COP2   |   BC2   |     |fmt|                               |
    ------6----------5------------2----------------------------------
    |---------------0---------------|---------------1---------------| lo
  0 |              BVF              |              BVT              |
  1 |              BVFL             |              BVTL             |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP IMM
    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x00, bvf, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_imm3, Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on Vfpu False
    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x01, bvt, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_imm3, Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true
    ) // Branch on Vfpu True
    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x02, bvfl, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_imm3, Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on Vfpu False Likely
    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x03, bvtl, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_imm3, Operand::core_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true
    ) // Branch on Vfpu True Likely

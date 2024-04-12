/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, bvf,
        .operands={RAB_OPERAND_r4000allegrex_imm3, RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on Vfpu False
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, bvt,
        .operands={RAB_OPERAND_r4000allegrex_imm3, RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on Vfpu True
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, bvfl,
        .operands={RAB_OPERAND_r4000allegrex_imm3, RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on Vfpu False Likely
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x03, bvtl,
        .operands={RAB_OPERAND_r4000allegrex_imm3, RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on Vfpu True Likely

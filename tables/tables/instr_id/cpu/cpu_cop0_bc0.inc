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
        cpu, 0x00, bc0f,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on FP False
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, bc0t,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on FP True
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, bc0fl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on FP False Likely
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x03, bc0tl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on FP True Likely

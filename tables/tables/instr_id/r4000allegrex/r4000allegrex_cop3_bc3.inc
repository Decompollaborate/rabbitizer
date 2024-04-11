/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-20------16------------------------------0
    |  = COP2   |   BC3   |   fmt   |                               |
    ------6----------5----------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BC3F  | BC3T  | BC3FL | BC3TL | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, bc3f,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on FP False
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, bc3t,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    ) // Branch on FP True
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, bc3fl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on FP False Likely
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x03, bc3tl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    ) // Branch on FP True Likely

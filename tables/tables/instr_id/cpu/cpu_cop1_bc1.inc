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
        cpu, 0x00, bc1f,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, bc1t,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, bc1fl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x03, bc1tl,
        .operands={RAB_OPERAND_cpu_branch_target_label},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isBranch=true,
        .isBranchLikely=true
    )

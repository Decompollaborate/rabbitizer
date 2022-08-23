/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x00, bc1f,    .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true)
RABBITIZER_DEF_INSTR_ID(cpu, 0x01, bc1t,    .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true)
RABBITIZER_DEF_INSTR_ID(cpu, 0x02, bc1fl,   .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true)
RABBITIZER_DEF_INSTR_ID(cpu, 0x03, bc1tl,   .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true)

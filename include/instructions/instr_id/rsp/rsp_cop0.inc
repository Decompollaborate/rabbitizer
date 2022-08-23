/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP rt, cop0d
RABBITIZER_DEF_INSTR_ID(rsp, 0x00, mfc0,    .operands={RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_RSP_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true, .notEmitedByCompilers=true)      // Move word From CP0
RABBITIZER_DEF_INSTR_ID(rsp, 0x04, mtc0,    .operands={RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_RSP_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true)      // Move word to CP0

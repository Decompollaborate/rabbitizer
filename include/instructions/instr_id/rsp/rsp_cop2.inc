/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP cop2t, vd[index]
RABBITIZER_DEF_INSTR_ID(rsp, 0x00, mfc2,    .operands={RABBITIZER_OPERAND_TYPE_RSP_cop2t, RABBITIZER_OPERAND_TYPE_RSP_vs_index}, .modifiesRt=true)
RABBITIZER_DEF_INSTR_ID(rsp, 0x04, mtc2,    .operands={RABBITIZER_OPERAND_TYPE_RSP_cop2t, RABBITIZER_OPERAND_TYPE_RSP_vs_index})

// OP rt, cop2cd
RABBITIZER_DEF_INSTR_ID(rsp, 0x02, cfc2,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_cop2cd}, .modifiesRt=true)
RABBITIZER_DEF_INSTR_ID(rsp, 0x06, ctc2,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_cop2cd})

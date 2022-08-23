/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP rs, IMM
RABBITIZER_DEF_INSTR_ID(rsp, 0x00, bltz,    .operands={RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true)      // Branch on Less Than Zero
RABBITIZER_DEF_INSTR_ID(rsp, 0x01, bgez,    .operands={RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true)      // Branch on Greater than or Equal to Zero

RABBITIZER_DEF_INSTR_ID(rsp, 0x10, bltzal,  .operands={RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true) // Branch on Less Than Zero and Link
RABBITIZER_DEF_INSTR_ID(rsp, 0x11, bgezal,  .operands={RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true) // Branch on Greater Than or Equal to Zero and Link

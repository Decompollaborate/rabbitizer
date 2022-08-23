/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP rt, fs
RABBITIZER_DEF_INSTR_ID(cpu, 0x00, mfc1,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)      // Move Word From Floating-Point
RABBITIZER_DEF_INSTR_ID(cpu, 0x01, dmfc1,   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)     // Doubleword Move From Floating-Point
RABBITIZER_DEF_INSTR_ID(cpu, 0x04, mtc1,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)      // Move Word to Floating-Point
RABBITIZER_DEF_INSTR_ID(cpu, 0x05, dmtc1,   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Doubleword Move To Floating-Point

// OP rt, cop1cs
RABBITIZER_DEF_INSTR_ID(cpu, 0x02, cfc1,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)      // Move Control Word from Floating-Point
RABBITIZER_DEF_INSTR_ID(cpu, 0x06, ctc1,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)      // Move Control Word to Floating-Point

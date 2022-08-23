/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP rs, IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x00, bltz,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Branch on Less Than Zero
RABBITIZER_DEF_INSTR_ID(cpu, 0x01, bgez,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Branch on Greater than or Equal to Zero
RABBITIZER_DEF_INSTR_ID(cpu, 0x02, bltzl,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)     // Branch on Less Than Zero Likely
RABBITIZER_DEF_INSTR_ID(cpu, 0x03, bgezl,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)     // Branch on Greater than or Equal to Zero Likely

RABBITIZER_DEF_INSTR_ID(cpu, 0x08, tgei,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)
RABBITIZER_DEF_INSTR_ID(cpu, 0x09, tgeiu,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)
RABBITIZER_DEF_INSTR_ID(cpu, 0x0A, tlti,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)
RABBITIZER_DEF_INSTR_ID(cpu, 0x0B, tltiu,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)
RABBITIZER_DEF_INSTR_ID(cpu, 0x0C, teqi,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)
RABBITIZER_DEF_INSTR_ID(cpu, 0x0E, tnei,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)

RABBITIZER_DEF_INSTR_ID(cpu, 0x10, bltzal,  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I) // Branch on Less Than Zero and Link
RABBITIZER_DEF_INSTR_ID(cpu, 0x11, bgezal,  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I) // Branch on Greater Than or Equal to Zero and Link
RABBITIZER_DEF_INSTR_ID(cpu, 0x12, bltzall, .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II) // Branch on Less Than Zero and Link Likely
RABBITIZER_DEF_INSTR_ID(cpu, 0x13, bgezall, .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II) // Branch on Greater Than or Equal to Zero and Link Likely

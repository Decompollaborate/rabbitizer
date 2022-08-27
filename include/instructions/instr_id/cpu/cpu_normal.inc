/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP LABEL
RABBITIZER_DEF_INSTR_ID(cpu, 0x02, j,       .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)         // Jump
RABBITIZER_DEF_INSTR_ID(cpu, 0x03, jal,     .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Jump And Link

// OP rs, rt, IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x04, beq,     .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Branch on EQual
RABBITIZER_DEF_INSTR_ID(cpu, 0x05, bne,     .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Branch on Not Equal
RABBITIZER_DEF_INSTR_ID(cpu, 0x14, beql,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Branch on EQual Likely
RABBITIZER_DEF_INSTR_ID(cpu, 0x15, bnel,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Branch on Not Equal Likely

// OP rs, IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x06, blez,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Branch on Less than or Equal to Zero
RABBITIZER_DEF_INSTR_ID(cpu, 0x16, blezl,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)     // Branch on Less than or Equal to Zero Likely
RABBITIZER_DEF_INSTR_ID(cpu, 0x07, bgtz,    .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Branch on Greater Than Zero
RABBITIZER_DEF_INSTR_ID(cpu, 0x17, bgtzl,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)     // Branch on Greater Than Zero Likely

// OP rt, rs, IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x08, addi,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .notEmitedByCompilers=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Add Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x09, addiu,   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)     // Add Immediate Unsigned Word
RABBITIZER_DEF_INSTR_ID(cpu, 0x0A, slti,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Set on Less Than Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x0B, sltiu,   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)     // Set on Less Than Immediate Unsigned
RABBITIZER_DEF_INSTR_ID(cpu, 0x0C, andi,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // And Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x0D, ori,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Or Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x0E, xori,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // eXclusive OR Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x18, daddi,   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)     // Doubleword add Immediate
RABBITIZER_DEF_INSTR_ID(cpu, 0x19, daddiu,  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)    // Doubleword add Immediate Unsigned

// OP rt, IMM
RABBITIZER_DEF_INSTR_ID(cpu, 0x0F, lui,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeHi=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Load Upper Immediate

// OP rt, IMM(base)
RABBITIZER_DEF_INSTR_ID(cpu, 0x1A, ldl,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Load Doubleword Left
RABBITIZER_DEF_INSTR_ID(cpu, 0x1B, ldr,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Load Doubleword Right
RABBITIZER_DEF_INSTR_ID(cpu, 0x20, lb,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Load Byte
RABBITIZER_DEF_INSTR_ID(cpu, 0x21, lh,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Load Halfword
RABBITIZER_DEF_INSTR_ID(cpu, 0x22, lwl,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Load Word Left
RABBITIZER_DEF_INSTR_ID(cpu, 0x23, lw,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Load Word
RABBITIZER_DEF_INSTR_ID(cpu, 0x24, lbu,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Load Byte Insigned
RABBITIZER_DEF_INSTR_ID(cpu, 0x25, lhu,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Load Halfword Unsigned
RABBITIZER_DEF_INSTR_ID(cpu, 0x26, lwr,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Load Word Right
RABBITIZER_DEF_INSTR_ID(cpu, 0x27, lwu,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Load Word Unsigned
RABBITIZER_DEF_INSTR_ID(cpu, 0x28, sb,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Store Byte
RABBITIZER_DEF_INSTR_ID(cpu, 0x29, sh,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Store Halfword
RABBITIZER_DEF_INSTR_ID(cpu, 0x2A, swl,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Store Word Left
RABBITIZER_DEF_INSTR_ID(cpu, 0x2B, sw,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)        // Store Word
RABBITIZER_DEF_INSTR_ID(cpu, 0x2C, sdl,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Store Doubleword Left
RABBITIZER_DEF_INSTR_ID(cpu, 0x2D, sdr,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Store Doubleword Right
RABBITIZER_DEF_INSTR_ID(cpu, 0x2E, swr,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)       // Store Word Right
RABBITIZER_DEF_INSTR_ID(cpu, 0x30, ll,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)        // Load Linked word
RABBITIZER_DEF_INSTR_ID(cpu, 0x33, pref,    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_IV)      // Prefetch
RABBITIZER_DEF_INSTR_ID(cpu, 0x34, lld,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Load Linked Doubleword
RABBITIZER_DEF_INSTR_ID(cpu, 0x37, ld,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)        // Load Doubleword
RABBITIZER_DEF_INSTR_ID(cpu, 0x38, sc,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)        // Store Conditional word
RABBITIZER_DEF_INSTR_ID(cpu, 0x3C, scd,     .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)       // Store Conditional Doubleword
RABBITIZER_DEF_INSTR_ID(cpu, 0x3F, sd,      .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III)        // Store Doubleword

// OP op, IMM(base)
RABBITIZER_DEF_INSTR_ID(cpu, 0x2F, cache,   .operands={RABBITIZER_OPERAND_TYPE_op, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .notEmitedByCompilers=true)     // Cache

// OP ft, IMM(base)
RABBITIZER_DEF_INSTR_ID(cpu, 0x31, lwc1,    .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Load Word to Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x35, ldc1,    .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Load Doubleword to Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x39, swc1,    .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Store Word from Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x3D, sdc1,    .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Store Doubleword from Coprocessor z

// OP cop2t, IMM(base)
RABBITIZER_DEF_INSTR_ID(cpu, 0x32, lwc2,    .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Load Word to Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x36, ldc2,    .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Load Doubleword to Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x3A, swc2,    .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I)      // Store Word from Coprocessor z
RABBITIZER_DEF_INSTR_ID(cpu, 0x3E, sdc2,    .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II)      // Store Doubleword from Coprocessor z


// Pseudo-Instruction Unique IDs

// OP IMM
RABBITIZER_DEF_INSTR_ID(cpu, -0x03, b,      .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true)         // Branch (unconditional)

// OP rs, IMM
RABBITIZER_DEF_INSTR_ID(cpu, -0x04, beqz,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true)      // Branch on EQual Zero
RABBITIZER_DEF_INSTR_ID(cpu, -0x05, bnez,   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true)      // Branch on Not Equal Zero

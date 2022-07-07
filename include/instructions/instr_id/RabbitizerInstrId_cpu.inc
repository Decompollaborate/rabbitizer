/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(cpu, INVALID,                       .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1),

    // OP rs
    RABBITIZER_DEF_INSTR_ID(cpu, mthi,                          .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I), // Move To HI register
    RABBITIZER_DEF_INSTR_ID(cpu, mtlo,                          .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Move To LO register
    RABBITIZER_DEF_INSTR_ID(cpu, jr,                            .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),                        // Jump Register
    RABBITIZER_DEF_INSTR_ID(cpu, jalr,                          .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Jump And Link Register

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, jalr_rd, jalr,         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),   // Jump And Link Register // Special case for rd != 31

    // OP rd
    RABBITIZER_DEF_INSTR_ID(cpu, mfhi,                          .operands={RABBITIZER_OPERAND_TYPE_rd}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Move From HI register
    RABBITIZER_DEF_INSTR_ID(cpu, mflo,                          .operands={RABBITIZER_OPERAND_TYPE_rd}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Move From LO register

    // OP rs, rt
    RABBITIZER_DEF_INSTR_ID(cpu, mult,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // MULTtiply word
    RABBITIZER_DEF_INSTR_ID(cpu, multu,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),     // MULTtiply Unsigned word
    RABBITIZER_DEF_INSTR_ID(cpu, dmult,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword MULTiply
    RABBITIZER_DEF_INSTR_ID(cpu, dmultu,                        .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),    // Doubleword MULTiply Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, tge,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),       // Trap if Greater or Equal
    RABBITIZER_DEF_INSTR_ID(cpu, tgeu,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Trap if Greater or Equal Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, tlt,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),       // Trap if Less Than
    RABBITIZER_DEF_INSTR_ID(cpu, tltu,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Trap if Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, teq,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),       // Trap if EQual
    RABBITIZER_DEF_INSTR_ID(cpu, tne,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),       // Trap if Not Equal

    // OP rd, rs, rt
    RABBITIZER_DEF_INSTR_ID(cpu, movz,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_IV),      // MOVe conditional on Zero
    RABBITIZER_DEF_INSTR_ID(cpu, movn,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_IV),      // MOVe conditional on Not zero
    RABBITIZER_DEF_INSTR_ID(cpu, div,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // DIVide word
    RABBITIZER_DEF_INSTR_ID(cpu, divu,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sn64_div, div,         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),  // DIVide word
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sn64_divu, divu,       .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I), // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID(cpu, ddiv,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword DIVide
    RABBITIZER_DEF_INSTR_ID(cpu, ddivu,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword DIVide Unsigned
    // RABBITIZER_DEF_INSTR_ID(cpu, ddiv,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),
    // RABBITIZER_DEF_INSTR_ID(cpu, ddivu,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),
    RABBITIZER_DEF_INSTR_ID(cpu, add,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .notEmitedByCompilers=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // ADD word
    RABBITIZER_DEF_INSTR_ID(cpu, addu,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .maybeIsMove=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(cpu, sub,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Subtract word
    RABBITIZER_DEF_INSTR_ID(cpu, subu,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // SUBtract Unsigned word
    RABBITIZER_DEF_INSTR_ID(cpu, and,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // AND
    RABBITIZER_DEF_INSTR_ID(cpu, or,                            .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .maybeIsMove=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // OR
    RABBITIZER_DEF_INSTR_ID(cpu, xor,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // eXclusive OR
    RABBITIZER_DEF_INSTR_ID(cpu, nor,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Not OR
    RABBITIZER_DEF_INSTR_ID(cpu, slt,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Set on Less Than
    RABBITIZER_DEF_INSTR_ID(cpu, sltu,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Set on Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, dadd,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword Add
    RABBITIZER_DEF_INSTR_ID(cpu, daddu,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .maybeIsMove=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword Add Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, dsub,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword SUBtract
    RABBITIZER_DEF_INSTR_ID(cpu, dsubu,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword SUBtract Unsigned

    // OP code
    RABBITIZER_DEF_INSTR_ID(cpu, syscall,                       .operands={RABBITIZER_OPERAND_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),   // SYStem CALL
    RABBITIZER_DEF_INSTR_ID(cpu, break,                         .operands={RABBITIZER_OPERAND_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),     // Break
    RABBITIZER_DEF_INSTR_ID(cpu, sync,                          .operands={RABBITIZER_OPERAND_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Sync

    // OP rd, rt, rs
    RABBITIZER_DEF_INSTR_ID(cpu, dsllv,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword Shift Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(cpu, dsrlv,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword Shift Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(cpu, dsrav,                         .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword Shift Right Arithmetic Variable
    RABBITIZER_DEF_INSTR_ID(cpu, sllv,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Shift word Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(cpu, srlv,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Shift word Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(cpu, srav,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Shift word Right Arithmetic Variable

    // OP rd, rt, sa
    RABBITIZER_DEF_INSTR_ID(cpu, sll,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Shift word Left Logical
    RABBITIZER_DEF_INSTR_ID(cpu, srl,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Shift word Right Logical
    RABBITIZER_DEF_INSTR_ID(cpu, sra,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Shift word Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(cpu, dsll,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword Shift Left Logical
    RABBITIZER_DEF_INSTR_ID(cpu, dsrl,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword Shift Right Logical
    RABBITIZER_DEF_INSTR_ID(cpu, dsra,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),      // Doubleword Shift Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(cpu, dsll32,                        .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),    // Doubleword Shift Left Logical plus 32
    RABBITIZER_DEF_INSTR_ID(cpu, dsrl32,                        .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),    // Doubleword Shift Right Logical plus 32
    RABBITIZER_DEF_INSTR_ID(cpu, dsra32,                        .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),    // Doubleword Shift Right Arithmetic plus 32

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, bltz,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Branch on Less Than Zero
    RABBITIZER_DEF_INSTR_ID(cpu, bgez,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Branch on Greater than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(cpu, bltzl,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),     // Branch on Less Than Zero Likely
    RABBITIZER_DEF_INSTR_ID(cpu, bgezl,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),     // Branch on Greater than or Equal to Zero Likely
    RABBITIZER_DEF_INSTR_ID(cpu, tgei,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),
    RABBITIZER_DEF_INSTR_ID(cpu, tgeiu,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),
    RABBITIZER_DEF_INSTR_ID(cpu, tlti,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),
    RABBITIZER_DEF_INSTR_ID(cpu, tltiu,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),
    RABBITIZER_DEF_INSTR_ID(cpu, bltzal,                        .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I), // Branch on Less Than Zero and Link
    RABBITIZER_DEF_INSTR_ID(cpu, bgezal,                        .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I), // Branch on Greater Than or Equal to Zero and Link
    RABBITIZER_DEF_INSTR_ID(cpu, bltzall,                       .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II), // Branch on Less Than Zero and Link Likely
    RABBITIZER_DEF_INSTR_ID(cpu, bgezall,                       .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II), // Branch on Greater Than or Equal to Zero and Link Likely
    RABBITIZER_DEF_INSTR_ID(cpu, teqi,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),
    RABBITIZER_DEF_INSTR_ID(cpu, tnei,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),

    // OP LABEL
    RABBITIZER_DEF_INSTR_ID(cpu, j,                             .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),         // Jump
    RABBITIZER_DEF_INSTR_ID(cpu, jal,                           .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true, .doesLink=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Jump And Link

    // OP rs, rt, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, beq,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Branch on EQual
    RABBITIZER_DEF_INSTR_ID(cpu, bne,                           .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Branch on Not Equal
    RABBITIZER_DEF_INSTR_ID(cpu, beql,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Branch on EQual Likely
    RABBITIZER_DEF_INSTR_ID(cpu, bnel,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Branch on Not Equal Likely

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, blez,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Branch on Less than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(cpu, bgtz,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Branch on Greater Than Zero
    RABBITIZER_DEF_INSTR_ID(cpu, blezl,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),     // Branch on Less than or Equal to Zero Likely
    RABBITIZER_DEF_INSTR_ID(cpu, bgtzl,                         .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isBranchLikely=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),     // Branch on Greater Than Zero Likely

    // OP rt, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, lui,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeHi=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Upper Immediate

    // OP rt, rs, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, andi,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // And Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, ori,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Or Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, xori,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // eXclusive OR Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, addi,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .notEmitedByCompilers=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Add Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, addiu,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),     // Add Immediate Unsigned Word
    RABBITIZER_DEF_INSTR_ID(cpu, daddi,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),     // Doubleword add Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, daddiu,                        .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),    // Doubleword add Immediate Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, slti,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Set on Less Than Immediate
    RABBITIZER_DEF_INSTR_ID(cpu, sltiu,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),     // Set on Less Than Immediate Unsigned

    // OP rt, IMM(base)
    RABBITIZER_DEF_INSTR_ID(cpu, ldl,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Load Doubleword Left
    RABBITIZER_DEF_INSTR_ID(cpu, ldr,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Load Doubleword Right
    RABBITIZER_DEF_INSTR_ID(cpu, lb,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Byte
    RABBITIZER_DEF_INSTR_ID(cpu, lh,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Halfword
    RABBITIZER_DEF_INSTR_ID(cpu, lwl,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Word Left
    RABBITIZER_DEF_INSTR_ID(cpu, lw,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Word
    RABBITIZER_DEF_INSTR_ID(cpu, lbu,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Byte Insigned
    RABBITIZER_DEF_INSTR_ID(cpu, lhu,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Halfword Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, lwr,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Word Right
    RABBITIZER_DEF_INSTR_ID(cpu, lwu,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Load Word Unsigned
    RABBITIZER_DEF_INSTR_ID(cpu, ll,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),        // Load Linked word
    RABBITIZER_DEF_INSTR_ID(cpu, ld,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),        // Load Doubleword
    RABBITIZER_DEF_INSTR_ID(cpu, lld,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Load Linked Doubleword
    RABBITIZER_DEF_INSTR_ID(cpu, sb,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Byte
    RABBITIZER_DEF_INSTR_ID(cpu, sh,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Halfword
    RABBITIZER_DEF_INSTR_ID(cpu, swl,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Store Word Left
    RABBITIZER_DEF_INSTR_ID(cpu, sw,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Word
    RABBITIZER_DEF_INSTR_ID(cpu, sdl,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Store Doubleword Left
    RABBITIZER_DEF_INSTR_ID(cpu, sdr,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Store Doubleword Right
    RABBITIZER_DEF_INSTR_ID(cpu, swr,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Store Word Right
    RABBITIZER_DEF_INSTR_ID(cpu, sd,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),        // Store Doubleword
    RABBITIZER_DEF_INSTR_ID(cpu, sc,                            .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),        // Store Conditional word
    RABBITIZER_DEF_INSTR_ID(cpu, scd,                           .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_III),       // Store Conditional Doubleword

    RABBITIZER_DEF_INSTR_ID(cpu, pref,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_IV),      // Prefetch
    RABBITIZER_DEF_INSTR_ID(cpu, cache,                         .operands={RABBITIZER_OPERAND_TYPE_op, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .notEmitedByCompilers=true),     // Cache

    // OP ft, IMM(base)
    RABBITIZER_DEF_INSTR_ID(cpu, lwc1,                          .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, ldc1,                          .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, swc1,                          .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, sdc1,                          .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Store Doubleword from Coprocessor z

    // OP cop2t, IMM(base)
    RABBITIZER_DEF_INSTR_ID(cpu, lwc2,                          .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, ldc2,                          .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, swc2,                          .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(cpu, sdc2,                          .operands={RABBITIZER_OPERAND_TYPE_cop2t, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_II),      // Store Doubleword from Coprocessor z

    // OP rt, cop0d
    RABBITIZER_DEF_INSTR_ID(cpu, mfc0,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true, .notEmitedByCompilers=true),      // Move word From CP0
    RABBITIZER_DEF_INSTR_ID(cpu, dmfc0,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true),     // Doubleword Move From CP0
    RABBITIZER_DEF_INSTR_ID(cpu, cfc0,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true),      // Move control word From CP0
    RABBITIZER_DEF_INSTR_ID(cpu, mtc0,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),      // Move word to CP0
    RABBITIZER_DEF_INSTR_ID(cpu, dmtc0,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),     // Doubleword Move To CP0
    RABBITIZER_DEF_INSTR_ID(cpu, ctc0,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Move control word To CP0

    // OP
    RABBITIZER_DEF_INSTR_ID(cpu, tlbr,                          .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),      // Read Indexed TLB Entry
    RABBITIZER_DEF_INSTR_ID(cpu, tlbwi,                         .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),     // Write Indexed TLB Entry
    RABBITIZER_DEF_INSTR_ID(cpu, tlbwr,                         .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),     // Write Random TLB Entry
    RABBITIZER_DEF_INSTR_ID(cpu, tlbp,                          .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),      // Probe TLB for Matching Entry
    RABBITIZER_DEF_INSTR_ID(cpu, eret,                          .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),      // Return from Exception

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(cpu, bc0t,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),      // Branch on FP True
    RABBITIZER_DEF_INSTR_ID(cpu, bc0f,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),      // Branch on FP False
    RABBITIZER_DEF_INSTR_ID(cpu, bc0tl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),     // Branch on FP True Likely
    RABBITIZER_DEF_INSTR_ID(cpu, bc0fl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),     // Branch on FP False Likely

    // OP rt, fs
    RABBITIZER_DEF_INSTR_ID(cpu, mfc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),      // Move Word From Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, dmfc1,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),     // Doubleword Move From Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, mtc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),      // Move Word to Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, dmtc1,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Doubleword Move To Floating-Point

    // OP rt, cop1cs
    RABBITIZER_DEF_INSTR_ID(cpu, cfc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),      // Move Control Word from Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, ctc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),      // Move Control Word to Floating-Point

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(cpu, bc1f,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(cpu, bc1t,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(cpu, bc1fl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),
    RABBITIZER_DEF_INSTR_ID(cpu, bc1tl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),

    // OP fd, fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, add_s, add.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),        // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sub_s, sub.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, mul_s, mul.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, div_s, div.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Divide
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, add_d, add.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sub_d, sub.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, mul_d, mul.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, div_d, div.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Divide

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sqrt_s, sqrt.s,        .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, abs_s, abs.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, mov_s, mov.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, neg_s, neg.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, sqrt_d, sqrt.d,        .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, abs_d, abs.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, mov_d, mov.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, neg_d, neg.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, round_l_s, round.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, trunc_l_s, trunc.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, ceil_l_s, ceil.l.s,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, floor_l_s, floor.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, round_l_d, round.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, trunc_l_d, trunc.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, ceil_l_d, ceil.l.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, floor_l_d, floor.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, round_w_s, round.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, trunc_w_s, trunc.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, ceil_w_s, ceil.w.s,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, floor_w_s, floor.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Floor Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, round_w_d, round.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, trunc_w_d, trunc.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, ceil_w_d, ceil.w.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, floor_w_d, floor.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Floor Convert to Word Fixed-Point

    // OP fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_f_s, c.f.s,          .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_un_s, c.un.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_eq_s, c.eq.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ueq_s, c.ueq.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_olt_s, c.olt.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ult_s, c.ult.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ole_s, c.ole.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ule_s, c.ule.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_f_d, c.f.d,          .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_un_d, c.un.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_eq_d, c.eq.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ueq_d, c.ueq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_olt_d, c.olt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ult_d, c.ult.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ole_d, c.ole.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ule_d, c.ule.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_sf_s, c.sf.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngle_s, c.ngle.s,    .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_seq_s, c.seq.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngl_s, c.ngl.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_lt_s, c.lt.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_nge_s, c.nge.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_le_s, c.le.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngt_s, c.ngt.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_sf_d, c.sf.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngle_d, c.ngle.d,    .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_seq_d, c.seq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngl_d, c.ngl.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_lt_d, c.lt.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_nge_d, c.nge.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_le_d, c.le.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, c_ngt_d, c.ngt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_s_d, cvt.s.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_s_w, cvt.s.w,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_s_l, cvt.s.l,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_d_s, cvt.d.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_d_w, cvt.d.w,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_d_l, cvt.d.l,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_w_s, cvt.w.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_w_d, cvt.w.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_l_s, cvt.l.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, cvt_l_d, cvt.l.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),

    // Pseudo-Instruction Unique IDs
    // OP
    RABBITIZER_DEF_INSTR_ID(cpu, nop,                           .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_R, .isPseudo=true),       // No OPeration

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(cpu, beqz,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true),      // Branch on EQual Zero
    RABBITIZER_DEF_INSTR_ID(cpu, bnez,                          .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true),      // Branch on Not Equal Zero

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(cpu, b,                             .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isBranch=true, .isPseudo=true),         // Branch (unconditional)

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(cpu, move,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .maybeIsMove=true, .isPseudo=true),      // Move
    RABBITIZER_DEF_INSTR_ID(cpu, not,                           .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .isPseudo=true),       // Not

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(cpu, negu,                          .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .isPseudo=true),

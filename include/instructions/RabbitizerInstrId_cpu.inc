    RABBITIZER_DEF_INSTR_ID(CPU_ID, INVALID, .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),

    // OP rs
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mthi, .operands={RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R), // Move To HI register
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mtlo, .operands={RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R),      // Move To LO register
    RABBITIZER_DEF_INSTR_ID(CPU_ID, jr,   .operands={RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true),                        // Jump Register
    RABBITIZER_DEF_INSTR_ID(CPU_ID, jalr, .operands={RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true),      // Jump And Link Register

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, jalr_rd, jalr, .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true),   // Jump And Link Register // Special case for rd != 31

    // OP rd
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mfhi,        .operands={RABBITIZER_REGISTER_TYPE_rd}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Move From HI register
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mflo,        .operands={RABBITIZER_REGISTER_TYPE_rd}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Move From LO register

    // OP rs, rt
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mult,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),      // MULTtiply word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, multu,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),     // MULTtiply Unsigned word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmult,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),     // Doubleword MULTiply
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmultu,      .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),    // Doubleword MULTiply Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tge,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),       // Trap if Greater or Equal
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tgeu,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),      // Trap if Greater or Equal Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlt,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),       // Trap if Less Than
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tltu,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),      // Trap if Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, teq,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),       // Trap if EQual
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tne,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .isTrap=true),       // Trap if Not Equal

    // OP rd, rs, rt
    RABBITIZER_DEF_INSTR_ID(CPU_ID, movz,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // MOVe conditional on Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, movn,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // MOVe conditional on Not zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, div,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),       // DIVide word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, divu,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),      // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sn64_div, div,    .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),  // DIVide word
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sn64_divu, divu,   .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R), // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID(CPU_ID, ddiv,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),      // Doubleword DIVide
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ddivu,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),     // Doubleword DIVide Unsigned
    // RABBITIZER_DEF_INSTR_ID(CPU_ID, ddiv,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),
    // RABBITIZER_DEF_INSTR_ID(CPU_ID, ddivu,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, add,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // ADD word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, addu,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sub,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Subtract word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, subu,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // SUBtract Unsigned word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, and,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // AND
    RABBITIZER_DEF_INSTR_ID(CPU_ID, or,          .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),        // OR
    RABBITIZER_DEF_INSTR_ID(CPU_ID, xor,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // eXclusive OR
    RABBITIZER_DEF_INSTR_ID(CPU_ID, nor,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Not OR
    RABBITIZER_DEF_INSTR_ID(CPU_ID, slt,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Set on Less Than
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sltu,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Set on Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dadd,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Doubleword Add
    RABBITIZER_DEF_INSTR_ID(CPU_ID, daddu,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),     // Doubleword Add Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsub,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Doubleword SUBtract
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsubu,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),     // Doubleword SUBtract Unsigned

    // OP code
    RABBITIZER_DEF_INSTR_ID(CPU_ID, syscall,     .operands={RABBITIZER_REGISTER_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R),   // SYStem CALL
    RABBITIZER_DEF_INSTR_ID(CPU_ID, break,       .operands={RABBITIZER_REGISTER_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R),     // Break
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sync,        .operands={RABBITIZER_REGISTER_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R),      // Sync

    // OP rd, rt, rs
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsllv,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),     // Doubleword Shift Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsrlv,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),     // Doubleword Shift Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsrav,       .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),     // Doubleword Shift Right Arithmetic Variable
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sllv,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(CPU_ID, srlv,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(CPU_ID, srav,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Right Arithmetic Variable

    // OP rd, rt, sa
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sll,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Left Logical
    RABBITIZER_DEF_INSTR_ID(CPU_ID, srl,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Right Logical
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sra,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsll,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Doubleword Shift Left Logical
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsrl,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Doubleword Shift Right Logical
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsra,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Doubleword Shift Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsll32,      .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),    // Doubleword Shift Left Logical plus 32
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsrl32,      .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),    // Doubleword Shift Right Logical plus 32
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dsra32,      .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),    // Doubleword Shift Right Arithmetic plus 32

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bltz,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Less Than Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgez,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Greater than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bltzl,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),     // Branch on Less Than Zero Likely
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgezl,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),     // Branch on Greater than or Equal to Zero Likely
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tgei,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tgeiu,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlti,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tltiu,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bltzal,      .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgezal,      .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bltzall,     .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgezall,     .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, teqi,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tnei,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isTrap=true),

    // OP LABEL
    RABBITIZER_DEF_INSTR_ID(CPU_ID, j,           .operands={RABBITIZER_REGISTER_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true),         // Jump
    RABBITIZER_DEF_INSTR_ID(CPU_ID, jal,         .operands={RABBITIZER_REGISTER_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true),       // Jump And Link

    // OP rs, rt, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, beq,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),       // Branch on EQual
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bne,         .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),       // Branch on Not Equal
    RABBITIZER_DEF_INSTR_ID(CPU_ID, beql,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),      // Branch on EQual Likely
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bnel,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),      // Branch on Not Equal Likely

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, blez,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Less than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgtz,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Greater Than Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, blezl,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),     // Branch on Less than or Equal to Zero Likely
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bgtzl,       .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .isBranchLikely=true),     // Branch on Greater Than Zero Likely

    // OP rt, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lui,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),       // Load Upper Immediate

    // OP rt, rs, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, andi,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),      // And Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ori,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),       // Or Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, xori,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),      // eXclusive OR Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, addi,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),      // Add Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, addiu,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),     // Add Immediate Unsigned Word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, daddi,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),     // Doubleword add Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, daddiu,      .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),    // Doubleword add Immediate Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, slti,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),      // Set on Less Than Immediate
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sltiu,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),     // Set on Less Than Immediate Unsigned

    // OP rt, IMM(base)
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ldl,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Doubleword Left
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ldr,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Doubleword Right
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lb,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Load Byte
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lh,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Load Halfword
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lwl,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Word Left
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lw,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Load Word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lbu,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Byte Insigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lhu,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Halfword Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lwr,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Word Right
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lwu,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Word Unsigned
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sb,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),        // Store Byte
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sh,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),        // Store Halfword
    RABBITIZER_DEF_INSTR_ID(CPU_ID, swl,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),       // Store Word Left
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sw,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),        // Store Word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sdl,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),       // Store Doubleword Left
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sdr,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),       // Store Doubleword Right
    RABBITIZER_DEF_INSTR_ID(CPU_ID, swr,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),       // Store Word Right
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ll,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Load Linked word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, pref,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),      // Prefetch
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lld,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Load Linked Doubleword
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ld,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Load Doubleword
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sc,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Store Conditional word
    RABBITIZER_DEF_INSTR_ID(CPU_ID, scd,         .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),       // Store Conditional Doubleword
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sd,          .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),        // Store Doubleword

    RABBITIZER_DEF_INSTR_ID(CPU_ID, cache,       .operands={RABBITIZER_REGISTER_TYPE_op, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),     // Cache

    // OP ft, IMM(base)
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lwc1,        .operands={RABBITIZER_REGISTER_TYPE_ft, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true),      // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ldc1,        .operands={RABBITIZER_REGISTER_TYPE_ft, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true),      // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, swc1,        .operands={RABBITIZER_REGISTER_TYPE_ft, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true),      // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sdc1,        .operands={RABBITIZER_REGISTER_TYPE_ft, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .isDouble=true),      // Store Doubleword from Coprocessor z

    // OP cop2t, IMM(base)
    RABBITIZER_DEF_INSTR_ID(CPU_ID, lwc2,        .operands={RABBITIZER_REGISTER_TYPE_cop2t, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),      // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ldc2,        .operands={RABBITIZER_REGISTER_TYPE_cop2t, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),      // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, swc2,        .operands={RABBITIZER_REGISTER_TYPE_cop2t, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),      // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(CPU_ID, sdc2,        .operands={RABBITIZER_REGISTER_TYPE_cop2t, RABBITIZER_REGISTER_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I),      // Store Doubleword from Coprocessor z

    // OP rt, cop0d
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mfc0,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true),      // Move word From CP0
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmfc0,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true),     // Doubleword Move From CP0
    RABBITIZER_DEF_INSTR_ID(CPU_ID, cfc0,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true),      // Move control word From CP0
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mtc0,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Move word to CP0
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmtc0,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),     // Doubleword Move To CP0
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ctc0,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Move control word To CP0

    // OP
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlbr,        .operands={}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Read Indexed TLB Entry
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlbwi,       .operands={}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),     // Write Indexed TLB Entry
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlbwr,       .operands={}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),     // Write Random TLB Entry
    RABBITIZER_DEF_INSTR_ID(CPU_ID, tlbp,        .operands={}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Probe TLB for Matching Entry
    RABBITIZER_DEF_INSTR_ID(CPU_ID, eret,        .operands={}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN),      // Return from Exception

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc0t,        .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),      // Branch on FP True
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc0f,        .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),      // Branch on FP False
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc0tl,       .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),     // Branch on FP True Likely
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc0fl,       .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),     // Branch on FP False Likely

    // OP rt, fs
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mfc1,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),      // Move Word From Floating-Point
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmfc1,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),     // Doubleword Move From Floating-Point
    RABBITIZER_DEF_INSTR_ID(CPU_ID, cfc1,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true),      // Move Control Word from Floating-Point
    RABBITIZER_DEF_INSTR_ID(CPU_ID, mtc1,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),      // Move Word to Floating-Point
    RABBITIZER_DEF_INSTR_ID(CPU_ID, dmtc1,       .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Doubleword Move To Floating-Point
    RABBITIZER_DEF_INSTR_ID(CPU_ID, ctc1,        .operands={RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),      // Move Control Word to Floating-Point

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc1f,        .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc1t,        .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc1fl,       .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bc1tl,       .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true),

    // OP fd, fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, add_s, add.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),        // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sub_s, sub.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, mul_s, mul.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, div_s, div.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Divide
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, add_d, add.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sub_d, sub.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, mul_d, mul.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, div_d, div.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Divide

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sqrt_s, sqrt.s,      .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, abs_s, abs.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, mov_s, mov.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, neg_s, neg.s,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, sqrt_d, sqrt.d,      .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, abs_d, abs.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, mov_d, mov.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, neg_d, neg.d,       .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, round_l_s, round.l.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, trunc_l_s, trunc.l.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, ceil_l_s, ceil.l.s,    .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, floor_l_s, floor.l.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, round_l_d, round.l.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, trunc_l_d, trunc.l.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, ceil_l_d, ceil.l.d,    .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, floor_l_d, floor.l.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, round_w_s, round.w.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, trunc_w_s, trunc.w.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, ceil_w_s, ceil.w.s,    .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, floor_w_s, floor.w.s,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true), // Floating-Point Floor Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, round_w_d, round.w.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, trunc_w_d, trunc.w.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, ceil_w_d, ceil.w.d,    .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, floor_w_d, floor.w.d,   .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true), // Floating-Point Floor Convert to Word Fixed-Point

    // OP fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_f_s, c.f.s,       .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_un_s, c.un.s,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_eq_s, c.eq.s,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ueq_s, c.ueq.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_olt_s, c.olt.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ult_s, c.ult.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ole_s, c.ole.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ule_s, c.ule.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_f_d, c.f.d,       .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_un_d, c.un.d,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_eq_d, c.eq.d,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ueq_d, c.ueq.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_olt_d, c.olt.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ult_d, c.ult.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ole_d, c.ole.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ule_d, c.ule.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_sf_s, c.sf.s,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngle_s, c.ngle.s,    .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_seq_s, c.seq.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngl_s, c.ngl.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_lt_s, c.lt.s,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_nge_s, c.nge.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_le_s, c.le.s,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngt_s, c.ngt.s,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_sf_d, c.sf.d,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngle_d, c.ngle.d,    .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_seq_d, c.seq.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngl_d, c.ngl.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_lt_d, c.lt.d,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_nge_d, c.nge.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_le_d, c.le.d,      .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, c_ngt_d, c.ngt.d,     .operands={RABBITIZER_REGISTER_TYPE_fs, RABBITIZER_REGISTER_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_s_d, cvt.s.d,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_s_w, cvt.s.w,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_s_l, cvt.s.l,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_d_s, cvt.d.s,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_d_w, cvt.d.w,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_d_l, cvt.d.l,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_w_s, cvt.w.s,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_w_d, cvt.w.d,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_l_s, cvt.l.s,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true),
    RABBITIZER_DEF_INSTR_ID_ALTNAME(CPU_ID, cvt_l_d, cvt.l.d,     .operands={RABBITIZER_REGISTER_TYPE_fd, RABBITIZER_REGISTER_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true),

    // Pseudo-Instruction Unique IDs
    // OP
    RABBITIZER_DEF_INSTR_ID(CPU_ID, nop,         .operands={}, .instrType=RABBITIZER_INSTR_TYPE_R),       // No OPeration

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, beqz,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on EQual Zero
    RABBITIZER_DEF_INSTR_ID(CPU_ID, bnez,        .operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Not Equal Zero

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(CPU_ID, b,           .operands={RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),         // Branch (unconditional)

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(CPU_ID, move,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Move
    RABBITIZER_DEF_INSTR_ID(CPU_ID, not,         .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Not

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(CPU_ID, negu,        .operands={RABBITIZER_REGISTER_TYPE_rd, RABBITIZER_REGISTER_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),



    // TODO: add RSP instructions

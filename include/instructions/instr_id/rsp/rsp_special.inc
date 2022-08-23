/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP rd, rt, sa
RABBITIZER_DEF_INSTR_ID(rsp, 0x00, sll,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Shift word Left Logical
RABBITIZER_DEF_INSTR_ID(rsp, 0x02, srl,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Shift word Right Logical
RABBITIZER_DEF_INSTR_ID(rsp, 0x03, sra,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Shift word Right Arithmetic

// OP rd, rt, rs
RABBITIZER_DEF_INSTR_ID(rsp, 0x04, sllv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // Shift word Left Logical Variable
RABBITIZER_DEF_INSTR_ID(rsp, 0x06, srlv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // Shift word Right Logical Variable
RABBITIZER_DEF_INSTR_ID(rsp, 0x07, srav,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // Shift word Right Arithmetic Variable

// OP rs
RABBITIZER_DEF_INSTR_ID(rsp, 0x08, jr,                     .operands={RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true)                        // Jump Register
RABBITIZER_DEF_INSTR_ID(rsp, 0x09, jalr,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true)      // Jump And Link Register

// OP rd, rs
RABBITIZER_DEF_INSTR_ID_ALTNAME(rsp, -0x09, jalr_rd, jalr,  .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true)   // Jump And Link Register // Special case for rd != 31

// OP rd, rs, rt
RABBITIZER_DEF_INSTR_ID(rsp, 0x0A, movz,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // MOVe conditional on Zero
RABBITIZER_DEF_INSTR_ID(rsp, 0x0B, movn,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // MOVe conditional on Not zero

RABBITIZER_DEF_INSTR_ID(rsp, 0x20, add,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .notEmitedByCompilers=true)       // ADD word
RABBITIZER_DEF_INSTR_ID(rsp, 0x21, addu,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // ADD Unsigned word
RABBITIZER_DEF_INSTR_ID(rsp, 0x22, sub,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Subtract word
RABBITIZER_DEF_INSTR_ID(rsp, 0x23, subu,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // SUBtract Unsigned word
RABBITIZER_DEF_INSTR_ID(rsp, 0x24, and,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // AND
RABBITIZER_DEF_INSTR_ID(rsp, 0x25, or,                     .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)        // OR
RABBITIZER_DEF_INSTR_ID(rsp, 0x26, xor,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // eXclusive OR
RABBITIZER_DEF_INSTR_ID(rsp, 0x27, nor,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Not OR
RABBITIZER_DEF_INSTR_ID(rsp, 0x2A, slt,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Set on Less Than
RABBITIZER_DEF_INSTR_ID(rsp, 0x2B, sltu,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // Set on Less Than Unsigned

// OP code
RABBITIZER_DEF_INSTR_ID(rsp, 0x0D, break,                  .operands={RABBITIZER_OPERAND_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R)     // Break


// Pseudo-Instruction Unique IDs
// OP
RABBITIZER_DEF_INSTR_ID(rsp, -0x01, nop,                    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_R)       // No OPeration

// OP rd, rs
RABBITIZER_DEF_INSTR_ID(rsp, -0x25, move,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)      // Move
RABBITIZER_DEF_INSTR_ID(rsp, -0x27, not,                    .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)       // Not

// OP rd, rt
RABBITIZER_DEF_INSTR_ID(rsp, -0x23, negu,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_rd, RABBITIZER_OPERAND_TYPE_RSP_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true)

/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // RSP instructions
    RABBITIZER_DEF_INSTR_ID(rsp, INVALID,               .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),

    // OP vd, vs, vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(rsp, vmulf,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmulu,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrndp,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmulq,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmudl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmudm,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmudn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmudh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmacf,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmacu,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrndn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmacq,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmadl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmadm,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmadn,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmadh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vadd,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vsub,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vabs,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vaddc,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vsubc,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vsar,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vand,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vnand,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vor,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vnor,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vxor,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vnxor,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vlt,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, veq,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vne,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vge,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vcl,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vch,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vcr,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmrg,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),

    // OP vd[vs], vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(rsp, vrcp,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrcpl,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}), // operands may be wrong
    RABBITIZER_DEF_INSTR_ID(rsp, vrcph,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vmov,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrsq,                  .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrsql,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),
    RABBITIZER_DEF_INSTR_ID(rsp, vrsqh,                 .operands={RABBITIZER_OPERAND_TYPE_RSP_vd_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh}),

    // OP
    RABBITIZER_DEF_INSTR_ID(rsp, vnop,                  .operands={0}),

    // OP rt, vd[index]
    RABBITIZER_DEF_INSTR_ID(rsp, mfc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_vd_index}, .modifiesRt=true),
    RABBITIZER_DEF_INSTR_ID(rsp, mtc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_RSP_vd_index}),

    // OP rt, rd
    RABBITIZER_DEF_INSTR_ID(rsp, cfc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rd}, .modifiesRt=true),
    RABBITIZER_DEF_INSTR_ID(rsp, ctc2,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rd}),

    // OP vt[elementlow], offset(vs)
    RABBITIZER_DEF_INSTR_ID(rsp, sbv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, ssv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, slv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, sdv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, sqv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, srv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, spv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, suv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, swv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, shv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, sfv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, stv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lbv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lsv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, llv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, ldv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lqv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lrv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lpv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, luv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lhv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, lfv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),
    RABBITIZER_DEF_INSTR_ID(rsp, ltv,                   .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs}),


    // CPU instructions but with rsp registers

    // OP rs
    RABBITIZER_DEF_INSTR_ID(rsp, jr,                    .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true),                        // Jump Register
    RABBITIZER_DEF_INSTR_ID(rsp, jalr,                  .operands={RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true),      // Jump And Link Register

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(rsp, jalr_rd, jalr, .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .isJump=true, .modifiesRd=true, .doesLink=true),   // Jump And Link Register // Special case for rd != 31

    // OP rd, rs, rt
    RABBITIZER_DEF_INSTR_ID(rsp, movz,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // MOVe conditional on Zero
    RABBITIZER_DEF_INSTR_ID(rsp, movn,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // MOVe conditional on Not zero
    RABBITIZER_DEF_INSTR_ID(rsp, add,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true, .notEmitedByCompilers=true),       // ADD word
    RABBITIZER_DEF_INSTR_ID(rsp, addu,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(rsp, sub,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Subtract word
    RABBITIZER_DEF_INSTR_ID(rsp, subu,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // SUBtract Unsigned word
    RABBITIZER_DEF_INSTR_ID(rsp, and,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // AND
    RABBITIZER_DEF_INSTR_ID(rsp, or,                    .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),        // OR
    RABBITIZER_DEF_INSTR_ID(rsp, xor,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // eXclusive OR
    RABBITIZER_DEF_INSTR_ID(rsp, nor,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Not OR
    RABBITIZER_DEF_INSTR_ID(rsp, slt,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Set on Less Than
    RABBITIZER_DEF_INSTR_ID(rsp, sltu,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Set on Less Than Unsigned

    // OP code
    RABBITIZER_DEF_INSTR_ID(rsp, break,                 .operands={RABBITIZER_OPERAND_TYPE_code}, .instrType=RABBITIZER_INSTR_TYPE_R),     // Break

    // OP rd, rt, rs
    RABBITIZER_DEF_INSTR_ID(rsp, sllv,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(rsp, srlv,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(rsp, srav,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Shift word Right Arithmetic Variable

    // OP rd, rt, sa
    RABBITIZER_DEF_INSTR_ID(rsp, sll,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Left Logical
    RABBITIZER_DEF_INSTR_ID(rsp, srl,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Right Logical
    RABBITIZER_DEF_INSTR_ID(rsp, sra,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_sa}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Shift word Right Arithmetic

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, bltz,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Less Than Zero
    RABBITIZER_DEF_INSTR_ID(rsp, bgez,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Greater than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(rsp, bltzal,                .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true), // Branch on Less Than Zero and Link
    RABBITIZER_DEF_INSTR_ID(rsp, bgezal,                .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true, .doesLink=true), // Branch on Greater Than or Equal to Zero and Link

    // OP LABEL
    RABBITIZER_DEF_INSTR_ID(rsp, j,                     .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true),         // Jump
    RABBITIZER_DEF_INSTR_ID(rsp, jal,                   .operands={RABBITIZER_OPERAND_TYPE_LABEL}, .instrType=RABBITIZER_INSTR_TYPE_J, .isJump=true, .doesLink=true),       // Jump And Link

    // OP rs, rt, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, beq,                   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),       // Branch on EQual
    RABBITIZER_DEF_INSTR_ID(rsp, bne,                   .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),       // Branch on Not Equal

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, blez,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Less than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(rsp, bgtz,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Greater Than Zero

    // OP rt, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, lui,                   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeHi=true),       // Load Upper Immediate

    // OP rt, rs, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, andi,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),      // And Immediate
    RABBITIZER_DEF_INSTR_ID(rsp, ori,                   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true, .canBeLo=true),       // Or Immediate
    RABBITIZER_DEF_INSTR_ID(rsp, xori,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .isUnsigned=true, .modifiesRt=true),      // eXclusive OR Immediate
    RABBITIZER_DEF_INSTR_ID(rsp, addi,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .notEmitedByCompilers=true, .canBeLo=true),      // Add Immediate
    RABBITIZER_DEF_INSTR_ID(rsp, addiu,                 .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true),     // Add Immediate Unsigned Word
    RABBITIZER_DEF_INSTR_ID(rsp, slti,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),      // Set on Less Than Immediate
    RABBITIZER_DEF_INSTR_ID(rsp, sltiu,                 .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true),     // Set on Less Than Immediate Unsigned

    // OP rt, IMM(base)
    RABBITIZER_DEF_INSTR_ID(rsp, lb,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Byte
    RABBITIZER_DEF_INSTR_ID(rsp, lh,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Halfword
    RABBITIZER_DEF_INSTR_ID(rsp, lw,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Load Word
    RABBITIZER_DEF_INSTR_ID(rsp, lbu,                   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Byte Insigned
    RABBITIZER_DEF_INSTR_ID(rsp, lhu,                   .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),       // Load Halfword Unsigned
    RABBITIZER_DEF_INSTR_ID(rsp, sb,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Byte
    RABBITIZER_DEF_INSTR_ID(rsp, sh,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Halfword
    RABBITIZER_DEF_INSTR_ID(rsp, sw,                    .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),        // Store Word

    RABBITIZER_DEF_INSTR_ID(rsp, pref,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .modifiesRt=true, .canBeLo=true, .doesDereference=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_IV),      // Prefetch
    RABBITIZER_DEF_INSTR_ID(rsp, cache,                 .operands={RABBITIZER_OPERAND_TYPE_op, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .notEmitedByCompilers=true),     // Cache

    // OP ft, IMM(base)
    RABBITIZER_DEF_INSTR_ID(rsp, lwc1,                  .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesLoad=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(rsp, swc1,                  .operands={RABBITIZER_OPERAND_TYPE_ft, RABBITIZER_OPERAND_TYPE_IMM_base}, .instrType=RABBITIZER_INSTR_TYPE_I, .isFloat=true, .canBeLo=true, .doesDereference=true, .doesStore=true, .architectureVersion=RABBITIZER_ARCHVERSION_MIPS_I),      // Store Word from Coprocessor z

    // OP rt, cop0d
    RABBITIZER_DEF_INSTR_ID(rsp, mfc0,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRt=true, .notEmitedByCompilers=true),      // Move word From CP0
    RABBITIZER_DEF_INSTR_ID(rsp, mtc0,                  .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop0d}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .notEmitedByCompilers=true),      // Move word to CP0

    // Pseudo-Instruction Unique IDs
    // OP
    RABBITIZER_DEF_INSTR_ID(rsp, nop,                   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_R),       // No OPeration

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(rsp, beqz,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on EQual Zero
    RABBITIZER_DEF_INSTR_ID(rsp, bnez,                  .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),      // Branch on Not Equal Zero

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(rsp, b,                     .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_REGIMM, .isBranch=true),         // Branch (unconditional)

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(rsp, move,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),      // Move
    RABBITIZER_DEF_INSTR_ID(rsp, not,                   .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),       // Not

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(rsp, negu,                  .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_R, .modifiesRd=true),

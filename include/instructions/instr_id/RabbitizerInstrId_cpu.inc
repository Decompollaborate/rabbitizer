/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(cpu, , INVALID,                       .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

#include "instructions/instr_id/cpu/cpu_normal.inc"
#include "instructions/instr_id/cpu/cpu_special.inc"
#include "instructions/instr_id/cpu/cpu_regimm.inc"
#include "instructions/instr_id/cpu/cpu_cop0.inc"
#include "instructions/instr_id/cpu/cpu_cop0_bc0.inc"
#include "instructions/instr_id/cpu/cpu_cop0_tlb.inc"

    // OP rt, fs
    RABBITIZER_DEF_INSTR_ID(cpu, , mfc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)      // Move Word From Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, , dmfc1,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)     // Doubleword Move From Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, , mtc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)      // Move Word to Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, , dmtc1,                         .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Doubleword Move To Floating-Point

    // OP rt, cop1cs
    RABBITIZER_DEF_INSTR_ID(cpu, , cfc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .modifiesRt=true)      // Move Control Word from Floating-Point
    RABBITIZER_DEF_INSTR_ID(cpu, , ctc1,                          .operands={RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_cop1cs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)      // Move Control Word to Floating-Point

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(cpu, , bc1f,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true)
    RABBITIZER_DEF_INSTR_ID(cpu, , bc1t,                          .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true)
    RABBITIZER_DEF_INSTR_ID(cpu, , bc1fl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true)
    RABBITIZER_DEF_INSTR_ID(cpu, , bc1tl,                         .operands={RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isBranch=true, .isBranchLikely=true)

    // OP fd, fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , add_s, add.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)        // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , sub_s, sub.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , mul_s, mul.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , div_s, div.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Divide
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , add_d, add.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Add
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , sub_d, sub.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Sub
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , mul_d, mul.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Multiply
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , div_d, div.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Divide

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , sqrt_s, sqrt.s,        .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , abs_s, abs.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , mov_s, mov.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , neg_s, neg.s,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , sqrt_d, sqrt.d,        .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)    // Floating-Point Square Root
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , abs_d, abs.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Absolute Value
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , mov_d, mov.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Move
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , neg_d, neg.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)     // Floating-Point Negate
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , round_l_s, round.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , trunc_l_s, trunc.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , ceil_l_s, ceil.l.s,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , floor_l_s, floor.l.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , round_l_d, round.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Round to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , trunc_l_d, trunc.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , ceil_l_d, ceil.l.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , floor_l_d, floor.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , round_w_s, round.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , trunc_w_s, trunc.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , ceil_w_s, ceil.w.s,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , floor_w_s, floor.w.s,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Floor Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , round_w_d, round.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Round to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , trunc_w_d, trunc.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , ceil_w_d, ceil.w.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , floor_w_d, floor.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true) // Floating-Point Floor Convert to Word Fixed-Point

    // OP fs, ft
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_f_s, c.f.s,          .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_un_s, c.un.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_eq_s, c.eq.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ueq_s, c.ueq.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_olt_s, c.olt.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ult_s, c.ult.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ole_s, c.ole.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ule_s, c.ule.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_f_d, c.f.d,          .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_un_d, c.un.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_eq_d, c.eq.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ueq_d, c.ueq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_olt_d, c.olt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ult_d, c.ult.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ole_d, c.ole.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ule_d, c.ule.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_sf_s, c.sf.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngle_s, c.ngle.s,    .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_seq_s, c.seq.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngl_s, c.ngl.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_lt_s, c.lt.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_nge_s, c.nge.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_le_s, c.le.s,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngt_s, c.ngt.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_sf_d, c.sf.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngle_d, c.ngle.d,    .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_seq_d, c.seq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngl_d, c.ngl.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_lt_d, c.lt.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_nge_d, c.nge.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_le_d, c.le.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , c_ngt_d, c.ngt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_s_d, cvt.s.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_s_w, cvt.s.w,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_s_l, cvt.s.l,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_d_s, cvt.d.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_d_w, cvt.d.w,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_d_l, cvt.d.l,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_w_s, cvt.w.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_w_d, cvt.w.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_l_s, cvt.l.s,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
    RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, , cvt_l_d, cvt.l.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)

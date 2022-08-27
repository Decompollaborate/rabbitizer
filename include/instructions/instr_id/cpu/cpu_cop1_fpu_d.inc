/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP fd, fs, ft
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x00, add_d, add.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)        // Floating-Point Add
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x01, sub_d, sub.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Sub
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x02, mul_d, mul.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Multiply
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x03, div_d, div.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Divide

// OP fd, fs
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x04, sqrt_d, sqrt.d,        .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)    // Floating-Point Square Root
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x05, abs_d, abs.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Absolute Value
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x06, mov_d, mov.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Move
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x07, neg_d, neg.d,          .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)     // Floating-Point Negate

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x08, round_l_d, round.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Round to Long Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x09, trunc_l_d, trunc.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Truncate to Long Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0A, ceil_l_d, ceil.l.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)  // Floating-Point Ceiling Convert to Long Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0B, floor_l_d, floor.l.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Floor Convert to Long Fixed-Point

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0C, round_w_d, round.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Round to Word Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0D, trunc_w_d, trunc.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Truncate to Word Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0E, ceil_w_d, ceil.w.d,    .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)  // Floating-Point Ceiling Convert to Word Fixed-Point
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x0F, floor_w_d, floor.w.d,  .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // Floating-Point Floor Convert to Word Fixed-Point

// OP fd, fs
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x20, cvt_s_d, cvt.s.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x24, cvt_w_d, cvt.w.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x25, cvt_l_d, cvt.l.d,      .operands={RABBITIZER_OPERAND_TYPE_fd, RABBITIZER_OPERAND_TYPE_fs}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true, .isDouble=true)

// OP fs, ft
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x30, c_f_d, c.f.d,          .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x31, c_un_d, c.un.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x32, c_eq_d, c.eq.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x33, c_ueq_d, c.ueq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x34, c_olt_d, c.olt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x35, c_ult_d, c.ult.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x36, c_ole_d, c.ole.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x37, c_ule_d, c.ule.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x38, c_df_d, c.df.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x39, c_ngle_d, c.ngle.d,    .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3A, c_deq_d, c.deq.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3B, c_ngl_d, c.ngl.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3C, c_lt_d, c.lt.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3D, c_nge_d, c.nge.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3E, c_le_d, c.le.d,        .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x3F, c_ngt_d, c.ngt.d,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

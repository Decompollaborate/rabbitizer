/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = MMI     |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | MADD  | MADDU |  ---  |  ---  | PLZCW |  ---  |  ---  |  ---  |
001 |  *1   |  *2   |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
010 | MFHI1 | MTHI1 | MFLO1 | MTLO1 |  ---  |  ---  |  ---  |  ---  |
011 | MULT1 | MULTU1| DIV1  | DIVU1 |  ---  |  ---  |  ---  |  ---  |
100 | MADD1 | MADDU1|  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
101 |  *3   |  *4   |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
110 | PMFHL | PMTHL |  ---  |  ---  | PSLLH |  ---  | PSRLH | PSRAH |
111 |  ---  |  ---  |  ---  |  ---  | PSLLW |  ---  | PSRLW | PSRAW |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = MMI0 list                    *2 = MMI2 list
     *3 = MMI1 list                    *4 = MMI3 list
*/

// TODO: operands and properties
RABBITIZER_DEF_INSTR_ID(r5900, 0x00, madd,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x01, maddu,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x04, plzcw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x10, mfhi1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x11, mthi1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x12, mflo1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, mtlo1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x18, mult1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x19, multu1,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, div1,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, divu1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x20, madd1,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x21, maddu1,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x30, pmfhl,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x31, pmthl,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x34, psllh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x36, psrlh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x37, psrah,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x3C, psllw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3E, psrlw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3F, psraw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

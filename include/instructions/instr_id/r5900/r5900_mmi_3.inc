/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    |           |                               |function  |  MMI3  |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 |PMADDUW|  ---  |  ---  |PSRAVW |
001 |  ---  |  ---  |  ---  |  ---  |
010 |PMTHI  | PMTLO |PINTEH |  ---  |
011 |PMULTUW| PDIVUW|PCPYUD |  ---  |
100 |  ---  |  ---  |  POR  | PNOR  |
101 |  ---  |  ---  |  ---  |  ---  |
110 |  ---  |  ---  | PEXCH | PCPYH |
111 |  ---  |  ---  | PEXCW |  ---  |
 hi |-------|-------|-------|-------|
*/

// TODO: operands and properties
RABBITIZER_DEF_INSTR_ID(r5900, 0x00, pmadduw,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x03, psravw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x08, pmthi,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x09, pmtlo,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0A, pinteh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x0C, pmultuw,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0D, pdivuw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0E, pcpyud,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x12, por,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, pnor,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, pexch,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, pcpyh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x1E, pexcw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

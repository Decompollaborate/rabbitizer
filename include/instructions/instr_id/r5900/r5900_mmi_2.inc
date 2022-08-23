/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    |---00--|---01--|---10--|---11--| lo
000 |PMADDW |  ---  |PSLLVW |PSRLVW |
001 |PMSUBW |  ---  |  ---  |  ---  |
010 |PMFHI  |PMFLO  |PINTH  |  ---  |
011 |PMULTW |PDIVW  |PCPYLD |  ---  |
100 |PMADDH |PHMADH | PAND  |  PXOR |
101 |PMSUBH |PHMSBH |  ---  |  ---  |
110 | ---   |  ---  | PEXEH | PREVH |
111 |PMULTH |PDIVBW | PEXEW |PROT3W |
 hi |-------|-------|-------|-------|
*/

// TODO: operands and properties
RABBITIZER_DEF_INSTR_ID(r5900, 0x00, pmaddw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x02, psllvw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x03, psrlvw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x04, pmsubw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x08, pmfhi,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x09, pmflo,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0A, pinth,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x0C, pmultw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0D, pdivw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0E, pcpyld,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x10, pmaddh,    .operands={RABBITIZER_OPERAND_TYPE_rd, RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .modifiesRd=true, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x11, phmadh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x12, pand,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, pxor,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x14, pmsubh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x15, phmsbh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, pexeh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, prevh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x1C, pmulth,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1D, pdivbw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1E, pexew,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1F, prot3w,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------10--------6-5--------0
    |          |                              |function  |   MMI0   |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 |PADDW  | PSUBW | PCGTW | PMAXW |
001 |PADDH  | PSUBH | PCGTH | PMAXH |
010 |PADDB  | PSUBB | PCGTB |  ---  |
011 | ---   | ---   |  ---  |  ---  |
100 |PADDSW |PSUBSW |PEXTLW | PPACW |
101 |PADDSH |PSUBSH |PEXTLH | PPACH |
110 |PADDSB |PSUBSB |PEXTLB | PPACB |
111 | ---   |  ---  | PEXT5 | PPAC5 |
 hi |-------|-------|-------|-------|
*/

// TODO: operands and properties
RABBITIZER_DEF_INSTR_ID(r5900, 0x00, paddw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x01, psubw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x02, pcgtw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x03, pmaxw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x04, paddh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x05, psubh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x06, pcgth,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x07, pmaxh,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x08, paddb,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x09, psubb,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0A, pcgtb,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x10, paddsw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x11, psubsw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x12, pextlw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, ppacw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x14, paddsh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x15, psubsh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x16, pextlh,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x17, ppach,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x18, paddsb,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x19, psubsb,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, pextlb,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, ppacb,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, 0x1E, pext5,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1F, ppac5,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

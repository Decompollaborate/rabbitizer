/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    |---00--|---01--|---10--|---11--| lo
000 |  ---  | PABSW | PCEQW | PMINW |
001 |PADSBH | PABSH | PCEQH | PMINH |
010 |  ---  |  ---  | PCEQB |  ---  |
011 |  ---  |  ---  |  ---  |  ---  |
100 |PADDUW |PSUBUW |PEXTUW |  ---  |
101 |PADDUH |PSUBUH |PEXTUH |  ---  |
110 |PADDUB |PSUBUB |PEXTUB | QFSRV |
111 |  ---  |  ---  |  ---  |  ---  |
 hi |-------|-------|-------|-------|
*/

// TODO: operands and properties
RABBITIZER_DEF_INSTR_ID(r5900, , pabsw,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pceqw,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pminw,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, , padsbh,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pabsh,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pceqh,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pminh,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, , pceqb,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, , padduw,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , psubuw,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pextuw,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, , padduh,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , psubuh,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pextuh,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

RABBITIZER_DEF_INSTR_ID(r5900, , paddub,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , psubub,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , pextub,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)
RABBITIZER_DEF_INSTR_ID(r5900, , qfsrv,       .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

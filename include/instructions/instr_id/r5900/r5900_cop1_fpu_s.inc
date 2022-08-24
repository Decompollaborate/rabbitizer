/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------------5--------0
    |  = COP1   |  = S    |                               | function|
    ------6----------5-----------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ADD.S | SUB.S | MUL.S | DIV.S | SQRT.S| ABS.S | MOV.S | NEG.S |
001 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  | ---   |
010 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |RSQRT.S|  ---  |
011 | ADDA.S| SUBA.S| MULA.S|  ---  | MADD.S| MSUB.S|MADDA.S|MSUBA.S|
100 |  ---  | ---   |  ---  |  ---  | CVT.W |  ---  |  ---  |  ---  |
101 | MAX.S | MIN.S |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
110 | C.F   | ---   | C.EQ  |  ---  | C.LT  |  ---  |  C.LE |  ---  |
111 | ---   | ---   |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

// The other instructions are implemented using the main CPU table

// TODO: operands and properties

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x16, rsqrt_s, rsqrt.s,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x18, adda_s, adda.s,      .operands={RABBITIZER_OPERAND_TYPE_fs, RABBITIZER_OPERAND_TYPE_ft}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true) // floating point ADD to Accumulator
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x19, suba_s, suba.s,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x1A, mula_s, mula.s,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x1C, madd_s, madd.s,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x1D, msub_s, msub.s,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x1E, madda_s, madda.s,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x1F, msuba_s, msuba.s,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x28, max_s, max.s,        .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)
RABBITIZER_DEF_INSTR_ID_ALTNAME(cpu, 0x29, min_s, min.s,        .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .isFloat=true)

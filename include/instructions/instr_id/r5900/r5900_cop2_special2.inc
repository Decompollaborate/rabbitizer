/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-25-----21-20------------------11-10------6-5-2-1--0
    |  =COP2   | Special2 |                        |  fhi  |1111|flo|
    ------6----------5-----------------------------------------------
Note: opcode is flo | (fhi * 4).
     |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
0000 |VADDAx |VADDAy |VADDAz |VADDAw |VSUBAx |VSUBAy |VSUBAz |VSUBAw |
0001 |VMADDAx|VMADDAy|VMADDAz|VMADDAw|VMSUBAx|VMSUBAy|VMSUBAz|VMSUBAw|
0010 |VITOF0 |VITOF4 |VITOF12|VITOF15|VFTOI0 |VFTOI4 |VFTOI12|VFTOI15|
0011 |VMULAx |VMULAy |VMULAz |VMULAw |VMULAq |VABS   |VMULAi |VCLIPw |
0100 |VADDAq |VMADDAq|VADDAi |VMADDAi|VSUBAq |VMSUBAq|VSUBAi |VMSUBAi|
0101 |VADDA  |VMADDA |VMULA  |  ---  |VSUBA  |VMSUBA |VOPMULA|VNOP   |
0110 |VMOVE  |VMR32  |  ---  |  ---  |VLQI   |VSQI   |VLQD   |VSQD   |
0111 |VDIV   |VSQRT  |VRSQRT |VWAITQ |VMTIR  |VMFIR  |VILWR  |VISWR  |
1000 |VRNEXT |VRGET  |VRINIT |VRXOR  |  ---  |  ---  |  ---  |  ---  |
1001 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1010 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1011 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1100 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1101 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1110 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
1111 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
 hi  |-------|-------|-------|-------|-------|-------|-------|-------|
*/

// TODO: operands and properties

RABBITIZER_DEF_INSTR_ID(r5900, 0x00, vaddax,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x01, vadday,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x02, vaddaz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x03, vaddaw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x04, vsubax,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x05, vsubay,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x06, vsubaz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x07, vsubaw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x08, vvmaddx,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x09, vvmaddy,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0A, vvmaddz,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0B, vvmaddw,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0C, vmsubax,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0D, vmsubay,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0E, vmsubaz,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0F, vmsubaw,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x10, vitof0,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x11, vitof4,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x12, vitof12,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, vitof15,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x14, vftoi0,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x15, vftoi4,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x16, vftoi12,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x17, vftoi15,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x18, vmulax,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x19, vmulay,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, vmulaz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, vmulaw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1C, vmulaq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1D, vabs,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1E, vmulai,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1F, vclipw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x20, vaddaq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x21, vmaddaq,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x22, vaddai,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x23, vmaddai,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x24, vsubaq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x25, vmsubaq,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x26, vsubai,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x27, vmsubai,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x28, vadda,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x29, vmadda,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2A, vmula,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2C, vsuba,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2D, vmsuba,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2E, vopmula,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2F, vnop,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x30, vmove,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x31, vmr32,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x34, vlqi,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x35, vsqi,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x36, vlqd,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x37, vsqd,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x38, vdiv,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x39, vsqrt,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3A, vrsqrt,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3B, vwaitq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3C, vmtir,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3D, vmfir,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3E, vilwr,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x3F, viswr,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x40, vrnext,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x41, vrget,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x42, vrinit,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x43, vrxor,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

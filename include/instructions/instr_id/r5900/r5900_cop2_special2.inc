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

// TODO: operands and properties and most instructions

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

RABBITIZER_DEF_INSTR_ID(r5900, 0x3A, vrsqrt,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

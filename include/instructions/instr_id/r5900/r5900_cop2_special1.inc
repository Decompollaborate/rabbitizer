/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-25-----21-20-----------------------------5--------0
    |  =COP2   | Special1 |                                |function|
    ------6----------5------------------------------------------6----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 |VADDx  |VADDy  |VADDz  |VADDw  |VSUBx  |VSUBy  |VSUBz  |VSUBw  |
001 |VMADDx |VMADDy |VMADDz |VMADDw |VMSUBx |VMSUBy |VMSUBz |VMSUBw |
010 |VMAXx  |VMAXy  |VMAXz  |VMAXw  |VMINIx |VMINIy |VMINIz |VMINIw |
011 |VMULx  |VMULy  |VMULz  |VMULw  |VMULq  |VMAXi  |VMULi  |VMINIi |
100 |VADDq  |VMADDq |VADDi  |VMADDi |VSUBq  |VMSUBq |VSUbi  |VMSUBi |
101 |VADD   |VMADD  |VMUL   |VMAX   |VSUB   |VMSUB  |VOPMSUB|VMINI  |
110 |VIADD  |VISUB  |VIADDI |  ---  |VIAND  |VIOR   |  ---  |  ---  |
111 |VCALLMS|CALLMSR|  ---  |  ---  |  *1   |  *1   |  *1   |  *1   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
    *1=see special2 table
*/

// TODO: operands and properties

RABBITIZER_DEF_INSTR_ID(r5900, 0x00, vaddx,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x01, vaddy,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x02, vaddz,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x03, vaddw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x04, vsubx,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x05, vsuby,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x06, vsubz,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x07, vsubw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x08, vmaddx,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x09, vmaddy,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0A, vmaddz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0B, vmaddw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0C, vmsubx,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0D, vmsuby,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0E, vmsubz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x0F, vmsubw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x10, vmaxx,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x11, vmaxy,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x12, vmaxz,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x13, vmaxw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x14, vminix,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x15, vminiy,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x16, vminiz,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x17, vminiw,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x18, vmulx,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x19, vmuly,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1A, vmulz,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1B, vmulw,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1C, vmulq,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1D, vmaxi,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1E, vmuli,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x1F, vminii,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x20, vaddq,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x21, vmaddq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x22, vaddi,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x23, vmaddi,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x24, vsubq,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x25, vmsubq,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x26, vsubi,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x27, vmsubi,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x28, vadd,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x29, vmadd,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2A, vmul,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2B, vmax,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2C, vsub,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2D, vmsub,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2E, vopmsub,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x2F, vmini,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

RABBITIZER_DEF_INSTR_ID(r5900, 0x30, viadd,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x31, visub,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x32, viaddi,    .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x34, viand,     .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x35, vior,      .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x38, vcallms,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)
RABBITIZER_DEF_INSTR_ID(r5900, 0x39, callmsr,   .operands={0}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN)

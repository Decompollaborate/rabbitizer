/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31-------26-25-----------------------------------------5--------0
    |  =COP2   |1|                                         |function|
    ------6-----1-----------------------------------------------6----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 |VADDx  |VADDy  |VADDz  |VADDw  |VSUBx  |VSUBy  |VSUBz  |VSUBw  |
001 |VMADDx |VMADDy |VMADDz |VMADDw |VMSUBx |VMSUBy |VMSUBz |VMSUBw |
010 |VMAXx  |VMAXy  |VMAXz  |VMAXw  |VMINIx |VMINIy |VMINIz |VMINIw |
011 |VMULx  |VMULy  |VMULz  |VMULw  |VMULq  |VMAXi  |VMULi  |VMINIi |
100 |VADDq  |VMADDq |VADDi  |VMADDi |VSUBq  |VMSUBq |VSUbi  |VMSUBi |
101 |VADD   |VMADD  |VMUL   |VMAX   |VSUB   |VMSUB  |VOPMSUB|VMINI  |
110 |VIADD  |VISUB  |VIADDI | ---   |VIAND  |VIOR   | ---   | ---   |
111 |VCALLMS|VCALLMSR| ---  | ---   |  *1   |  *1   |  *1   |  *1   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
    *1=see special2 table
*/

    // TODO: properties

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x00, vaddx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x01, vaddy,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x02, vaddz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, vaddw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x04, vsubx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x05, vsuby,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x06, vsubz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x07, vsubw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x08, vmaddx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x09, vmaddy,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0A, vmaddz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0B, vmaddw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0C, vmsubx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0D, vmsuby,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0E, vmsubz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0F, vmsubw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract broadcast

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x10, vmaxx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x11, vmaxy,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, vmaxz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x13, vmaxw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x14, vminix,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x15, vminiy,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x16, vminiz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x17, vminiw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINimum broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x18, vmulx,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x19, vmuly,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, vmulz,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, vmulw,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C, vmulq,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1D, vmaxi,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1E, vmuli,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1F, vminii,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINImum I

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x20, vaddq,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x21, vmaddq,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x22, vaddi,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x23, vmaddi,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x24, vsubq,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x25, vmsubq,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x26, vsubi,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x27, vmsubi,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x28, vadd,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // ADD
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x29, vmadd,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-ADD
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2A, vmul,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MULtiply
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2B, vmax,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MAXimum
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2C, vsub,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // SUBtract
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2D, vmsub,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply-SUBtract
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2E, vopmsub,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Outer product post decrement
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2F, vmini,
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // MINImum

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x30, viadd,
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer ADD
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x31, visub,
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer SUBtract
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x32, viaddi,
        operands: Operand::arr3(Operand::r5900_vit, Operand::r5900_vis, Operand::r5900_immediate5)
    ) // Integer ADD Immediate
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x34, viand,
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer AND
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x35, vior,
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer OR

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x38, vcallms,
        operands: Operand::arr1(Operand::r5900_immediate15)
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x39, vcallmsr,
        operands: Operand::arr1(Operand::r5900_vis)
    )

/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
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

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x00, vaddx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x01, vaddy, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x02, vaddz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x03, vaddw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x04, vsubx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x05, vsuby, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x06, vsubz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x07, vsubw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x08, vmaddx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x09, vmaddy, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0A, vmaddz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0B, vmaddw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0C, vmsubx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0D, vmsuby, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0E, vmsubz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x0F, vmsubw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x10, vmaxx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x11, vmaxy, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x12, vmaxz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x13, vmaxw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x14, vminix, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x15, vminiy, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x16, vminiz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x17, vminiw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x18, vmulx, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x19, vmuly, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1A, vmulz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1B, vmulw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1C, vmulq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply Q
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1D, vmaxi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1E, vmuli, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x1F, vminii, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINImum I

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x20, vaddq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD Q
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x21, vmaddq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD Q
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x22, vaddi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x23, vmaddi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x24, vsubq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract Q
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x25, vmsubq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract Q
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x26, vsubi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x27, vmsubi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract I
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x28, vadd, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // ADD
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x29, vmadd, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-ADD
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2A, vmul, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MULtiply
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2B, vmax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MAXimum
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2C, vsub, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // SUBtract
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2D, vmsub, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply-SUBtract
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2E, vopmsub, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Outer product post decrement
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x2F, vmini, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vfdxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // MINImum

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x30, viadd, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vid, Operand::r5900ee_vis, Operand::r5900ee_vit)
    ) // Integer ADD
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x31, visub, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vid, Operand::r5900ee_vis, Operand::r5900ee_vit)
    ) // Integer SUBtract
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x32, viaddi, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vit, Operand::r5900ee_vis, Operand::r5900ee_immediate5)
    ) // Integer ADD Immediate
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x34, viand, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vid, Operand::r5900ee_vis, Operand::r5900ee_vit)
    ) // Integer AND
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x35, vior, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_vid, Operand::r5900ee_vis, Operand::r5900ee_vit)
    ) // Integer OR

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x38, vcallms, R5900EE,
        operands: Operand::arr1(Operand::r5900ee_immediate15)
    )
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x39, vcallmsr, R5900EE,
        operands: Operand::arr1(Operand::r5900ee_vis)
    )

    RAB_DEF_OPCODE_REDIRECT(0x3C, r5900ee, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3D, r5900ee, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3E, r5900ee, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3F, r5900ee, coprocessor2_special2)

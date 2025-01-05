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

    RAB_DEF_OPCODE(
        r5900, 0x00, vaddx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x01, vaddy, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x02, vaddz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x03, vaddw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x04, vsubx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x05, vsuby, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x06, vsubz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x07, vsubw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x08, vmaddx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x09, vmaddy, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0A, vmaddz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0B, vmaddw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0C, vmsubx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0D, vmsuby, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0E, vmsubz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast
    RAB_DEF_OPCODE(
        r5900, 0x0F, vmsubw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract broadcast

    RAB_DEF_OPCODE(
        r5900, 0x10, vmaxx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x11, vmaxy, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x12, vmaxz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x13, vmaxw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x14, vminix, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x15, vminiy, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x16, vminiz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x17, vminiw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINimum broadcast
    RAB_DEF_OPCODE(
        r5900, 0x18, vmulx, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE(
        r5900, 0x19, vmuly, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE(
        r5900, 0x1A, vmulz, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE(
        r5900, 0x1B, vmulw, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply broadcast
    RAB_DEF_OPCODE(
        r5900, 0x1C, vmulq, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply Q
    RAB_DEF_OPCODE(
        r5900, 0x1D, vmaxi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum I
    RAB_DEF_OPCODE(
        r5900, 0x1E, vmuli, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply I
    RAB_DEF_OPCODE(
        r5900, 0x1F, vminii, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINImum I

    RAB_DEF_OPCODE(
        r5900, 0x20, vaddq, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD Q
    RAB_DEF_OPCODE(
        r5900, 0x21, vmaddq, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD Q
    RAB_DEF_OPCODE(
        r5900, 0x22, vaddi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD I
    RAB_DEF_OPCODE(
        r5900, 0x23, vmaddi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD I
    RAB_DEF_OPCODE(
        r5900, 0x24, vsubq, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract Q
    RAB_DEF_OPCODE(
        r5900, 0x25, vmsubq, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract Q
    RAB_DEF_OPCODE(
        r5900, 0x26, vsubi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract I
    RAB_DEF_OPCODE(
        r5900, 0x27, vmsubi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract I
    RAB_DEF_OPCODE(
        r5900, 0x28, vadd, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // ADD
    RAB_DEF_OPCODE(
        r5900, 0x29, vmadd, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-ADD
    RAB_DEF_OPCODE(
        r5900, 0x2A, vmul, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MULtiply
    RAB_DEF_OPCODE(
        r5900, 0x2B, vmax, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MAXimum
    RAB_DEF_OPCODE(
        r5900, 0x2C, vsub, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // SUBtract
    RAB_DEF_OPCODE(
        r5900, 0x2D, vmsub, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Multiply-SUBtract
    RAB_DEF_OPCODE(
        r5900, 0x2E, vopmsub, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // Outer product post decrement
    RAB_DEF_OPCODE(
        r5900, 0x2F, vmini, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vfdxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900_xyzw),
        is_float: true
    ) // MINImum

    RAB_DEF_OPCODE(
        r5900, 0x30, viadd, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer ADD
    RAB_DEF_OPCODE(
        r5900, 0x31, visub, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer SUBtract
    RAB_DEF_OPCODE(
        r5900, 0x32, viaddi, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vit, Operand::r5900_vis, Operand::r5900_immediate5)
    ) // Integer ADD Immediate
    RAB_DEF_OPCODE(
        r5900, 0x34, viand, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer AND
    RAB_DEF_OPCODE(
        r5900, 0x35, vior, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit)
    ) // Integer OR

    RAB_DEF_OPCODE(
        r5900, 0x38, vcallms, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr1(Operand::r5900_immediate15)
    )
    RAB_DEF_OPCODE(
        r5900, 0x39, vcallmsr, IsaVersion::EXTENSION, Some(IsaExtension::R5900),
        operands: Operand::arr1(Operand::r5900_vis)
    )

    RAB_DEF_OPCODE_REDIRECT(0x3C, r5900, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3D, r5900, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3E, r5900, coprocessor2_special2)
    RAB_DEF_OPCODE_REDIRECT(0x3F, r5900, coprocessor2_special2)

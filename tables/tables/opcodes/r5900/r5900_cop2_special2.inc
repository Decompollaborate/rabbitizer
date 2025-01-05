/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
     31--------26-25----------------------------10------6-5-----2-1--0
     |    COP2   |1|                           |   fhi   |1 1 1 1|flo|
     ------6------1---------------------------------5--------4-----2--
     |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
0000 |VADDAx |VADDAy |VADDAz |VADDAw |VSUBAx |VSUBAy |VSUBAz |VSUBAw |
0001 |VMADDAx|VMADDAy|VMADDAz|VMADDAw|VMSUBAx|VMSUBAy|VMSUBAz|VMSUBAw|
0010 |VITOF0 |VITOF4 |VITOF12|VITOF15|VFTOI0 |VFTOI4 |VFTOI12|VFTOI15|
0011 |VMULAx |VMULAy |VMULAz |VMULAw |VMULAq |VABS   |VMULAi |VCLIPw |
0100 |VADDAq |VMADDAq|VADDAi |VMADDAi|VSUBAq |VMSUBAq|VSUBAi |VMSUBAi|
0101 |VADDA  |VMADDA |VMULA  | ---   |VSUBA  |VMSUBA |VOPMULA|VNOP   |
0110 |VMOVE  |VMR32  | ---   | ---   |VLQI   |VSQI   |VLQD   |VSQD   |
0111 |VDIV   |VSQRT  |VRSQRT |VWAITQ |VMTIR  |VMFIR  | *1    | *1    |
1000 |VRNEXT |VRGET  |VRINIT |VRXOR  | ---   | ---   | ---   | ---   |
1001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1100 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
1111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi  |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = VIWR list
*/

    // TODO: operands and properties

    RAB_DEF_OPCODE(
        r5900, 0x00, vaddax, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x01, vadday, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x02, vaddaz, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x03, vaddaw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x40, vsubax, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x41, vsubay, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x42, vsubaz, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x43, vsubaw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x80, vmaddax, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x81, vmadday, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x82, vmaddaz, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0x83, vmaddaw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0xC0, vmsubax, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0xC1, vmsubay, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0xC2, vmsubaz, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE(
        r5900, 0xC3, vmsubaw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast

    RAB_DEF_OPCODE(
        r5900, 0x100, vitof0, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE(
        r5900, 0x101, vitof4, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE(
        r5900, 0x102, vitof12, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE(
        r5900, 0x103, vitof15, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE(
        r5900, 0x140, vftoi0, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE(
        r5900, 0x141, vftoi4, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE(
        r5900, 0x142, vftoi12, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE(
        r5900, 0x143, vftoi15, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer

    RAB_DEF_OPCODE(
        r5900, 0x180, vmulax, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RAB_DEF_OPCODE(
        r5900, 0x181, vmulay, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RAB_DEF_OPCODE(
        r5900, 0x182, vmulaz, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RAB_DEF_OPCODE(
        r5900, 0x183, vmulaw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    // TODO
    RAB_DEF_OPCODE(
        r5900, 0x1C0, vmulaq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RAB_DEF_OPCODE(
        r5900, 0x1C1, vabs, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Absolute

    // TODO
    RAB_DEF_OPCODE(
        r5900, 0x1C2, vmulai, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RAB_DEF_OPCODE(
        r5900, 0x1C3, vclipw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Clip

    RAB_DEF_OPCODE(
        r5900, 0x200, vaddaq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator Q
    RAB_DEF_OPCODE(
        r5900, 0x201, vmaddaq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator Q
    RAB_DEF_OPCODE(
        r5900, 0x202, vaddai, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator I
    RAB_DEF_OPCODE(
        r5900, 0x203, vmaddai, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator I
    RAB_DEF_OPCODE(
        r5900, 0x240, vsubaq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Subtract accumulator Q
    RAB_DEF_OPCODE(
        r5900, 0x241, vmsubaq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and Subtract accumulator Q
    RAB_DEF_OPCODE(
        r5900, 0x242, vsubai, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Subtract accumulator I
    RAB_DEF_OPCODE(
        r5900, 0x243, vmsubai, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and Subtract accumulator I
    RAB_DEF_OPCODE(
        r5900, 0x280, vadda, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator
    RAB_DEF_OPCODE(
        r5900, 0x281, vmadda, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator

    // TODO
    RAB_DEF_OPCODE(
        r5900, 0x282, vmula, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RAB_DEF_OPCODE(
        r5900, 0x2C0, vsuba, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    )
    RAB_DEF_OPCODE(
        r5900, 0x2C1, vmsuba, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator
    RAB_DEF_OPCODE(
        r5900, 0x2C2, vopmula, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Outer product pre increment
    RAB_DEF_OPCODE(
        r5900, 0x2C3, vnop, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    ) // No operation

    RAB_DEF_OPCODE(
        r5900, 0x300, vmove, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move floating point registers
    RAB_DEF_OPCODE(
        r5900, 0x301, vmr32, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move and rotate per word
    RAB_DEF_OPCODE(
        r5900, 0x340, vlqi, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Load quadraword post increment
    RAB_DEF_OPCODE(
        r5900, 0x341, vsqi, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Store quadraword post increment
    RAB_DEF_OPCODE(
        r5900, 0x342, vlqd, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Load quadraword pre decrement
    RAB_DEF_OPCODE(
        r5900, 0x343, vsqd, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Store quadraword pre decrement

    RAB_DEF_OPCODE(
        r5900, 0x380, vdiv, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm)
    )
    RAB_DEF_OPCODE(
        r5900, 0x381, vsqrt, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_Q, Operand::r5900_vftm)
    )
    RAB_DEF_OPCODE(
        r5900, 0x382, vrsqrt, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl,Operand::r5900_vftm)
    )

    RAB_DEF_OPCODE(
        r5900, 0x383, vwaitq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    ) // Wait Q operation

    RAB_DEF_OPCODE(
        r5900, 0x3C0, vmtir, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vfsl)
    )

    RAB_DEF_OPCODE(
        r5900, 0x3C1, vmfir, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move from integer register

    RAB_DEF_OPCODE(
        r5900, 0x400, vrnext, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // R next
    RAB_DEF_OPCODE(
        r5900, 0x401, vrget, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // R move

    // TODO
    RAB_DEF_OPCODE(
        r5900, 0x402, vrinit, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl)
    )
    RAB_DEF_OPCODE(
        r5900, 0x403, vrxor, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl)
    )

    RAB_DEF_OPCODE_REDIRECT(0x3C2, r5900, coprocessor2_viwr)
    RAB_DEF_OPCODE_REDIRECT(0x3C3, r5900, coprocessor2_viwr)

/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x00, vaddax,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x01, vadday,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x02, vaddaz,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, vaddaw,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x40, vsubax,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x41, vsubay,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x42, vsubaz,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x43, vsubaw,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x80, vmaddax,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x81, vmadday,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x82, vmaddaz,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x83, vmaddaw,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    ) // Multiply and add accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0xC0, vmsubax,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0xC1, vmsubay,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0xC2, vmsubaz,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0xC3, vmsubaw,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator broadcast

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x100, vitof0,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x101, vitof4,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x102, vitof12,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x103, vitof15,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Integer to floating point
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x140, vftoi0,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x141, vftoi4,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x142, vftoi12,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x143, vftoi15,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Floating to integer

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x180, vmulax,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x181, vmulay,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x182, vmulaz,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x183, vmulaw,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw
    )
    // TODO
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C0, vmulaq,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C1, vabs,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Absolute

    // TODO
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C2, vmulai,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1C3, vclipw,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Clip

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x200, vaddaq,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x201, vmaddaq,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x202, vaddai,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x203, vmaddai,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x240, vsubaq,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Subtract accumulator Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x241, vmsubaq,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_Q),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and Subtract accumulator Q
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x242, vsubai,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Subtract accumulator I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x243, vmsubai,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_I),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and Subtract accumulator I
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x280, vadda,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Add accumulator
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x281, vmadda,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and add accumulator

    // TODO
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x282, vmula,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw
    )

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2C0, vsuba,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2C1, vmsuba,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Multiply and substract accumulator
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2C2, vopmula,
        operands: Operand::arr3(Operand::r5900_ACCxyzw, Operand::r5900_vfsxyzw, Operand::r5900_vftxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Outer product pre increment
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x2C3, vnop,
        operands: Operand::arr0()
    ) // No operation

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x300, vmove,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move floating point registers
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x301, vmr32,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move and rotate per word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x340, vlqi,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Load quadraword post increment
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x341, vsqi,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Store quadraword post increment
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x342, vlqd,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Load quadraword pre decrement
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x343, vsqd,
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Store quadraword pre decrement

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x380, vdiv,
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm)
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x381, vsqrt,
        operands: Operand::arr2(Operand::r5900_Q, Operand::r5900_vftm)
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x382, vrsqrt,
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl,Operand::r5900_vftm)
    )

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x383, vwaitq,
        operands: Operand::arr0()
    ) // Wait Q operation

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x3C0, vmtir,
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vfsl)
    )

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x3C1, vmfir,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // Move from integer register

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x400, vrnext,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // R next
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x401, vrget,
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true
    ) // R move

    // TODO
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x402, vrinit,
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl)
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x403, vrxor,
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl)
    )

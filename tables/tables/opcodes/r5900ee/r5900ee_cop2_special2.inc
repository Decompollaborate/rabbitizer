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

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x00, vaddax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x01, vadday, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x02, vaddaz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x03, vaddaw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x40, vsubax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x41, vsubay, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x42, vsubaz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x43, vsubaw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x80, vmaddax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x81, vmadday, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x82, vmaddaz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x83, vmaddaw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    ) // Multiply and add accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0xC0, vmsubax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0xC1, vmsubay, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0xC2, vmsubaz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0xC3, vmsubaw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator broadcast

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x100, vitof0, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x101, vitof4, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x102, vitof12, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x103, vitof15, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Integer to floating point
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x140, vftoi0, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x141, vftoi4, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x142, vftoi12, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Floating to integer
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x143, vftoi15, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Floating to integer

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x180, vmulax, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x181, vmulay, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x182, vmulaz, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x183, vmulaw, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )
    // TODO
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1C0, vmulaq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1C1, vabs, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Absolute

    // TODO
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1C2, vmulai, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1C3, vclipw, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Clip

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x200, vaddaq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator Q
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x201, vmaddaq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and add accumulator Q
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x202, vaddai, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator I
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x203, vmaddai, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and add accumulator I
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x240, vsubaq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Subtract accumulator Q
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x241, vmsubaq, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_Q),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and Subtract accumulator Q
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x242, vsubai, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Subtract accumulator I
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x243, vmsubai, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_I),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and Subtract accumulator I
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x280, vadda, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Add accumulator
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x281, vmadda, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and add accumulator

    // TODO
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x282, vmula, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x2C0, vsuba, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x2C1, vmsuba, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Multiply and substract accumulator
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x2C2, vopmula, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_ACCxyzw, Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Outer product pre increment
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x2C3, vnop, R5900EE,
        operands: Operand::arr0()
    ) // No operation

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x300, vmove, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Move floating point registers
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x301, vmr32, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Move and rotate per word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x340, vlqi, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_postincr),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Load quadraword post increment
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x341, vsqi, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_postincr),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Store quadraword post increment
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x342, vlqd, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_predecr),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Load quadraword pre decrement
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x343, vsqd, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_predecr),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Store quadraword pre decrement

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x380, vdiv, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_Q, Operand::r5900ee_vfsl, Operand::r5900ee_vftm)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x381, vsqrt, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_Q, Operand::r5900ee_vftm)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x382, vrsqrt, R5900EE,
        operands: Operand::arr3(Operand::r5900ee_Q, Operand::r5900ee_vfsl,Operand::r5900ee_vftm)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x383, vwaitq, R5900EE,
        operands: Operand::arr0()
    ) // Wait Q operation

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x3C0, vmtir, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vfsl)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x3C1, vmfir, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // Move from integer register

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x400, vrnext, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // R next
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x401, vrget, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
        instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
        is_float: true
    ) // R move

    // TODO
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x402, vrinit, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl)
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x403, vrxor, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl)
    )

    RAB_DEF_OPCODE_REDIRECT(0x3C2, r5900ee, coprocessor2_viwr)
    RAB_DEF_OPCODE_REDIRECT(0x3C3, r5900ee, coprocessor2_viwr)

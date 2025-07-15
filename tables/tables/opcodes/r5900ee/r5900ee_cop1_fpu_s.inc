/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------------5--------0
    |  = COP1   |  = S    |                               | function|
    ------6----------5-----------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ADD.S | SUB.S | MUL.S | DIV.S | SQRT.S| ABS.S | MOV.S | NEG.S |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   |RSQRT.S| ---   |
011 | ADDA.S| SUBA.S| MULA.S| ---   | MADD.S| MSUB.S|MADDA.S|MSUBA.S|
100 | ---   | ---   | ---   | ---   | CVT.W | ---   | ---   | ---   |
101 | MAX.S | MIN.S | ---   | ---   | ---   | ---   | ---   | ---   |
110 | C.F   | ---   | C.EQ  | ---   | C.LT  | ---   |  C.LE | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // The other instructions are implemented using the main CORE table

    // sqrt.s may sometimes not be properly encoded, so as a workaround we decode it like this.
    // TODO: We may be able to just decode it as words and call it a day.
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x04, c1__sqrt_s, c1, R5900EE,
        operands: Operand::arr1(Operand::core_copraw)
    )

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x16, rsqrt_s, rsqrt.s, R5900EE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point Reciprocal SQuare RooT

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x18, adda_s, adda.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true
    ) // floating point ADD to Accumulator
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x19, suba_s, suba.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true
    ) // floating point SUBtract to Accumulator
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x1A, mula_s, mula.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true
    ) // floating point MULtiply to Accumulator

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x1C, madd_s, madd.s, R5900EE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point Multiply-ADD
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x1D, msub_s, msub.s, R5900EE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point Multiply abd SUBtract
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x1E, madda_s, madda.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point Multiply-ADD Accumulator
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x1F, msuba_s, msuba.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point Multiply SUBtract from Accumulator

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x28, max_s, max.s, R5900EE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point MAXimum
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x29, min_s, min.s, R5900EE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // floating point MINimum

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x34, c_lt_s, c.lt.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP1_FPUS, 0x36, c_le_s, c.le.s, R5900EE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )

    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor1_fpu_s)

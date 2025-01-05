/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP1   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC1  | DMFC1 | CFC1  | ---   | MTC1  | DMTC1 | CTC1  | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | *3    | ---   | ---   | *4    | *5    | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = BC instructions, see BC1 list
     *2 = S instr, see FPU S list
     *3 = D instr, see FPU D list
     *4 = W instr, see FPU W list
     *5 = L instr, see FPU L list
*/

    // OP rt, fs
    RAB_DEF_OPCODE(
        core, 0x00, mfc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true
    ) // Move Word From Floating-Point
    RAB_DEF_OPCODE(
        core, 0x01, dmfc1, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true
    ) // Doubleword Move From Floating-Point
    RAB_DEF_OPCODE(
        core, 0x04, mtc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true
    ) // Move Word to Floating-Point
    RAB_DEF_OPCODE(
        core, 0x05, dmtc1, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true
    ) // Doubleword Move To Floating-Point

    // OP rt, cop1cs
    RAB_DEF_OPCODE(
        core, 0x02, cfc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true
    ) // Move Control Word from Floating-Point
    RAB_DEF_OPCODE(
        core, 0x06, ctc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true
    ) // Move Control Word to Floating-Point

    RAB_DEF_OPCODE_REDIRECT(0x08, none, coprocessor1_bc1)
    RAB_DEF_OPCODE_REDIRECT(0x10, none, coprocessor1_fpu_s)
    RAB_DEF_OPCODE_REDIRECT(0x11, none, coprocessor1_fpu_d)
    RAB_DEF_OPCODE_REDIRECT(0x14, none, coprocessor1_fpu_w)
    RAB_DEF_OPCODE_REDIRECT(0x15, none, coprocessor1_fpu_l)

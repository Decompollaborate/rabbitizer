/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x00, mfc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true
    ) // Move Word From Floating-Point
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, dmfc1, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true
    ) // Doubleword Move From Floating-Point
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x04, mtc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true
    ) // Move Word to Floating-Point
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x05, dmtc1, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true
    ) // Doubleword Move To Floating-Point

    // OP rt, cop1cs
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, cfc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true
    ) // Move Control Word from Floating-Point
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x06, ctc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true
    ) // Move Control Word to Floating-Point

/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP0   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC0  | DMFC0 | CFC0  | ---   | MTC0  | DMTC0 | CTC0  | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
       *1=BC See BC0 list       *2 = TLB instr, see TLB list
*/

    // OP rt, cop0d
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x00, mfc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Move word From CP0
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, dmfc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Doubleword Move From CP0
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, cfc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true
    ) // Move control word From CP0
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x04, mtc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Move word to CP0
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x05, dmtc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Doubleword Move To CP0
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x06, ctc0,
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Move control word To CP0

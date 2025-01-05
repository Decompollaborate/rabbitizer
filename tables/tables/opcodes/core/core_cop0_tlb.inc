/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21--------------------------------5--------0
    |  = COP0   |   TLB   |                                |  fmt   |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ---   | TLBR  | TLBWI | ---   | ---   | ---   | TLBWR | ---   |
001 | TLBP  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | RFE   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ERET  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------| 
*/

    // OP
    RAB_DEF_OPCODE(
        core, 0x01, tlbr, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true
    ) // Read Indexed TLB Entry
    RAB_DEF_OPCODE(
        core, 0x02, tlbwi, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true
    ) // Write Indexed TLB Entry

    RAB_DEF_OPCODE(
        core, 0x06, tlbwr, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN
    ) // Write Random TLB Entry

    RAB_DEF_OPCODE(
        core, 0x08, tlbp, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true
    ) // Probe TLB for Matching Entry

    RAB_DEF_OPCODE(
        core, 0x10, rfe, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true
    ) // Restore From Exception

    RAB_DEF_OPCODE(
        core, 0x18, eret, IsaVersion::MIPS_I, None,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true
    ) // Return from Exception

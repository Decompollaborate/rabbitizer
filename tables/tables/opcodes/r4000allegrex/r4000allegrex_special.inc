/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = SPECIAL |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLL   | ---   | *1    | SRA   | SLLV  | ---   | *2    | SRAV  |
001 | JR    | JALR  | MOVZ  | MOVN  |SYSCALL| BREAK | ---   | SYNC  |
010 | MFHI  | MTHI  | MFLO  | MTLO  | ---   | ---   | CLZ   | CLO   |
011 | MULT  | MULTU | DIV   | DIVU  | MADD  | MADDU | ---   | ---   |
100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
101 | ---   | ---   | SLT   | SLTU  | MAX   | MIN   | MSUB  | MSUBU |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL rs, see SPECIAL rs list
     *2 = SPECIAL sa, see SPECIAL sa list
*/

    // The other instructions are implemented using the main CORE table

    // OP rd, rs

    RAB_DEF_OPCODE(
        r4000allegrex, 0x16, clz, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true
    ) // Count Leading Zero

    RAB_DEF_OPCODE(
        r4000allegrex, 0x17, clo, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true
    ) // Count Leading One

    // OP rs, rt

    RAB_DEF_OPCODE(
        r4000allegrex, 0x1C, madd, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Multiply ADD

    RAB_DEF_OPCODE(
        r4000allegrex, 0x1D, maddu, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Multiply ADD Unsigned

    RAB_DEF_OPCODE(
        r4000allegrex, 0x2E, msub, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Multiply SUBtract

    RAB_DEF_OPCODE(
        r4000allegrex, 0x2F, msubu, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Multiply SUBtract Unsigned

    // OP rd, rs, rt

    RAB_DEF_OPCODE(
        r4000allegrex, 0x0A, movz, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RAB_DEF_OPCODE(
        r4000allegrex, 0x0B, movn, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero

    RAB_DEF_OPCODE(
        r4000allegrex, 0x2C, max, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // select MAX

    RAB_DEF_OPCODE(
        r4000allegrex, 0x2D, min, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // select MIN

    RAB_DEF_OPCODE_REDIRECT(0x02, r4000allegrex, special_rs)
    RAB_DEF_OPCODE_REDIRECT(0x06, r4000allegrex, special_sa)

    RAB_DEF_OPCODE_REDIRECT(_, none, special)

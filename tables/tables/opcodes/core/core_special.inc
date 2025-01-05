/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = SPECIAL |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLL   | ---   | SRL   | SRA   | SLLV  | ---   | SRLV  | SRAV  |
001 | JR    | JALR  | MOVZ  | MOVN  |SYSCALL| BREAK | ---   | SYNC  |
010 | MFHI  | MTHI  | MFLO  | MTLO  | DSLLV | ---   | DSRLV | DSRAV |
011 | MULT  | MULTU | DIV   | DIVU  | DMULT | DMULTU| DDIV  | DDIVU |
100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
101 | ---   | ---   | SLT   | SLTU  | DADD  | DADDU | DSUB  | DSUBU |
110 | TGE   | TGEU  | TLT   | TLTU  | TEQ   | ---   | TNE   | ---   |
111 | DSLL  | ---   | DSRL  | DSRA  | DSLL32| ---   | DSRL32| DSRA32|
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP rd, rt, sa
    RAB_DEF_OPCODE(
        core, 0x00, sll, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Left Logical
    RAB_DEF_OPCODE(
        core, 0x02, srl, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical
    RAB_DEF_OPCODE(
        core, 0x03, sra, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Arithmetic
    RAB_DEF_OPCODE(
        core, 0x38, dsll, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical
    RAB_DEF_OPCODE(
        core, 0x3A, dsrl, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical
    RAB_DEF_OPCODE(
        core, 0x3B, dsra, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic
    RAB_DEF_OPCODE(
        core, 0x3C, dsll32, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical plus 32
    RAB_DEF_OPCODE(
        core, 0x3E, dsrl32, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical plus 32
    RAB_DEF_OPCODE(
        core, 0x3F, dsra32, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic plus 32

    // OP rd, rt, rs
    RAB_DEF_OPCODE(
        core, 0x14, dsllv, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical Variable
    RAB_DEF_OPCODE(
        core, 0x16, dsrlv, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical Variable
    RAB_DEF_OPCODE(
        core, 0x17, dsrav, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic Variable
    RAB_DEF_OPCODE(
        core, 0x04, sllv, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Left Logical Variable
    RAB_DEF_OPCODE(
        core, 0x06, srlv, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable
    RAB_DEF_OPCODE(
        core, 0x07, srav, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Arithmetic Variable

    // OP rs
    RAB_DEF_OPCODE(
        core, 0x11, mthi, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_hi: true
    ) // Move To HI register
    RAB_DEF_OPCODE(
        core, 0x13, mtlo, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_lo: true
    ) // Move To LO register
    RAB_DEF_OPCODE(
        core, 0x08, jr, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true
    ) // Jump Register
    RAB_DEF_OPCODE(
        core, 0x09, jalr, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true
    ) // Jump And Link Register

    // OP rd
    RAB_DEF_OPCODE(
        core, 0x10, mfhi, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_hi: true
    ) // Move From HI register
    RAB_DEF_OPCODE(
        core, 0x12, mflo, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_lo: true
    ) // Move From LO register

    // OP rd, rs, rt
    RAB_DEF_OPCODE(
        core, 0x0A, movz, IsaVersion::MIPS_IV, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RAB_DEF_OPCODE(
        core, 0x0B, movn, IsaVersion::MIPS_IV, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero
    RAB_DEF_OPCODE(
        core, 0x1A, div, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide word
    RAB_DEF_OPCODE(
        core, 0x1B, divu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide Unsigned word

    RAB_DEF_OPCODE(
        core, 0x1E, ddiv, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide
    RAB_DEF_OPCODE(
        core, 0x1F, ddivu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide Unsigned

    RAB_DEF_OPCODE(
        core, 0x20, add, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // ADD word
    RAB_DEF_OPCODE(
        core, 0x21, addu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // ADD Unsigned word
    RAB_DEF_OPCODE(
        core, 0x22, sub, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true
    ) // Subtract word
    RAB_DEF_OPCODE(
        core, 0x23, subu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // SUBtract Unsigned word
    RAB_DEF_OPCODE(
        core, 0x24, and, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // AND
    RAB_DEF_OPCODE(
        core, 0x25, or, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        maybe_is_move: true,
        reads_rs: true,
        reads_rt: true
    ) // OR
    RAB_DEF_OPCODE(
        core, 0x26, xor, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // eXclusive OR
    RAB_DEF_OPCODE(
        core, 0x27, nor, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Not OR
    RAB_DEF_OPCODE(
        core, 0x2A, slt, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than
    RAB_DEF_OPCODE(
        core, 0x2B, sltu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than Unsigned
    RAB_DEF_OPCODE(
        core, 0x2C, dadd, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Add
    RAB_DEF_OPCODE(
        core, 0x2D, daddu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // Doubleword Add Unsigned
    RAB_DEF_OPCODE(
        core, 0x2E, dsub, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword SUBtract
    RAB_DEF_OPCODE(
        core, 0x2F, dsubu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword SUBtract Unsigned

    // OP code
    RAB_DEF_OPCODE(
        core, 0x0C, syscall, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_code_lower),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true
    ) // SYStem CALL
    RAB_DEF_OPCODE(
        core, 0x0D, break, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_code),
        instr_type: InstrType::R
    ) // Break
    RAB_DEF_OPCODE(
        core, 0x0F, sync, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // Sync

    // OP rs, rt
    RAB_DEF_OPCODE(
        core, 0x18, mult, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply word
    RAB_DEF_OPCODE(
        core, 0x19, multu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply Unsigned word
    RAB_DEF_OPCODE(
        core, 0x1C, dmult, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply
    RAB_DEF_OPCODE(
        core, 0x1D, dmultu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply Unsigned

    RAB_DEF_OPCODE(
        core, 0x30, tge, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal
    RAB_DEF_OPCODE(
        core, 0x31, tgeu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal Unsigned
    RAB_DEF_OPCODE(
        core, 0x32, tlt, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than
    RAB_DEF_OPCODE(
        core, 0x33, tltu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than Unsigned
    RAB_DEF_OPCODE(
        core, 0x34, teq, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if EQual
    RAB_DEF_OPCODE(
        core, 0x36, tne, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Not Equal


    // Pseudo-Instruction Unique IDs

    // OP
    RAB_DEF_OPCODE_PSEUDO(
        core, nop, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // No OPeration

    // OP rd, rs
    RAB_DEF_OPCODE_PSEUDO(
        core, move, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true
    ) // Move
    RAB_DEF_OPCODE_PSEUDO(
        core, not, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true
    ) // Not

    // OP rd, rt
    RAB_DEF_OPCODE_PSEUDO(
        core, neg, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true
    )

    // OP rd, rt
    RAB_DEF_OPCODE_PSEUDO(
        core, negu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    )

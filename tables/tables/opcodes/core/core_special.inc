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
    RAB_DEF_OPCODE_MIPS1(
        core, 0x00, sll,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Left Logical
    RAB_DEF_OPCODE_MIPS1(
        core, 0x02, srl,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical
    RAB_DEF_OPCODE_MIPS1(
        core, 0x03, sra,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Arithmetic
    RAB_DEF_OPCODE(
        core, 0x38, dsll, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical
    RAB_DEF_OPCODE(
        core, 0x3A, dsrl, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical
    RAB_DEF_OPCODE(
        core, 0x3B, dsra, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic
    RAB_DEF_OPCODE(
        core, 0x3C, dsll32, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical plus 32
    RAB_DEF_OPCODE(
        core, 0x3E, dsrl32, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical plus 32
    RAB_DEF_OPCODE(
        core, 0x3F, dsra32, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic plus 32

    // OP rd, rt, rs
    RAB_DEF_OPCODE(
        core, 0x14, dsllv, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical Variable
    RAB_DEF_OPCODE(
        core, 0x16, dsrlv, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical Variable
    RAB_DEF_OPCODE(
        core, 0x17, dsrav, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic Variable
    RAB_DEF_OPCODE_MIPS1(
        core, 0x04, sllv,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Left Logical Variable
    RAB_DEF_OPCODE_MIPS1(
        core, 0x06, srlv,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable
    RAB_DEF_OPCODE_MIPS1(
        core, 0x07, srav,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Arithmetic Variable

    // OP rs
    RAB_DEF_OPCODE_MIPS1(
        core, 0x11, mthi,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_hi: true
    ) // Move To HI register
    RAB_DEF_OPCODE_MIPS1(
        core, 0x13, mtlo,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_lo: true
    ) // Move To LO register
    RAB_DEF_OPCODE_MIPS1(
        core, 0x08, jr,
        operands: Operand::arr1(Operand::core_rs),
        instr_type: InstrType::R,
        is_jump: true,
        jumps_to_register: true,
        reads_rs: true
    ) // Jump Register
    RAB_DEF_OPCODE_MIPS1(
        core, 0x09, jalr,
        operands: Operand::arr1(Operand::core_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        jumps_to_register: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true
    ) // Jump And Link Register

    // OP rd
    RAB_DEF_OPCODE_MIPS1(
        core, 0x10, mfhi,
        operands: Operand::arr1(Operand::core_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_hi: true
    ) // Move From HI register
    RAB_DEF_OPCODE_MIPS1(
        core, 0x12, mflo,
        operands: Operand::arr1(Operand::core_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_lo: true
    ) // Move From LO register

    // OP rd, rs, rt
    RAB_DEF_OPCODE(
        core, 0x0A, movz, MIPS_IV,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RAB_DEF_OPCODE(
        core, 0x0B, movn, MIPS_IV,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero
    RAB_DEF_OPCODE_MIPS1(
        core, 0x1A, div,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x1B, divu,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide Unsigned word

    RAB_DEF_OPCODE(
        core, 0x1E, ddiv, MIPS_III,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide
    RAB_DEF_OPCODE(
        core, 0x1F, ddivu, MIPS_III,
        operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide Unsigned

    RAB_DEF_OPCODE_MIPS1(
        core, 0x20, add,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        adds_registers: true,
        not_emitted_by_compilers: true
    ) // ADD word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x21, addu,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        adds_registers: true
    ) // ADD Unsigned word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x22, sub,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true,
        subs_registers: true
    ) // Subtract word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x23, subu,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        subs_registers: true
    ) // SUBtract Unsigned word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x24, and,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ands_registers: true
    ) // AND
    RAB_DEF_OPCODE_MIPS1(
        core, 0x25, or,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        ors_registers: true,
        reads_rs: true,
        reads_rt: true
    ) // OR
    RAB_DEF_OPCODE_MIPS1(
        core, 0x26, xor,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // eXclusive OR
    RAB_DEF_OPCODE_MIPS1(
        core, 0x27, nor,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Not OR
    RAB_DEF_OPCODE_MIPS1(
        core, 0x2A, slt,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than
    RAB_DEF_OPCODE_MIPS1(
        core, 0x2B, sltu,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than Unsigned
    RAB_DEF_OPCODE(
        core, 0x2C, dadd, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        adds_registers: true
    ) // Doubleword Add
    RAB_DEF_OPCODE(
        core, 0x2D, daddu, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        adds_registers: true
    ) // Doubleword Add Unsigned
    RAB_DEF_OPCODE(
        core, 0x2E, dsub, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        subs_registers: true
    ) // Doubleword SUBtract
    RAB_DEF_OPCODE(
        core, 0x2F, dsubu, MIPS_III,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        subs_registers: true
    ) // Doubleword SUBtract Unsigned

    // OP code
    RAB_DEF_OPCODE_MIPS1(
        core, 0x0C, syscall,
        operands: Operand::arr1(Operand::core_code_lower),
        instr_type: InstrType::R,
        causes_exception: true,
        causes_unconditional_exception: true,
        causes_returnable_exception: true,
        not_emitted_by_compilers: true
    ) // SYStem CALL
    RAB_DEF_OPCODE_MIPS1(
        core, 0x0D, break,
        operands: Operand::arr1(Operand::core_code),
        causes_exception: true,
        causes_unconditional_exception: true,
        instr_type: InstrType::R
    ) // Break
    RAB_DEF_OPCODE(
        core, 0x0F, sync, MIPS_II,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // Sync

    // OP rs, rt
    RAB_DEF_OPCODE_MIPS1(
        core, 0x18, mult,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply word
    RAB_DEF_OPCODE_MIPS1(
        core, 0x19, multu,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply Unsigned word
    RAB_DEF_OPCODE(
        core, 0x1C, dmult, MIPS_III,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply
    RAB_DEF_OPCODE(
        core, 0x1D, dmultu, MIPS_III,
        operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply Unsigned

    RAB_DEF_OPCODE(
        core, 0x30, tge, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal
    RAB_DEF_OPCODE(
        core, 0x31, tgeu, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal Unsigned
    RAB_DEF_OPCODE(
        core, 0x32, tlt, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than
    RAB_DEF_OPCODE(
        core, 0x33, tltu, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than Unsigned
    RAB_DEF_OPCODE(
        core, 0x34, teq, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if EQual
    RAB_DEF_OPCODE(
        core, 0x36, tne, MIPS_II,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
        instr_type: InstrType::R,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // Trap if Not Equal


    // Pseudo-instructions

    // OP
    RAB_DEF_OPCODE_MIPS1_PSEUDO(
        core, nop,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // No OPeration

    RAB_DEF_OPCODE_MIPS1_PSEUDO(
        core, not,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true
    ) // Not

    // OP rd, rt
    RAB_DEF_OPCODE_MIPS1_PSEUDO(
        core, neg,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true
    )

    // OP rd, rt
    RAB_DEF_OPCODE_MIPS1_PSEUDO(
        core, negu,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    )

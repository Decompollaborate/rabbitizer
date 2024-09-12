/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x00, sll,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Left Logical
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, srl,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x03, sra,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x38, dsll,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x3A, dsrl,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x3B, dsra,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x3C, dsll32,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical plus 32
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x3E, dsrl32,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical plus 32
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x3F, dsra32,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic plus 32

    // OP rd, rt, rs
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x14, dsllv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x16, dsrlv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x17, dsrav,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Shift Right Arithmetic Variable
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x04, sllv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Left Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x06, srlv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x07, srav,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Arithmetic Variable

    // OP rs
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x11, mthi,
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_hi: true
    ) // Move To HI register
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x13, mtlo,
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_lo: true
    ) // Move To LO register
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x08, jr,
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true
    ) // Jump Register
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x09, jalr,
        operands: Operand::arr1(Operand::cpu_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true
    ) // Jump And Link Register

    // OP rd
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x10, mfhi,
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_hi: true
    ) // Move From HI register
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x12, mflo,
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_lo: true
    ) // Move From LO register

    // OP rd, rs, rt
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0A, movz,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Zero
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0B, movn,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MOVe conditional on Not zero
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1A, div,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1B, divu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        cpu, -0x1A, sn64_div, div,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide word
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        cpu, -0x1B, sn64_divu, divu,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // DIVide Unsigned word

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1E, ddiv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1F, ddivu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword DIVide Unsigned

    /*
    RABBITIZER_DEF_INSTR_ID(
        cpu, , ddiv,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    )
    */
    /*
    RABBITIZER_DEF_INSTR_ID(
        cpu, , ddivu,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    )
    */

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x20, add,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true
    ) // ADD word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x21, addu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // ADD Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x22, sub,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true
    ) // Subtract word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x23, subu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // SUBtract Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x24, and,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // AND
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x25, or,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        maybe_is_move: true,
        reads_rs: true,
        reads_rt: true
    ) // OR
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x26, xor,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // eXclusive OR
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x27, nor,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Not OR
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2A, slt,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2B, sltu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Set on Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2C, dadd,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword Add
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2D, daddu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // Doubleword Add Unsigned
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2E, dsub,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword SUBtract
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x2F, dsubu,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Doubleword SUBtract Unsigned

    // OP code
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0C, syscall,
        operands: Operand::arr1(Operand::cpu_code_lower),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true
    ) // SYStem CALL
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0D, break,
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R
    ) // Break
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0F, sync,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // Sync

    // OP rs, rt
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x18, mult,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x19, multu,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // MULTtiply Unsigned word
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1C, dmult,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x1D, dmultu,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true
    ) // Doubleword MULTiply Unsigned

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x30, tge,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x31, tgeu,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Greater or Equal Unsigned
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x32, tlt,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x33, tltu,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Less Than Unsigned
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x34, teq,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if EQual
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x36, tne,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true
    ) // Trap if Not Equal


    // Pseudo-Instruction Unique IDs

    // OP
    RABBITIZER_DEF_INSTR_ID(
        cpu, -0x01, nop,
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true
    ) // No OPeration

    // OP rd, rs
    RABBITIZER_DEF_INSTR_ID(
        cpu, -0x25, move,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true
    ) // Move
    RABBITIZER_DEF_INSTR_ID(
        cpu, -0x27, not,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true
    ) // Not

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(
        cpu, -0x22, neg,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )

    // OP rd, rt
    RABBITIZER_DEF_INSTR_ID(
        cpu, -0x23, negu,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true
    )

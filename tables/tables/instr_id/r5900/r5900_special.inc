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
011 | MULT  | MULTU | DIV   | DIVU  | ----  | ---   | ----  | ----- |
100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
101 | MFSA  | MTSA  | SLT   | SLTU  | DADD  | DADDU | DSUB  | DSUBU |
110 | TGE   | TGEU  | TLT   | TLTU  | TEQ   | ---   | TNE   | ---   |
111 | DSLL  | ---   | DSRL  | DSRA  |DSLL32 | ---   |DSRL32 |DSRA32 |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // The other instructions are implemented using the main CPU table

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r5900, 0x0F, sync_p, sync.p, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    ) // Sync

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x18, mult, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // MULTtiply word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x28, mfsa, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true
    ) // Move From Shift Amount register
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x29, mtsa, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true
    ) // Move To Shift Amount register

/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | = SPECIAL |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLL   | ---   | SRL   | SRA   | SLLV  | ---   | SRLV  | SRAV  |
001 | JR    | JALR  | MOVZ  | MOVN  | ---   | BREAK | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
101 | ---   | ---   | SLT   | SLTU  | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0C, core, syscall, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x0F, core, sync, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x10, core, mfhi, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x11, core, mthi, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x12, core, mflo, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x13, core, mtlo, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x14, core, dsllv, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x16, core, dsrlv, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x17, core, dsrav, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x18, core, mult, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x19, core, multu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1A, core, div, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1B, core, divu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1C, core, dmult, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1D, core, dmultu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1E, core, ddiv, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x1F, core, ddivu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2C, core, dadd, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2D, core, daddu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2E, core, dsub, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x2F, core, dsubu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x30, core, tge, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x31, core, tgeu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x32, core, tlt, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x33, core, tltu, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x34, core, teq, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x36, core, tne, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x38, core, dsll, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3A, core, dsrl, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3B, core, dsra, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3C, core, dsll32, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3E, core, dsrl32, RSP)
    RAB_DEF_OPCODE_EXTENSION_REMOVED(0x3F, core, dsra32, RSP)

    RAB_DEF_OPCODE_REDIRECT(_, none, special)

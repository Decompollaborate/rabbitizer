/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
vnop
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|
    ------6-------------------------------26-------------------------
*/

    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x3FF0000, vnop, R4000ALLEGREX,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN
    ) // Nop

/*
vsync
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 1 1 0 0 1 0 0 0 0 0|
    ------6-------------------------------26-------------------------
*/

    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x3FF0320, vsync, R4000ALLEGREX,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN
    ) // Synchronize

/*
vflush
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 1 0 0 0 0 0 0 1 1 0 1|
    ------6-------------------------------26-------------------------
*/

    RAB_DEF_OPCODE_EXTENSION(
        r4000allegrex, 0x3FF040D, vflush, R4000ALLEGREX,
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN
    ) // Flush Write Buffer

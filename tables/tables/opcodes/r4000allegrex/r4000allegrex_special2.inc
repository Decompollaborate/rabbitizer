/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | =SPECIAL2 |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | SLEEP | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | MFIE  | ---   | MTIE  | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP

    RAB_DEF_OPCODE_EXT(
        r4000allegrex, 0x00, sleep, R4000ALLEGREX,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    )

    RAB_DEF_OPCODE_EXT(
        r4000allegrex, 0x24, mfie, R4000ALLEGREX,
        operands: Operand::arr1(Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rt: true
    )

    RAB_DEF_OPCODE_EXT(
        r4000allegrex, 0x26, mtie, R4000ALLEGREX,
        operands: Operand::arr1(Operand::core_rt),
        instr_type: InstrType::R,
        reads_rt: true
    )

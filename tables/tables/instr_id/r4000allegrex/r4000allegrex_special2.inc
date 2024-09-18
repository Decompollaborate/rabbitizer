/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, sleep, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr0(),
        instr_type: InstrType::R
    )

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x24, mfie, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rt: true
    )

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x26, mtie, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rt: true
    )

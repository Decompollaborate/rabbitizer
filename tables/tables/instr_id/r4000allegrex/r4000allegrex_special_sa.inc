/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26--------------------------------10------6-5--------0
    | = SPECIAL |                               |   sa    |  0x06   |
    ------6------------------------------------------5---------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | srlv  | rotrv | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, srlv, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Shift word Right Logical Variable

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, rotrv, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // ROTate word Right Variable

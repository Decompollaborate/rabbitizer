/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21---------------------------------5--------0
    | = SPECIAL |   rs    |                               |  0x02   |
    ------6----------5-----------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | srl   | rotr  | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x00, srl, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Shift word Right Logical

    RAB_DEF_OPCODE(
        r4000allegrex, 0x01, rotr, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // ROTate word Right

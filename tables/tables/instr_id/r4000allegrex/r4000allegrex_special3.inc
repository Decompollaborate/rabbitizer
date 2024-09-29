/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    | =SPECIAL3 |                                         | function|
    ------6----------------------------------------------------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | EXT   | ---   | ---   | ---   | INS   | ---   | ---   | ---   |
001 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL3 BSHFL
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, ext, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr4(Operand::core_rt, Operand::core_rs, Operand::r4000allegrex_pos, Operand::r4000allegrex_size),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true
    ) // EXTract bit field

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x04, ins, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr4(Operand::core_rt, Operand::core_rs, Operand::r4000allegrex_pos, Operand::r4000allegrex_size_plus_pos),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true
    ) // INSert bit field

/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------------------5--------0
    |           |                               |function  |  MMI1  |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 | ---   | PABSW | PCEQW | PMINW |
001 |PADSBH | PABSH | PCEQH | PMINH |
010 | ---   | ---   | PCEQB | ---   |
011 | ---   | ---   | ---   | ---   |
100 |PADDUW |PSUBUW |PEXTUW | ---   |
101 |PADDUH |PSUBUH |PEXTUH | ---   |
110 |PADDUB |PSUBUB |PEXTUB | QFSRV |
111 | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|
*/

    // TODO: operands and properties
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x01, pabsw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x02, pceqw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, pminw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x04, padsbh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADd/SuBtract Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x05, pabsh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x06, pceqh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x07, pminh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0A, pceqb, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Byte

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x10, padduw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x11, psubuw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, pextuw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x14, padduh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x15, psubuh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x16, pextuh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x18, paddub, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // Parallel ADD Unsigned saturation Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x19, psubub, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, pextub, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, qfsrv, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Quadword Funnel Shift Right Variable

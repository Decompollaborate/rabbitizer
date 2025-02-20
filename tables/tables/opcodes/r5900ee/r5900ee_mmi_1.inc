/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
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
    RAB_DEF_OPCODE(
        r5900ee, 0x01, pabsw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Word
    RAB_DEF_OPCODE(
        r5900ee, 0x02, pceqw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Word
    RAB_DEF_OPCODE(
        r5900ee, 0x03, pminw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Word

    RAB_DEF_OPCODE(
        r5900ee, 0x04, padsbh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADd/SuBtract Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x05, pabsh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x06, pceqh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x07, pminh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Halfword

    RAB_DEF_OPCODE(
        r5900ee, 0x0A, pceqb, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Byte

    RAB_DEF_OPCODE(
        r5900ee, 0x10, padduw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Word
    RAB_DEF_OPCODE(
        r5900ee, 0x11, psubuw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Word
    RAB_DEF_OPCODE(
        r5900ee, 0x12, pextuw, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Word

    RAB_DEF_OPCODE(
        r5900ee, 0x14, padduh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x15, psubuh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Halfword
    RAB_DEF_OPCODE(
        r5900ee, 0x16, pextuh, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Halfword

    RAB_DEF_OPCODE(
        r5900ee, 0x18, paddub, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // Parallel ADD Unsigned saturation Byte
    RAB_DEF_OPCODE(
        r5900ee, 0x19, psubub, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Byte
    RAB_DEF_OPCODE(
        r5900ee, 0x1A, pextub, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Byte
    RAB_DEF_OPCODE(
        r5900ee, 0x1B, qfsrv, IsaVersion::EXTENSION, Some(IsaExtension::R5900EE),
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Quadword Funnel Shift Right Variable

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
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x01, pabsw, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x02, pceqw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x03, pminw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x04, padsbh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADd/SuBtract Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x05, pabsh, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x06, pceqh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x07, pminh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x0A, pceqb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Byte

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x10, padduw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x11, psubuw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x12, pextuw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x14, padduh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x15, psubuh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x16, pextuh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x18, paddub, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        adds_registers: true
    ) // Parallel ADD Unsigned saturation Byte
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x19, psubub, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Byte
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1A, pextub, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Byte
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1B, qfsrv, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Quadword Funnel Shift Right Variable

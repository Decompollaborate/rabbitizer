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
        r5900, 0x01, pabsw,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x02, pceqw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x03, pminw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x04, padsbh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADd/SuBtract Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x05, pabsh,
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel ABSolute Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x06, pceqh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x07, pminh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MINimum Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x0A, pceqb,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for EQual Byte

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x10, padduw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x11, psubuw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Word
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x12, pextuw,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Word

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x14, padduh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Unsigned saturation Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x15, psubuh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Halfword
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x16, pextuh,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Halfword

    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x18, paddub,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true
    ) // Parallel ADD Unsigned saturation Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x19, psubub,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Unsigned saturation Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1A, pextub,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Upper from Byte
    RABBITIZER_DEF_INSTR_ID(
        r5900, 0x1B, qfsrv,
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Quadword Funnel Shift Right Variable

/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26--------------------------------10------6-5--------0
    | =SPECIAL3 |                               |   sa    | = BSHFL |
    ------6------------------------------------------5---------6-----
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | ---   | ---   | WSBH  | WSBW  | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | SEB   | ---   | ---   | ---   | BITREV| ---   | ---   | ---   |
 11 | SEH   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP rd, rt

    RAB_DEF_OPCODE(
        r4000allegrex, 0x02, wsbh, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Word Swap Bytes within Halfword

    RAB_DEF_OPCODE(
        r4000allegrex, 0x03, wsbw, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Word Swap Bytes within Word

    RAB_DEF_OPCODE(
        r4000allegrex, 0x10, seb, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Sign-Extend Byte

    RAB_DEF_OPCODE(
        r4000allegrex, 0x18, seh, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // Sign-Extend Halfword

    RAB_DEF_OPCODE(
        r4000allegrex, 0x14, bitrev, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true
    ) // BIT REVerse

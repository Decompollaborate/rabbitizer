/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP2   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | ---   | QMFC2 | CFC2  | ---   | ---   | QMTC2 | CTC2  | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | *2    | *2    | *2    | *2    | *2    | *2    | *2    |
 11 | *2    | *2    | *2    | *2    | *2    | *2    | *2    | *2    |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = BC instructions, see BC2 list   *2 =see special1 table
*/

    // TODO: properties
    RAB_DEF_OPCODE(
        r5900, 0x01, qmfc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900_vfs),
        modifies_rt: true
    )
    RAB_DEF_OPCODE(
        r5900, 0x02, cfc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900_vis),
        modifies_rt: true
    )
    RAB_DEF_OPCODE(
        r5900, 0x05, qmtc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900_vfs),
        reads_rt: true
    )
    RAB_DEF_OPCODE(
        r5900, 0x06, ctc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900_vis),
        reads_rt: true
    )

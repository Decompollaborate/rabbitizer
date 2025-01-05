/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23----------------------------------------------0
    |   VFPU5   | fmt |                                             |
    ------6--------3-------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
    | vpfxs | ---   | vpfxt | ---   | vpfxd | ---   | viim.s| vfim.s|
    |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 See fmt3 table
*/

/*
vpfxs
    31--------26-25-23-------19-------------------------------------0
    |   VFPU5   |0 0 0|     |                                       |
    ------6--------3-----------------------------20-------------------
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x00, vpfxs, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr4(Operand::r4000allegrex_rpx, Operand::r4000allegrex_rpy, Operand::r4000allegrex_rpz, Operand::r4000allegrex_rpw),
        instr_type: InstrType::UNKNOWN
    ) // Source Prefix

/*
vpfxt
    31--------26-25-23-------19-------------------------------------0
    |   VFPU5   |0 1 0|     |                                       |
    ------6--------3----------------------------20-------------------
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x02, vpfxt, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr4(Operand::r4000allegrex_rpx, Operand::r4000allegrex_rpy, Operand::r4000allegrex_rpz, Operand::r4000allegrex_rpw),
        instr_type: InstrType::UNKNOWN
    ) // Target Prefix

/*
vpfxd
    31--------26----23-------19-------------------------------------0
    |   VFPU5   |1 0 0|     |                                       |
    ------6--------3----------------------------20-------------------
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x04, vpfxd, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr4(Operand::r4000allegrex_wpx, Operand::r4000allegrex_wpy, Operand::r4000allegrex_wpz, Operand::r4000allegrex_wpw),
        instr_type: InstrType::UNKNOWN
    ) // Destination Prefix

/*
viim.s
    31--------26-25-23-22---------16-15-----------------------------0
    |   VFPU5   |1 1 0|      vt     |              imm16            |
    ------6--------3---------7-----------------------16--------------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x6, viim_s, viim.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_int16),
        instr_type: InstrType::R
    ) // Convert integer immediate to float Single Word

/*
vfim.s
    31--------26-25-23-22---------16-15-----------------------------0
    |   VFPU5   |1 1 1|      vt     |              imm16            |
    ------6--------3---------7-----------------------16--------------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x7, vfim_s, vfim.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_float16),
        instr_type: InstrType::R
    ) // Convert integer immediate to float Single Word

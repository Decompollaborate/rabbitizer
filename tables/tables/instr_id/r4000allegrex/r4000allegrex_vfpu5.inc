/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, vpfxs,
        .operands={RAB_OPERAND_r4000allegrex_rpx, RAB_OPERAND_r4000allegrex_rpy, RAB_OPERAND_r4000allegrex_rpz, RAB_OPERAND_r4000allegrex_rpw},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Source Prefix

/*
vpfxt
    31--------26-25-23-------19-------------------------------------0
    |   VFPU5   |0 1 0|     |                                       |
    ------6--------3----------------------------20-------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, vpfxt,
        .operands={RAB_OPERAND_r4000allegrex_rpx, RAB_OPERAND_r4000allegrex_rpy, RAB_OPERAND_r4000allegrex_rpz, RAB_OPERAND_r4000allegrex_rpw},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Target Prefix

/*
vpfxd
    31--------26----23-------19-------------------------------------0
    |   VFPU5   |1 0 0|     |                                       |
    ------6--------3----------------------------20-------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x04, vpfxd,
        .operands={RAB_OPERAND_r4000allegrex_wpx, RAB_OPERAND_r4000allegrex_wpy, RAB_OPERAND_r4000allegrex_wpz, RAB_OPERAND_r4000allegrex_wpw},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Destination Prefix

/*
viim.s
    31--------26-25-23-22---------16-15-----------------------------0
    |   VFPU5   |1 1 0|      vt     |              imm16            |
    ------6--------3---------7-----------------------16--------------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6, viim_s, viim.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt, RAB_OPERAND_r4000allegrex_int16},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer immediate to float Single Word

/*
vfim.s
    31--------26-25-23-22---------16-15-----------------------------0
    |   VFPU5   |1 1 1|      vt     |              imm16            |
    ------6--------3---------7-----------------------16--------------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x7, vfim_s, vfim.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt, RAB_OPERAND_r4000allegrex_float16},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer immediate to float Single Word

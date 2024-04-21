/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-----23---------------------------------------------0
    |   VFPU5   |1 1|f|                                             |
    ------6-------2--1-----------------------------------------------
    |---------------0---------------|---------------1---------------| f
    | viim.s                        | vfim.s                        |
    |---------------|---------------|---------------|---------------|
*/

/*
viim.s
    31--------26-25-23-22---------16-15-----------------------------0
    |   VFPU5   |1 1|0|      vt     |              imm16            |
    ------6-------2--1-------7-----------------------16--------------
*/

    // TODO: vt, imm16
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0, viim_s, viim.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt, RAB_OPERAND_r4000allegrex_int16},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer immediate to float Single Word

/*
vfim.s
    31--------26-25----22---------16-15-----------------------------0
    |   VFPU5   |1 1|1|      vt     |              imm16            |
    ------6-------2--1-------7-----------------------16--------------
*/

    // TODO: vt, imm16
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1, vfim_s, vfim.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt, RAB_OPERAND_r4000allegrex_float16},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer immediate to float Single Word

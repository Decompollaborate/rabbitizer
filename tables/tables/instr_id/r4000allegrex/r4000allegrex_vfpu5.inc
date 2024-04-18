/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-24----------------------------------------------0
    |   VFPU5   |fmt|                                               |
    ------6-------2--------------------------------------------------
    |-------00------|-------01------|-------10------|-------11------| fmt
    | vpfxs         | vpfxt         | vpfxd         | *1            |
    |---------------|---------------|---------------|---------------|
     *1 See fmt3 table
*/

/*
vpfxs
    31--------26-----23---------------------------------------------0
    |   VFPU5   |0 0|                     imm24                     |
    ------6-------2-------------------------24-----------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, vpfxs,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Source Prefix

/*
vpfxt
    31--------26-----23---------------------------------------------0
    |   VFPU5   |0 1|                     imm24                     |
    ------6-------2-------------------------24-----------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, vpfxt,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Target Prefix

/*
vpfxd
    31--------26-----23---------------------------------------------0
    |   VFPU5   |1 0|                     imm24                     |
    ------6-------2-------------------------24-----------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, vpfxd,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Destination Prefix

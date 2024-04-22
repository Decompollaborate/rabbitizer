/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
vnop
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|
    ------6-------------------------------26-------------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x3FF0000, vnop,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Nop

/*
vsync
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 1 1 0 0 1 0 0 0 0 0|
    ------6-------------------------------26-------------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x3FF0320, vsync,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Synchronize

/*
vflush
    31--------26-25-------------------------------------------------0
    |   VFPU7   |1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 1 0 0 0 0 0 0 1 1 0 1|
    ------6-------------------------------26-------------------------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x3FF040D, vflush,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Flush Write Buffer

/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x02, wsbh,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // Word Swap Bytes within Halfword

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x03, wsbw,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // Word Swap Bytes within Word

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x10, seb,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // Sign-Extend Byte

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x18, seh,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // Sign-Extend Halfword

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x14, bitrev,
        .operands={RAB_OPERAND_cpu_rd, RAB_OPERAND_cpu_rt},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRd=true,
        .readsRt=true
    ) // BIT REVerse

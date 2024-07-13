/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21--------------------------------5--------0
    |  = COP0   |   TLB   |                                |  fmt   |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ---   | TLBR  | TLBWI | ---   | ---   | ---   | TLBWR | ---   |
001 | TLBP  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | RFE   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ERET  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------| 
*/

    // OP
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, tlbr,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .notEmittedByCompilers=true
    ) // Read Indexed TLB Entry
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, tlbwi,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .notEmittedByCompilers=true
    ) // Write Indexed TLB Entry

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x06, tlbwr,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    ) // Write Random TLB Entry

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x08, tlbp,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .notEmittedByCompilers=true
    ) // Probe TLB for Matching Entry

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x10, rfe,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .notEmittedByCompilers=true
    ) // Restore From Exception

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x18, eret,
        .operands={0},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .notEmittedByCompilers=true
    ) // Return from Exception

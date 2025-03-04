/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21--------------------------------5--------0
    |  = COP0   |   TLB   |                                |  fmt   |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | ---   |  TLBR | TLBWI | ---   | ---   | ---   | TLBWR | ---   |
001 | TLBP  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
010 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
011 | ERET  | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
100 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
101 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
110 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
111 | EI    | DI    | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // The other instructions are implemented using the main CORE table

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x38, ei, R5900EE,
        operands: Operand::arr0()
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x39, di, R5900EE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor0_tlb)

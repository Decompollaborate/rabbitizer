/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP0   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC0  | DMFC0 | CFC0  | ---   | MTC0  | DMTC0 | CTC0  | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
       *1 = BC See BC0 list
       *2 = TLB instr, see TLB list
*/

    // The other instructions are implemented using the main CORE table

    RAB_DEF_OPCODE_REDIRECT(0x8, none, coprocessor0_bc0)
    RAB_DEF_OPCODE_REDIRECT(0x10, r5900, coprocessor0_tlb)
    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor0)

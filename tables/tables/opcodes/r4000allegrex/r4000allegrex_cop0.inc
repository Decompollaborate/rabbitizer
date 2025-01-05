/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP0   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC0  | ---   | ---   | ---   | MTC0  | ---   | ---   | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
       *1 = BC See BC0 list
       *2 = TLB instr, see TLB list
*/

    // The other instructions are implemented using the main CORE table

    RAB_DEF_OPCODE_REMOVED(0x01, core, dmfc0, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    RAB_DEF_OPCODE_REMOVED(0x02, core, cfc0, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    RAB_DEF_OPCODE_REMOVED(0x05, core, dmtc0, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    RAB_DEF_OPCODE_REMOVED(0x06, core, ctc0, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)

    RAB_DEF_OPCODE_REDIRECT(0x08, r4000allegrex, coprocessor0_bc0)
    RAB_DEF_OPCODE_REDIRECT(0x10, r4000allegrex, coprocessor0_tlb)

    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor0)

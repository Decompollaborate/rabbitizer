/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP2   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC2  | ---   | CFC2  | *2    | MTC2  | ---   | CTC2  | *3    |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = See BC2 table
     *2 = See MFHC2 table
     *3 = See MTHC2 table
*/

    // The other instructions are implemented using the main CORE table

    RAB_DEF_OPCODE_REDIRECT(0x08, r4000allegrex, coprocessor2_bc2)
    RAB_DEF_OPCODE_REDIRECT(0x03, r4000allegrex, coprocessor2_mfhc2)
    RAB_DEF_OPCODE_REDIRECT(0x07, r4000allegrex, coprocessor2_mthc2)

    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor2)

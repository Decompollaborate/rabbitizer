/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----------------------------------------------1-0
    |  = COP2   |h|                                               |l|
    ------6------1-------------------------------------------------1-
    |---0---|---1---| l
  0 | *1    | *2    |
  1 | *3    | *3    |
  h |-------|-------|

    *1 = No interlock. See ni table.
    *2 = Interlock. See i table.
    *3 = See special1 table
*/

    RAB_DEF_OPCODE_REDIRECT(0x0000000, r5900ee, coprocessor2_ni)
    RAB_DEF_OPCODE_REDIRECT(0x0000001, r5900ee, coprocessor2_i)
    RAB_DEF_OPCODE_REDIRECT(0x2000000, r5900ee, coprocessor2_special1)
    RAB_DEF_OPCODE_REDIRECT(0x2000001, r5900ee, coprocessor2_special1)

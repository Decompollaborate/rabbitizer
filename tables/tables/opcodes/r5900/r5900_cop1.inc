/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP1   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC1  | DMFC1 | CFC1  | ---   | MTC1  | DMTC1 | CTC1  | ---   |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | *3    | ---   | ---   | *4    | *5    | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = BC instructions, see BC1 list
     *2 = S instr, see FPU S list
     *3 = D instr, see FPU D list
     *4 = W instr, see FPU W list
     *5 = L instr, see FPU L list
*/

    RAB_DEF_OPCODE_REDIRECT(0x8, none, coprocessor1_bc1)
    RAB_DEF_OPCODE_REDIRECT(0x10, r5900, coprocessor1_fpu_s)
    RAB_DEF_OPCODE_REDIRECT(0x11, none, coprocessor1_fpu_d)
    RAB_DEF_OPCODE_REDIRECT(0x14, none, coprocessor1_fpu_w)
    RAB_DEF_OPCODE_REDIRECT(0x15, none, coprocessor1_fpu_l)
    RAB_DEF_OPCODE_REDIRECT(_, none, coprocessor1)

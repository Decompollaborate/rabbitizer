/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP1   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC1  | ---   | CFC1  | MFHC1 | MTC1  | ---   | CTC1  | MTHC1 |
 01 | *1    | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *2    | ---   | ---   | ---   | *3    | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = BC See BC1 list
     *2 = S instr, see FPU S list
     *3 = W instr, see FPU W list
*/

/*
TODO:
- MFHC1
- MTHC1
*/

    // The other instructions are implemented using the main CPU table

/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7-6---4--------0
    |  = COP2   |  MFHC2  |                         |1| fmt |       |
    ------6----------5-------------------------------1---3-----------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
    | mfvc  | ---   | ---   | ---   | ---   | ---   | ---   | *1    |
    |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 See S list
*/

/*
mfvc
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 0 1 1|    rt   |0 0 0 0 0 0 0 0|1|    cop2cd   |
    ------6----------5---------5---------------------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x0, mfvc,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop2cd},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .modifiesRt=true
    ) // Move word From Vfpu Control

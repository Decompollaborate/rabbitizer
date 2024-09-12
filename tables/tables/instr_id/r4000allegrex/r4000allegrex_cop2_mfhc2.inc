/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7--------------0
    |  = COP2   |  MFHC2  |                         |p|             |
    ------6----------5-------------------------------1---------------
    |---------------0---------------|---------------1---------------| p
    | mfv                           | *1                            |
    |-------------------------------|-------------------------------|
     *1 = See P table
*/

/*
mfv
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 0 1 1|    rt   |0 0 0 0 0 0 0 0|0|      vd     |
    ------6----------5---------5---------------------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0, mfv,
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        modifies_rt: true
    ) // Move word From Vfpu

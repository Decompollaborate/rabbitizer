/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7--------------0
    |  = COP2   |  MFHC2  |                         |p|             |
    ------6----------5-------------------------------1---------------
    |---------------0---------------|---------------1---------------| p
    | mtv                           | mtvc                          |
    |-------------------------------|-------------------------------|
*/

/*
mtv
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 1 1 1|    rt   |0 0 0 0 0 0 0 0|0|      vd     |
    ------6----------5---------5---------------------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x0, mtv,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRt=true
    ) // Move word To Vfpu

/*
mtvc
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 1 1 1|    rt   |0 0 0 0 0 0 0 0|1|    cop2cd   |
    ------6----------5---------5---------------------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x1, mtvc,
        .operands={RAB_OPERAND_cpu_rt, RAB_OPERAND_r4000allegrex_cop2cd},
        .instrType=RABBITIZER_INSTR_TYPE_R,
        .readsRt=true
    ) // Move word To Vfpu Control

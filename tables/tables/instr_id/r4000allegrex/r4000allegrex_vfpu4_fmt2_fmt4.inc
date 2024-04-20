/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21-----------15--------------7--------------0
    |   VFPU4   |1 0|1 0 0|         |t|             |p|             |
    ------6-------2----3-------------1---------------1---------------
    |-------|
 00 | vi2f.s|
 01 | vi2f.p|
 10 | vi2f.t|
 11 | vi2f.q|
 tp |-------|
*/

/*
vi2f.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0, vi2f_s, vi2f.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer to float with Scaling Single Word

/*
vi2f.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1, vi2f_p, vi2f.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer to float with Scaling Single Pair

/*
vi2f.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2, vi2f_t, vi2f.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer to float with Scaling Single Triple

/*
vi2f.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3, vi2f_q, vi2f.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer to float with Scaling Single Quad

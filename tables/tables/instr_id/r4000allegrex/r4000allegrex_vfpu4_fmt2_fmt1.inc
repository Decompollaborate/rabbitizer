/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21-----------15--------------7--------------0
    |   VFPU4   |1 0|0 0 1|         |t|             |p|             |
    ------6-------2----3-------------1---------------1---------------
    |-------|
 00 |vf2iz.s|
 01 |vf2iz.p|
 10 |vf2iz.t|
 11 |vf2iz.q|
 tp |-------|
*/

/*
vf2iz.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0, vf2iz_s, vf2iz.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to zero integer from float with Scaling Single Word

/*
vf2iz.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1, vf2iz_p, vf2iz.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to zero integer from float with Scaling Single Pair

/*
vf2iz.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2, vf2iz_t, vf2iz.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to zero integer from float with Scaling Single Triple

/*
vf2iz.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3, vf2iz_q, vf2iz.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to zero integer from float with Scaling Single Quad

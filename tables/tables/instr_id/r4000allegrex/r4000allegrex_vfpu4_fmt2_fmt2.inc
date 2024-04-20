/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21-----------15--------------7--------------0
    |   VFPU4   |1 0|0 1 0|         |t|             |p|             |
    ------6-------2----3-------------1---------------1---------------
    |-------|
 00 |vf2iu.s|
 01 |vf2iu.p|
 10 |vf2iu.t|
 11 |vf2iu.q|
 tp |-------|
*/

/*
vf2iu.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0, vf2iu_s, vf2iu.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to smallest integer from float with Scaling Single Word

/*
vf2iu.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1, vf2iu_p, vf2iu.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to smallest integer from float with Scaling Single Pair

/*
vf2iu.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2, vf2iu_t, vf2iu.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to smallest integer from float with Scaling Single Triple

/*
vf2iu.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3, vf2iu_q, vf2iu.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to smallest integer from float with Scaling Single Quad

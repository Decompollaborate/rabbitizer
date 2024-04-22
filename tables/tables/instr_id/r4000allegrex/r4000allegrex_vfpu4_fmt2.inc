/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21------------------------------------------0
    |   VFPU4   |1 0| fmt |         |t|             |p|             |
    ------6-------2----3---------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 |vf2in.s|vf2iz.s|vf2iu.s|vf2id.s| vi2f.s| *1    | ---   | ---   |
 01 |vf2in.p|vf2iz.p|vf2iu.p|vf2id.p| vi2f.p| *1    | ---   | ---   |
 10 |vf2in.t|vf2iz.t|vf2iu.t|vf2id.t| vi2f.t| *1    | ---   | ---   |
 11 |vf2in.q|vf2iz.q|vf2iu.q|vf2id.q| vi2f.q| *1    | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = See CNDMOVE table
*/

/*
vf2in.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x0, vf2in_s, vf2in.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to nearest integer from float with Scaling Single Word

/*
vf2in.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x1, vf2in_p, vf2in.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to nearest integer from float with Scaling Single Pair

/*
vf2in.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x2, vf2in_t, vf2in.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to nearest integer from float with Scaling Single Triple

/*
vf2in.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x3, vf2in_q, vf2in.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to nearest integer from float with Scaling Single Quad

/*
vf2iz.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x0, vf2iz_s, vf2iz.s,
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
        r4000allegrex, 0x1 << 2 | 0x1, vf2iz_p, vf2iz.p,
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
        r4000allegrex, 0x1 << 2 | 0x2, vf2iz_t, vf2iz.t,
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
        r4000allegrex, 0x1 << 2 | 0x3, vf2iz_q, vf2iz.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to zero integer from float with Scaling Single Quad

/*
vf2iu.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x0, vf2iu_s, vf2iu.s,
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
        r4000allegrex, 0x2 << 2 | 0x1, vf2iu_p, vf2iu.p,
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
        r4000allegrex, 0x2 << 2 | 0x2, vf2iu_t, vf2iu.t,
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
        r4000allegrex, 0x2 << 2 | 0x3, vf2iu_q, vf2iu.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to smallest integer from float with Scaling Single Quad

/*
vf2id.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x0, vf2id_s, vf2id.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to largest integer from float with Scaling Single Word

/*
vf2id.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x1, vf2id_p, vf2id.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to largest integer from float with Scaling Single Pair

/*
vf2id.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x2, vf2id_t, vf2id.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to largest integer from float with Scaling Single Triple

/*
vf2id.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x3, vf2id_q, vf2id.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Round to largest integer from float with Scaling Single Quad

/*
vi2f.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x4 << 2 | 0x0, vi2f_s, vi2f.s,
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
        r4000allegrex, 0x4 << 2 | 0x1, vi2f_p, vi2f.p,
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
        r4000allegrex, 0x4 << 2 | 0x2, vi2f_t, vi2f.t,
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
        r4000allegrex, 0x4 << 2 | 0x3, vi2f_q, vi2f.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_power_of_two},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Convert integer to float with Scaling Single Quad

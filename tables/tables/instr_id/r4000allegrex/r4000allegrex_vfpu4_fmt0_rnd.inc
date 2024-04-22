/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 1 0 0| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 |vrnds.s|vrndi.s|vrndf1.s|vrndf2.s|---   |---   | ---   | ---   |
 01 | ---   |vrndi.p|vrndf1.p|vrndf2.p|---   |---   | ---   | ---   |
 10 | ---   |vrndi.t|vrndf1.t|vrndf2.t|---   |---   | ---   | ---   |
 11 | ---   |vrndi.q|vrndf1.q|vrndf2.q|---   |---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vrnds.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 0|0|      vs     |0|0 0 0 0 0 0 0|
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x0, vrnds_s, vrnds.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Seed Single word

/*
vrndi.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x0, vrndi_s, vrndi.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Integer Single word

/*
vrndi.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vrndi_p, vrndi.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Integer Pair word

/*
vrndi.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x2, vrndi_t, vrndi.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Integer Triple word

/*
vrndi.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vrndi_q, vrndi.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Integer Quad word

/*
vrndf1.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x0, vrndf1_s, vrndf1.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Single word

/*
vrndf1.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x1, vrndf1_p, vrndf1.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Pair word

/*
vrndf1.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x2, vrndf1_t, vrndf1.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Triple word

/*
vrndf1.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x3, vrndf1_q, vrndf1.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Quad word

/*
vrndf2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x0, vrndf2_s, vrndf2.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Pair word

/*
vrndf2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x1, vrndf2_p, vrndf2.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Pair word

/*
vrndf2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x2, vrndf2_t, vrndf2.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Quad word

/*
vrndf2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x3, vrndf2_q, vrndf2.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // RaNDom Floating Quad word

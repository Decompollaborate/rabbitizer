/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 0 0 0| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vmov.s| vabs.s| vneg.s| ---   |vsat0.s|vsat1.s|vzero.s| vone.s|
 01 | vmov.p| vabs.p| vneg.p| vidt.p|vsat0.p|vsat1.p|vzero.p| vone.p|
 10 | vmov.t| vabs.t| vneg.t| ---   |vsat0.t|vsat1.t|vzero.t| vone.t|
 11 | vmov.q| vabs.q| vneg.q| vidt.q|vsat0.q|vsat1.q|vzero.q| vone.q|
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vmov.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x0, vmov_s, vmov.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MOVe Single word

/*
vmov.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x1, vmov_p, vmov.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MOVe Pair word

/*
vmov.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x2, vmov_t, vmov.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MOVe Triple word

/*
vmov.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vmov_q, vmov.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MOVe Quad word

/*
vabs.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x0, vabs_s, vabs.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ABSolute value Single word

/*
vabs.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vabs_p, vabs.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ABSolute value Pair word

/*
vabs.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x2, vabs_t, vabs.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ABSolute value Triple word

/*
vabs.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vabs_q, vabs.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ABSolute value Quad word

/*
vneg.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x0, vneg_s, vneg.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // NEGate Single word

/*
vneg.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x1, vneg_p, vneg.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // NEGate Pair word

/*
vneg.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x2, vneg_t, vneg.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // NEGate Triple word

/*
vneg.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x3, vneg_q, vneg.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // NEGate Quad word

/*
vidt.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x1, vidt_p, vidt.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // IDenTity Pair word

/*
vidt.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|0 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x3, vidt_q, vidt.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // IDenTity Quad word

/*
vsat0.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x0, vsat0_s, vsat0.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [0.0:1.0] Single word

/*
vsat0.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x1, vsat0_p, vsat0.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [0.0:1.0] Pair word

/*
vsat0.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x2, vsat0_t, vsat0.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [0.0:1.0] Triple word

/*
vsat0.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x3, vsat0_q, vsat0.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [0.0:1.0] Quad word

/*
vsat1.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x0, vsat1_s, vsat1.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [-1.0:1.0] Single word

/*
vsat1.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x1, vsat1_p, vsat1.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [-1.0:1.0] Pair word

/*
vsat1.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x2, vsat1_t, vsat1.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [-1.0:1.0] Triple word

/*
vsat1.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x3, vsat1_q, vsat1.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SATurate to [-1.0:1.0] Quad word

/*
vzero.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 0|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x0, vzero_s, vzero.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // set ZERO Single word

/*
vzero.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 0|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x1, vzero_p, vzero.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // set ZERO Pair word

/*
vzero.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 0|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x2, vzero_t, vzero.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // set ZERO Triple word

/*
vzero.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 0|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x3, vzero_q, vzero.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // set ZERO Quad word

/*
vone.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 1|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x0, vone_s, vone.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One's ComPlement Single word

/*
vone.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x1, vone_p, vone.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One's ComPlement Pair word

/*
vone.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x2, vone_t, vone.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One's ComPlement Triple word

/*
vone.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|0 0|1 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x3, vone_q, vone.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One's ComPlement Quad word

/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 1 0 0 1| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | ---   | ---   | vsgn.s| ---   | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | vsgn.p| ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | vsgn.t| ---   | ---   | ---   | ---   | ---   |
 11 |vsrt3.q|vsrt4.q| vsgn.q| ---   | ---   | ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vsrt3.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vsrt3_q, vsrt3.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SoRT 3 Quad word

/*
vsrt4.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vsrt4_q, vsrt4.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SoRT 4 Quad word

/*
vsgn.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x0, vsgn_s, vsgn.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SiGN Single word

/*
vsgn.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x1, vsgn_p, vsgn.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SiGN Pair word

/*
vsgn.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x2, vsgn_t, vsgn.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SiGN Triple word

/*
vsgn.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x3, vsgn_q, vsgn.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SiGN Quad word

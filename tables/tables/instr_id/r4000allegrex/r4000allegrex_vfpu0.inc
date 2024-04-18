/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU0   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vadd.s| vsub.s| vsbn.s| ---   | ---   | ---   | ---   | vdiv.s|
 01 | vadd.p| vsub.p| ---   | ---   | ---   | ---   | ---   | ---   |
 10 | vadd.t| vsub.t| ---   | ---   | ---   | ---   | ---   | ---   |
 11 | vadd.q| vsub.q| ---   | ---   | ---   | ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vd, vs, vt

/*
vadd.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x0, vadd_s, vadd.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Single word

/*
vadd.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x1, vadd_p, vadd.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Pair word

/*
vadd.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x2, vadd_t, vadd.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Triple word

/*
vadd.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vadd_q, vadd.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Quad word

/*
vsub.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x0, vsub_s, vsub.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SUBtract Single word

/*
vsub.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vsub_p, vsub.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Pair word

/*
vsub.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x2, vsub_t, vsub.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Triple word

/*
vsub.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vsub_q, vsub.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ADD Quad word

/*
vsbn.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x0, vsbn_s, vsbn.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // ScaleBN Single word

/*
vdiv.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x0, vdiv_s, vdiv.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DIVide Single word

/*
vdiv.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x1, vdiv_p, vdiv.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DIVide Single word

/*
vdiv.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x2, vdiv_t, vdiv.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DIVide Single word

/*
vdiv.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x3, vdiv_q, vdiv.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DIVide Single word

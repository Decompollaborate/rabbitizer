/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU3   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vcmp.s| ---   | vmin.s| vmax.s| ---   |vscmp.s| vsge.s| vslt.s|
 01 | vcmp.p| ---   | vmin.p| vmax.p| ---   |vscmp.p| vsge.p| vslt.p|
 10 | vcmp.t| ---   | vmin.t| vmax.t| ---   |vscmp.t| vsge.t| vslt.t|
 11 | vcmp.q| ---   | vmin.q| vmax.q| ---   |vscmp.q| vsge.q| vslt.q|
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vcmp.s
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |0|      vs     |0|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x0, vcmp_s, vcmp.s,
        .operands={RAB_OPERAND_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // CoMPare Single word

/*
vcmp.p
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |0|      vs     |1|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x1, vcmp_p, vcmp.p,
        .operands={RAB_OPERAND_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // CoMPare Pair word

/*
vcmp.t
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |1|      vs     |0|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x2, vcmp_t, vcmp.t,
        .operands={RAB_OPERAND_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // CoMPare Triple word

/*
vcmp.q
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |1|      vs     |1|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x3, vcmp_q, vcmp.q,
        .operands={RAB_OPERAND_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // CoMPare Quad word

/*
vmin.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x0, vmin_s, vmin.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MINimum Single word

/*
vmin.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x1, vmin_p, vmin.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MINimum Pair word

/*
vmin.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x2, vmin_t, vmin.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MINimum Triple word

/*
vmin.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x3, vmin_q, vmin.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MINimum Quad word

/*
vmax.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x0, vmax_s, vmax.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MAXimum Single word

/*
vmax.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x1, vmax_p, vmax.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MAXimum Pair word

/*
vmax.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x2, vmax_t, vmax.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MAXimum Triple word

/*
vmax.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x3, vmax_q, vmax.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MAXimum Quad word

/*
vscmp.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x5 << 2 | 0x0, vscmp_s, vscmp.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Set ComPare Single word

/*
vscmp.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/


    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x5 << 2 | 0x1, vscmp_p, vscmp.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Set ComPare Pair word

/*
vscmp.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x5 << 2 | 0x2, vscmp_t, vscmp.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Set ComPare Triple word

/*
vscmp.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x5 << 2 | 0x3, vscmp_q, vscmp.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Set ComPare Quad word

/*
vsge.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6 << 2 | 0x0, vsge_s, vsge.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Greater Equal Single word to value

/*
vsge.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6 << 2 | 0x1, vsge_p, vsge.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Greater Equal Pair word to value

/*
vsge.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6 << 2 | 0x2, vsge_t, vsge.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Greater Equal Triple word to value

/*
vsge.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6 << 2 | 0x3, vsge_q, vsge.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // CGreater Equal Quad word to value

/*
vslt.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x7 << 2 | 0x0, vslt_s, vslt.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Less Than Single word to value

/*
vslt.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x7 << 2 | 0x1, vslt_p, vslt.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Less Than Pair word to value

/*
vslt.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x7 << 2 | 0x2, vslt_t, vslt.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Less Than Triple word to value

/*
vslt.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x7 << 2 | 0x3, vslt_q, vslt.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Less Than Quad word to value

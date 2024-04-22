/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU0   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vmul.s| ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | vmul.p| vdot.p| vscl.p| ---   | vhdp.p| ---   | vdet.p| ---   |
 10 | vmul.t| vdot.t| vscl.t| ---   | vhdp.t| vcrs.t| ---   | ---   |
 11 | vmul.q| vdot.q| vscl.q| ---   | vhdp.q| ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vd, vs, vt

/*
vmul.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x0, vmul_s, vmul.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MULtiply Single word

/*
vmul.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x1, vmul_p, vmul.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MULtiply Pair word

/*
vmul.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x2, vmul_t, vmul.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MULtiply Triple word

/*
vmul.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x3, vmul_q, vmul.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // MULtiply Quad word

/*
vdot.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x1, vdot_p, vdot.p,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DOT Product Pair word

/*
vdot.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x2, vdot_t, vdot.t,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DOT Product Triple word

/*
vdot.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x3, vdot_q, vdot.q,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DOT Product Quad word

/*
vscl.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x1, vscl_p, vscl.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DOT Product Pair word

/*
vscl.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x2, vscl_t, vscl.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // DOT Product Triple word

/*
vscl.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x2 << 2 | 0x3, vscl_q, vscl.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_s_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // SCaLe Quad word

/*
vhdp.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x4 << 2 | 0x1, vhdp_p, vhdp.p,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Homogeneous Dot Product Pair word

/*
vhdp.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x4 << 2 | 0x2, vhdp_t, vhdp.t,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Homogeneous Dot Product Pair word

/*
vhdp.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x4 << 2 | 0x3, vhdp_q, vhdp.q,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_q_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Homogeneous Dot Product Pair word

/*
vcrs.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x5 << 2 | 0x2, vcrs_t, vcrs.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_t_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Cross Triple Word

/*
vdet.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x6 << 2 | 0x1, vdet_p, vdet.p,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_p_vt},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // 2x2 matrix DETerminant

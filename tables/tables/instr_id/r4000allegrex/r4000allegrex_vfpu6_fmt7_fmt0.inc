/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26---------------18-16-15--------------7--------------0
    |   VFPU6   |1 1 1|0 0|   | fmt |t|             |p|             |
    ------6--------3----2--------3---1---------------1---------------
    |---000--|---001--|---010--|---011--|---100--|---101--|---110--|---111--| fmt
  00| ---    | ---    | ---    | ---    | ---    | ---    | ---    |  ---   |
  01| vmmov.p| ---    | ---    | vmidt.p| ---    | ---    |vmzero.p| vmone.p|
  10| vmmov.t| ---    | ---    | vmidt.t| ---    | ---    |vmzero.t| vmone.t|
  11| vmmov.q| ---    | ---    | vmidt.q| ---    | ---    |vmzero.q| vmone.q|
  tp|--------|--------|--------|--------|--------|--------|--------|--------|
*/

/*
vmmov.p
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 0 0|0|      vs     |1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x1, vmmov_p, vmmov.p,
        .operands={RAB_OPERAND_r4000allegrex_mp_vd, RAB_OPERAND_r4000allegrex_mp_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Move Pair x Pair Matrix

/*
vmmov.t
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 0 0|1|      vs     |0|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x2, vmmov_t, vmmov.t,
        .operands={RAB_OPERAND_r4000allegrex_mt_vd, RAB_OPERAND_r4000allegrex_mt_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Move Triple x Triple Matrix

/*
vmmov.q
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 0 0|1|      vs     |1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/


    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vmmov_q, vmmov.q,
        .operands={RAB_OPERAND_r4000allegrex_mq_vd, RAB_OPERAND_r4000allegrex_mq_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Move Quad x Quad Matrix

/*
vmidt.p
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x1, vmidt_p, vmidt.p,
        .operands={RAB_OPERAND_r4000allegrex_mp_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Identity Pair x Pair Matrix

/*
vmidt.t
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 1 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x2, vmidt_t, vmidt.t,
        .operands={RAB_OPERAND_r4000allegrex_mt_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Identity Triple x Triple Matrix

/*
vmidt.q
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|0 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x3, vmidt_q, vmidt.q,
        .operands={RAB_OPERAND_r4000allegrex_mq_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Identity Quad x Quad Matrix

/*
vmzero.p
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 0|0|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x1, vmzero_p, vmzero.p,
        .operands={RAB_OPERAND_r4000allegrex_mp_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Zero Pair x Pair Matrix

/*
vmzero.t
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 0|1|0 0 0 0 0 0 0|0|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x2, vmzero_t, vmzero.t,
        .operands={RAB_OPERAND_r4000allegrex_mt_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Zero Triple x Triple Matrix

/*
vmzero.q
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 0|1|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x3, vmzero_q, vmzero.q,
        .operands={RAB_OPERAND_r4000allegrex_mq_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Zero Quad x Quad Matrix

/*
vmone.p
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x1, vmone_p, vmone.p,
        .operands={RAB_OPERAND_r4000allegrex_mp_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One Pair x Pair Matrix

/*
vmone.t
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x2, vmone_t, vmone.t,
        .operands={RAB_OPERAND_r4000allegrex_mt_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One Triple x Triple Matrix

/*
vmone.q
    31--------26-25------------18-16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 0|0 0|1 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6----------------------3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x3, vmone_q, vmone.q,
        .operands={RAB_OPERAND_r4000allegrex_mq_vd},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // One Quad x Quad Matrix

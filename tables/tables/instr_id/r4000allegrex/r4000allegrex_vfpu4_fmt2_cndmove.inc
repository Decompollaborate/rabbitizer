/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21--19-------15--------------7--------------0
    |   VFPU4   |1 0|1 0 1|fmt|     |t|             |p|             |
    ------6-------2----3----2--------1---------------1---------------
    |-------00------|-------01------|-------10------|-------11------| fmt
 00 | vcmovt.s      | vcmovf.s      | ---           | ---           |
 01 | vcmovt.p      | vcmovf.p      | ---           | ---           |
 10 | vcmovt.q      | vcmovf.q      | ---           | ---           |
 11 | vcmovt.t      | vcmovf.t      | ---           | ---           |
 tp |---------------|---------------|---------------|---------------|
*/

/*
vcmovt.s
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x0, vcmovt_s, vcmovt.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on True Single Word

/*
vcmovt.p
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x1, vcmovt_p, vcmovt.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on True Single Pair

/*
vcmovt.t
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x2, vcmovt_t, vcmovt.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on True Single Triple

/*
vcmovt.q
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x0 << 2 | 0x3, vcmovt_q, vcmovt.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on True Single Quad

/*
vcmovf.s
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x0, vcmovf_s, vcmovf.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on False Single Word

/*
vcmovf.p
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x1, vcmovf_p, vcmovf.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_p_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on False Single Pair

/*
vcmovf.t
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x2, vcmovf_t, vcmovf.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_t_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on False Single Triple

/*
vcmovf.q
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x1 << 2 | 0x3, vcmovf_q, vcmovf.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_q_vs, RAB_OPERAND_r4000allegrex_vfpu_cc_bit},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Conditional Move on False Single Quad

/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26--------21-----------15--------------7--------------0
    |   VFPU6   |1 1 1|fmt|         |t|             |p|             |
    ------6--------3----2------------1---------------1---------------
    |-------00------|-------01------|-------10------|-------11------| fmt
  00| *1            | ---           | ---           | ---           |
  01| *1            | vrot.p        | ---           | ---           |
  10| *1            | vrot.t        | ---           | ---           |
  11| *1            | vrot.q        | ---           | ---           |
  tp|---------------|---------------|---------------|---------------|
     *1 See fmt7 table
*/

/*
vrot.p
    31--------26-25-----21-20-----16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 1|   imm5  |0|      vs     |1|      vd     |
    ------6--------3----2------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vrot_p, vrot.p,
        .operands={RAB_OPERAND_r4000allegrex_p_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_p_vrot_code},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Rotator Pair word

/*
vrot.t
    31--------26-25-----21-20-----16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 1|   imm5  |1|      vs     |0|      vd     |
    ------6--------3----2------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x2, vrot_t, vrot.t,
        .operands={RAB_OPERAND_r4000allegrex_t_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_t_vrot_code},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Rotator Triple word

/*
vrot.q
    31--------26-25-----21-20-----16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 1|   imm5  |1|      vs     |1|      vd     |
    ------6--------3----2------5-----1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vrot_q, vrot.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vd, RAB_OPERAND_r4000allegrex_s_vs, RAB_OPERAND_r4000allegrex_q_vrot_code},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // Rotator Quad word

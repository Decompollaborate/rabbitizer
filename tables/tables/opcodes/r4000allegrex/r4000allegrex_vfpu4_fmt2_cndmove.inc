/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
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

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x00000, vcmovt_s, vcmovt.s, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on True Single Word

/*
vcmovt.p
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x00001, vcmovt_p, vcmovt.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on True Single Pair

/*
vcmovt.t
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x00100, vcmovt_t, vcmovt.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on True Single Triple

/*
vcmovt.q
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 0| imm3|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x00101, vcmovt_q, vcmovt.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on True Single Quad

/*
vcmovf.s
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x01000, vcmovf_s, vcmovf.s, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on False Single Word

/*
vcmovf.p
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x01001, vcmovf_p, vcmovf.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on False Single Pair

/*
vcmovf.t
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x01100, vcmovf_t, vcmovf.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on False Single Triple

/*
vcmovf.q
    31--------26-25-24--21--19-18-16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 1|0 1| imm3|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT2_CNDMOVE, 0x01101, vcmovf_q, vcmovf.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_vfpu_cc_bit),
        instr_type: InstrType::R
    ) // Conditional Move on False Single Quad

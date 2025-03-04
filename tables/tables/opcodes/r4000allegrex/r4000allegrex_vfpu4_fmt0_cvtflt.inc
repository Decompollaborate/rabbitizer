/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 1 1 0| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | ---   | ---   | ---   | vh2f.s| ---   | ---   | vsbz.s| vlgb.s|
 01 | ---   | ---   | vf2h.p| vh2f.p| ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | vf2h.q| ---   | ---   | ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vf2h.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0401, vf2h_p, vf2h.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert float to float16 Pair word

/*
vf2h.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0501, vf2h_q, vf2h.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Convert float to float16 Quad word

/*
vh2f.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|0 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0600, vh2f_s, vh2f.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Convert float16 to float Pair word

/*
vh2f.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|0 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0601, vh2f_p, vh2f.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert float16 to float Pair word

/*
vsbz.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|1 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0C00, vsbz_s, vsbz.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // ScaleBZ Pair word

/*
vlgb.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 0|1 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x0E00, vlgb_s, vlgb.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // LoGB Pair word

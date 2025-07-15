/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 1 1 1| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00|vuc2ifs.s|vc2i.s|vus2i.s| vs2i.s| ---   | ---   | ---   | ---   |
 01 | ---   | ---   |vus2i.p| vs2i.p| ---   | ---   |vi2us.p| vi2s.p|
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   |vi2uc.q| vi2c.q|vi2us.q| vi2s.q|
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vuc2ifs.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0000, vuc2ifs_s, vuc2ifs.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // convert Unsigned Char single word to Integer

/*
vc2i.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0200, vc2i_s, vc2i.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Convert signed char single word to integer

/*
vus2i.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0400, vus2i_s, vus2i.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Convert unsigned short to integer Single Word

/*
vus2i.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0401, vus2i_p, vus2i.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert unsigned short to integer Pair Word

/*
vs2i.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0600, vs2i_s, vs2i.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Convert signed short to integer Single Word

/*
vs2i.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|0 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0601, vs2i_p, vs2i.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert signed short to integer Single Word

/*
vi2uc.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0901, vi2uc_q, vi2uc.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Convert integer to unsigned char Quad Word

/*
vi2c.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0B01, vi2c_q, vi2c.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Convert integer to signed char Quad Word

/*
vi2us.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0C01, vi2us_p, vi2us.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert integer to unsigned short Pair Word

/*
vi2us.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0D01, vi2us_q, vi2us.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Convert integer to unsigned short Quad Word

/*
vi2s.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0E01, vi2s_p, vi2s.p, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Convert integer to signed short Pair Word

/*
vi2s.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|1 1|1 1 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_VFPU4_FMT0_CVTINT, 0x0F01, vi2s_q, vi2s.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Convert integer to signed short Quad Word

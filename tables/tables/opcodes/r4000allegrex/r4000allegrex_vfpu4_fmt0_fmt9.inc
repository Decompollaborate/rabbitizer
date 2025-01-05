/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
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

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0101, vsrt3_q, vsrt3.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SoRT 3 Quad word

/*
vsrt4.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0301, vsrt4_q, vsrt4.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SoRT 4 Quad word

/*
vsgn.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0400, vsgn_s, vsgn.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // SiGN Single word

/*
vsgn.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0401, vsgn_p, vsgn.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // SiGN Pair word

/*
vsgn.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0500, vsgn_t, vsgn.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // SiGN Triple word

/*
vsgn.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|0 1|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0501, vsgn_q, vsgn.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SiGN Quad word

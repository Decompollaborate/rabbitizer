/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 1 0 0| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 |vrnds.s|vrndi.s|vrndf1.s|vrndf2.s|---   |---   | ---   | ---   |
 01 | ---   |vrndi.p|vrndf1.p|vrndf2.p|---   |---   | ---   | ---   |
 10 | ---   |vrndi.t|vrndf1.t|vrndf2.t|---   |---   | ---   | ---   |
 11 | ---   |vrndi.q|vrndf1.q|vrndf2.q|---   |---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vrnds.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 0|0|      vs     |0|0 0 0 0 0 0 0|
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0000, vrnds_s, vrnds.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // RaNDom Seed Single word

/*
vrndi.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0200, vrndi_s, vrndi.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R
    ) // RaNDom Integer Single word

/*
vrndi.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0201, vrndi_p, vrndi.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R
    ) // RaNDom Integer Pair word

/*
vrndi.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0300, vrndi_t, vrndi.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R
    ) // RaNDom Integer Triple word

/*
vrndi.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 0 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0301, vrndi_q, vrndi.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R
    ) // RaNDom Integer Quad word

/*
vrndf1.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0400, vrndf1_s, vrndf1.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Single word

/*
vrndf1.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0401, vrndf1_p, vrndf1.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Pair word

/*
vrndf1.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0500, vrndf1_t, vrndf1.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Triple word

/*
vrndf1.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 0|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0501, vrndf1_q, vrndf1.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Quad word

/*
vrndf2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0600, vrndf2_s, vrndf2.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Pair word

/*
vrndf2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0601, vrndf2_p, vrndf2.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Pair word

/*
vrndf2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0700, vrndf2_t, vrndf2.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Quad word

/*
vrndf2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 1|0 0|0 1 1|1|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0701, vrndf2_q, vrndf2.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R
    ) // RaNDom Floating Quad word

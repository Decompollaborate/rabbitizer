/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU0   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vadd.s| vsub.s| vsbn.s| ---   | ---   | ---   | ---   | vdiv.s|
 01 | vadd.p| vsub.p| ---   | ---   | ---   | ---   | ---   | ---   |
 10 | vadd.t| vsub.t| ---   | ---   | ---   | ---   | ---   | ---   |
 11 | vadd.q| vsub.q| ---   | ---   | ---   | ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vd, vs, vt

/*
vadd.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00000, vadd_s, vadd.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // ADD Single word

/*
vadd.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00001, vadd_p, vadd.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // ADD Pair word

/*
vadd.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00100, vadd_t, vadd.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // ADD Triple word

/*
vadd.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00101, vadd_q, vadd.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // ADD Quad word

/*
vsub.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10000, vsub_s, vsub.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // SUBtract Single word

/*
vsub.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10001, vsub_p, vsub.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // ADD Pair word

/*
vsub.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10100, vsub_t, vsub.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // ADD Triple word

/*
vsub.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10101, vsub_q, vsub.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // ADD Quad word

/*
vsbn.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |0 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x20000, vsbn_s, vsbn.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // ScaleBN Single word

/*
vdiv.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70000, vdiv_s, vdiv.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // DIVide Single word

/*
vdiv.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70001, vdiv_p, vdiv.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // DIVide Single word

/*
vdiv.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70100, vdiv_t, vdiv.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // DIVide Single word

/*
vdiv.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU0   |1 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70101, vdiv_q, vdiv.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // DIVide Single word

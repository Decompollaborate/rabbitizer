/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU6   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |---000--|---001--|---010--|---011--|---100--|---101--|---110--|---111--| fmt
  00| ---    |vhtfm2.p| ---    | ---    | ---    | ---    | ---    | *1     |
  01| vmmul.p| vtfm2.p|vhtfm3.t| ---    | vmscl.p| ---    | ---    | *1     |
  10| vmmul.t| ---    | vtfm3.t|vhtfm4.q| vmscl.t| vcrsp.t| ---    | *1     |
  11| vmmul.q| ---    | ---    | vtfm4.q| vmscl.q| vqmul.q| ---    | *1     |
  tp|--------|--------|--------|--------|--------|--------|--------|--------|
     *1 See fmt7 table
*/

/*
vmmul.p
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 0 0|      vt     |0|     vs^t    |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x1, vmmul_p, vmmul.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs_transpose, Operand::r4000allegrex_mp_vt),
        instr_type: InstrType::R
    ) // Multiply Pair x Pair Matrix

/*
vmmul.t
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 0 0|      vt     |1|     vs^t    |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x2, vmmul_t, vmmul.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs_transpose, Operand::r4000allegrex_mt_vt),
        instr_type: InstrType::R
    ) // Multiply Triple x Triple Matrix

/*
vmmul.q
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 0 0|      vt     |1|     vs^t    |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vmmul_q, vmmul.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs_transpose, Operand::r4000allegrex_mq_vt),
        instr_type: InstrType::R
    ) // Multiply Quad x Quad Matrix

/*
vhtfm2.p
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 0 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x0, vhtfm2_p, vhtfm2.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_mp_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Homogeneous Transform 2 Pair Word

/*
vtfm2.p
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vtfm2_p, vtfm2.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_mp_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Transform 2 Pair Word

/*
vhtfm3.t
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x1, vhtfm3_t, vhtfm3.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_mt_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Homogeneous Transform 3 Triple Word

/*
vtfm3.t
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x2, vtfm3_t, vtfm3.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_mt_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Transform 3 Triple Word

/*
vhtfm4.q
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x2, vhtfm4_q, vhtfm4.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_mq_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Homogeneous Transform 4 Quad Word

/*
vtfm4.q
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |0 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x3, vtfm4_q, vtfm4.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_mq_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Transform 4 Quad Word

/*
vmscl.p
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |1 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x1, vmscl_p, vmscl.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Scale Pair x Pair Matrix

/*
vmscl.t
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |1 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x2, vmscl_t, vmscl.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Scale Triple x Triple Matrix

/*
vmscl.q
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |1 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x3, vmscl_q, vmscl.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Scale Quad x Quad Matrix

/*
vcrsp.t
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |1 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x2, vcrsp_t, vcrsp.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Cross Product Triple Word

/*
vqmul.q
    31--------26-25----22---------16---14----------8---6------------0
    |   VFPU6   |1 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x3, vqmul_q, vqmul.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Quaternion Multiply Quad Word

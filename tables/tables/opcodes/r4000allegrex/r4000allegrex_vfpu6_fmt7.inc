/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
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

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04001, vrot_p, vrot.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_p_vrot_code),
        instr_type: InstrType::R
    ) // Rotator Pair word

/*
vrot.t
    31--------26-25-----21-20-----16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 1|   imm5  |1|      vs     |0|      vd     |
    ------6--------3----2------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04100, vrot_t, vrot.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_t_vrot_code),
        instr_type: InstrType::R
    ) // Rotator Triple word

/*
vrot.q
    31--------26-25-----21-20-----16---14----------8---6------------0
    |   VFPU6   |1 1 1|0 1|   imm5  |1|      vs     |1|      vd     |
    ------6--------3----2------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04101, vrot_q, vrot.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_q_vrot_code),
        instr_type: InstrType::R
    ) // Rotator Quad word

    RAB_DEF_OPCODE_REDIRECT(0x000, r4000allegrex, vfpu6_fmt7_fmt0)
    RAB_DEF_OPCODE_REDIRECT(0x001, r4000allegrex, vfpu6_fmt7_fmt0)
    RAB_DEF_OPCODE_REDIRECT(0x100, r4000allegrex, vfpu6_fmt7_fmt0)
    RAB_DEF_OPCODE_REDIRECT(0x101, r4000allegrex, vfpu6_fmt7_fmt0)

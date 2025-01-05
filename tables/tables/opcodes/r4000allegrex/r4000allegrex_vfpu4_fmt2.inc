/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21------------------------------------------0
    |   VFPU4   |1 0| fmt |         |t|             |p|             |
    ------6-------2----3---------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 |vf2in.s|vf2iz.s|vf2iu.s|vf2id.s| vi2f.s| *1    | ---   | ---   |
 01 |vf2in.p|vf2iz.p|vf2iu.p|vf2id.p| vi2f.p| *1    | ---   | ---   |
 10 |vf2in.t|vf2iz.t|vf2iu.t|vf2id.t| vi2f.t| *1    | ---   | ---   |
 11 |vf2in.q|vf2iz.q|vf2iu.q|vf2id.q| vi2f.q| *1    | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = See CNDMOVE table
*/

/*
vf2in.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00000, vf2in_s, vf2in.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to nearest integer from float with Scaling Single Word

/*
vf2in.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00001, vf2in_p, vf2in.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to nearest integer from float with Scaling Single Pair

/*
vf2in.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00100, vf2in_t, vf2in.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to nearest integer from float with Scaling Single Triple

/*
vf2in.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00101, vf2in_q, vf2in.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to nearest integer from float with Scaling Single Quad

/*
vf2iz.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04000, vf2iz_s, vf2iz.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to zero integer from float with Scaling Single Word

/*
vf2iz.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04001, vf2iz_p, vf2iz.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to zero integer from float with Scaling Single Pair

/*
vf2iz.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04100, vf2iz_t, vf2iz.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to zero integer from float with Scaling Single Triple

/*
vf2iz.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 0 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x04101, vf2iz_q, vf2iz.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to zero integer from float with Scaling Single Quad

/*
vf2iu.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x08000, vf2iu_s, vf2iu.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to smallest integer from float with Scaling Single Word

/*
vf2iu.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x08001, vf2iu_p, vf2iu.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to smallest integer from float with Scaling Single Pair

/*
vf2iu.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x08100, vf2iu_t, vf2iu.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to smallest integer from float with Scaling Single Triple

/*
vf2iu.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x08101, vf2iu_q, vf2iu.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to smallest integer from float with Scaling Single Quad

/*
vf2id.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C000, vf2id_s, vf2id.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to largest integer from float with Scaling Single Word

/*
vf2id.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C001, vf2id_p, vf2id.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to largest integer from float with Scaling Single Pair

/*
vf2id.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C100, vf2id_t, vf2id.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to largest integer from float with Scaling Single Triple

/*
vf2id.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|0 1 1|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C101, vf2id_q, vf2id.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Round to largest integer from float with Scaling Single Quad

/*
vi2f.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |0|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10000, vi2f_s, vi2f.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Convert integer to float with Scaling Single Word

/*
vi2f.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |0|      vs     |1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10001, vi2f_p, vi2f.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Convert integer to float with Scaling Single Pair

/*
vi2f.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10100, vi2f_t, vi2f.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Convert integer to float with Scaling Single Triple

/*
vi2f.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |1 0|1 0 0|   imm5  |1|      vs     |0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x10101, vi2f_q, vi2f.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_power_of_two),
        instr_type: InstrType::R
    ) // Convert integer to float with Scaling Single Quad

    RAB_DEF_OPCODE_REDIRECT(0x14000, r4000allegrex, vfpu4_fmt2_cndmove)
    RAB_DEF_OPCODE_REDIRECT(0x14001, r4000allegrex, vfpu4_fmt2_cndmove)
    RAB_DEF_OPCODE_REDIRECT(0x14100, r4000allegrex, vfpu4_fmt2_cndmove)
    RAB_DEF_OPCODE_REDIRECT(0x14101, r4000allegrex, vfpu4_fmt2_cndmove)

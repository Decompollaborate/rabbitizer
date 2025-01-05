/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-21-----------15--------------7--------------0
    |   VFPU4   |0 0|0 1 1|         |t|             |p|             |
    ------6-------2----3-------------1---------------1---------------
    |-------|
 00 | vcst.s|
 01 | vcst.p|
 10 | vcst.t|
 11 | vcst.q|
 tp |-------|
*/

/*
vcst.s
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 1|   imm5  |0|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0, vcst_s, vcst.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_vconstant),
        instr_type: InstrType::R
    ) // Set ConsTant Single word

/*
vcst.p
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 1|   imm5  |0|0 0 0 0 0 0 0|1|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x1, vcst_p, vcst.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_vconstant),
        instr_type: InstrType::R
    ) // Set ConsTant Pair word

/*
vcst.t
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 1|   imm5  |1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x100, vcst_t, vcst.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_vconstant),
        instr_type: InstrType::R
    ) // Set ConsTant Triple word

/*
vcst.q
    31--------26-25-24--21-20-----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 1|   imm5  |1|0 0 0 0 0 0 0|0|      vd     |
    ------6-------2----3-------5-----1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x101, vcst_q, vcst.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_vconstant),
        instr_type: InstrType::R
    ) // Set ConsTant Quad word

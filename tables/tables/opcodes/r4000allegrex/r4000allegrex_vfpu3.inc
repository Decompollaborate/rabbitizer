/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU3   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vcmp.s| ---   | vmin.s| vmax.s| ---   |vscmp.s| vsge.s| vslt.s|
 01 | vcmp.p| ---   | vmin.p| vmax.p| ---   |vscmp.p| vsge.p| vslt.p|
 10 | vcmp.t| ---   | vmin.t| vmax.t| ---   |vscmp.t| vsge.t| vslt.t|
 11 | vcmp.q| ---   | vmin.q| vmax.q| ---   |vscmp.q| vsge.q| vslt.q|
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vcmp.s
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |0|      vs     |0|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00000, vcmp_s, vcmp.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt),
        instr_type: InstrType::R
    ) // CoMPare Single word

/*
vcmp.p
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |0|      vs     |1|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00001, vcmp_p, vcmp.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt),
        instr_type: InstrType::R
    ) // CoMPare Pair word

/*
vcmp.t
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |1|      vs     |0|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00100, vcmp_t, vcmp.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt),
        instr_type: InstrType::R
    ) // CoMPare Triple word

/*
vcmp.q
    31--------26-25-23-22---------16---14----------8-7-------3------0
    |   VFPU3   |0 0 0|      vt     |1|      vs     |1|0 0 0|  cond |
    ------6--------3---------7-------1-------7-------1----------4----
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x00101, vcmp_q, vcmp.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt),
        instr_type: InstrType::R
    ) // CoMPare Quad word

/*
vmin.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x20000, vmin_s, vmin.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // MINimum Single word

/*
vmin.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x20001, vmin_p, vmin.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // MINimum Pair word

/*
vmin.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x20100, vmin_t, vmin.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // MINimum Triple word

/*
vmin.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x20101, vmin_q, vmin.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // MINimum Quad word

/*
vmax.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x30000, vmax_s, vmax.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // MAXimum Single word

/*
vmax.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x30001, vmax_p, vmax.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // MAXimum Pair word

/*
vmax.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x30100, vmax_t, vmax.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // MAXimum Triple word

/*
vmax.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |0 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x30101, vmax_q, vmax.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // MAXimum Quad word

/*
vscmp.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x50000, vscmp_s, vscmp.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Set ComPare Single word

/*
vscmp.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/


    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x50001, vscmp_p, vscmp.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Set ComPare Pair word

/*
vscmp.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x50100, vscmp_t, vscmp.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Set ComPare Triple word

/*
vscmp.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x50101, vscmp_q, vscmp.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Set ComPare Quad word

/*
vsge.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x60000, vsge_s, vsge.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Greater Equal Single word to value

/*
vsge.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x60001, vsge_p, vsge.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Greater Equal Pair word to value

/*
vsge.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x60100, vsge_t, vsge.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Greater Equal Triple word to value

/*
vsge.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x60101, vsge_q, vsge.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // CGreater Equal Quad word to value

/*
vslt.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70000, vslt_s, vslt.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // Less Than Single word to value

/*
vslt.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70001, vslt_p, vslt.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Less Than Pair word to value

/*
vslt.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70100, vslt_t, vslt.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Less Than Triple word to value

/*
vslt.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU3   |1 1 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x70101, vslt_q, vslt.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Less Than Quad word to value

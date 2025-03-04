/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-23---------------15--------------7--------------0
    |   VFPU0   | fmt |             |t|             |p|             |
    ------6--------3-----------------1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vmul.s| ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | vmul.p| vdot.p| vscl.p| ---   | vhdp.p| ---   | vdet.p| ---   |
 10 | vmul.t| vdot.t| vscl.t| ---   | vhdp.t| vcrs.t| ---   | ---   |
 11 | vmul.q| vdot.q| vscl.q| ---   | vhdp.q| ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vd, vs, vt

/*
vmul.s
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |0|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x00000, vmul_s, vmul.s, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // MULtiply Single word

/*
vmul.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x00001, vmul_p, vmul.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // MULtiply Pair word

/*
vmul.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x00100, vmul_t, vmul.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // MULtiply Triple word

/*
vmul.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x00101, vmul_q, vmul.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // MULtiply Quad word

/*
vdot.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x10001, vdot_p, vdot.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // DOT Product Pair word

/*
vdot.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x10100, vdot_t, vdot.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // DOT Product Triple word

/*
vdot.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 0 1|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x10101, vdot_q, vdot.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // DOT Product Quad word

/*
vscl.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x20001, vscl_p, vscl.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // DOT Product Pair word

/*
vscl.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x20100, vscl_t, vscl.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // DOT Product Triple word

/*
vscl.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |0 1 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x20101, vscl_q, vscl.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_s_vt),
        instr_type: InstrType::R
    ) // SCaLe Quad word

/*
vhdp.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x40001, vhdp_p, vhdp.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // Homogeneous Dot Product Pair word

/*
vhdp.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x40100, vhdp_t, vhdp.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Homogeneous Dot Product Pair word

/*
vhdp.q
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 0|      vt     |1|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x40101, vhdp_q, vhdp.q, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs, Operand::r4000allegrex_q_vt),
        instr_type: InstrType::R
    ) // Homogeneous Dot Product Pair word

/*
vcrs.t
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 0 1|      vt     |1|      vs     |0|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x50100, vcrs_t, vcrs.t, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs, Operand::r4000allegrex_t_vt),
        instr_type: InstrType::R
    ) // Cross Triple Word

/*
vdet.p
    31--------26-25-23-22---------16---14----------8---6------------0
    |   VFPU1   |1 1 0|      vt     |0|      vs     |1|      vd     |
    ------6--------3---------7-------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, 0x60001, vdet_p, vdet.p, R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs, Operand::r4000allegrex_p_vt),
        instr_type: InstrType::R
    ) // 2x2 matrix DETerminant

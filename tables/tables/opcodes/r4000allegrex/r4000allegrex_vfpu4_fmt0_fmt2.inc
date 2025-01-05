/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 0 0 1 0| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | vrcp.s| vrsq.s| vsin.s| vcos.s|vexp2.s|vlog2.s|vsqrt.s|vasin.s|
 01 | vrcp.p| vrsq.p| vsin.p| vcos.p|vexp2.p|vlog2.p|vsqrt.p|vasin.p|
 10 | vrcp.t| vrsq.t| vsin.t| vcos.t|vexp2.t|vlog2.t|vsqrt.t|vasin.t|
 11 | vrcp.q| vrsq.q| vsin.q| vcos.q|vexp2.q|vlog2.q|vsqrt.q|vasin.q|
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vrcp.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0000, vrcp_s, vrcp.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Single word

/*
vrcp.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0001, vrcp_p, vrcp.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Pair word

/*
vrcp.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0100, vrcp_t, vrcp.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Triple word

/*
vrcp.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0101, vrcp_q, vrcp.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Quad word

/*
vrsq.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0200, vrsq_s, vrsq.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Single word

/*
vrsq.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0201, vrsq_p, vrsq.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Pair word

/*
vrsq.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0300, vrsq_t, vrsq.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Triple word

/*
vrsq.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0301, vrsq_q, vrsq.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Quad word


/*
vsin.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/


    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0400, vsin_s, vsin.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // SINe Single word

/*
vsin.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0401, vsin_p, vsin.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // SINe Pair word

/*
vsin.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0500, vsin_t, vsin.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // SINe Triple word

/*
vsin.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0501, vsin_q, vsin.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SINe Quad word

/*
vcos.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0600, vcos_s, vcos.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // COSine Pair word

/*
vcos.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0601, vcos_p, vcos.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // COSine Pair word

/*
vcos.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0700, vcos_t, vcos.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // COSine Quad word

/*
vcos.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0701, vcos_q, vcos.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // COSine Quad word

/*
vexp2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0800, vexp2_s, vexp2.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Single word

/*
vexp2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0801, vexp2_p, vexp2.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Pair word

/*
vexp2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0900, vexp2_t, vexp2.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Triple word

/*
vexp2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0901, vexp2_q, vexp2.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Quad word

/*
vlog2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0A00, vlog2_s, vlog2.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Single word

/*
vlog2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0A01, vlog2_p, vlog2.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Pair word

/*
vlog2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0B00, vlog2_t, vlog2.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Triple word

/*
vlog2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0B01, vlog2_q, vlog2.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Quad word

/*
vsqrt.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C00, vsqrt_s, vsqrt.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Single word

/*
vsqrt.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0C01, vsqrt_p, vsqrt.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Pair word

/*
vsqrt.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0D00, vsqrt_t, vsqrt.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Triple word

/*
vsqrt.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0D01, vsqrt_q, vsqrt.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Quad word

/*
vasin.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0E00, vasin_s, vasin.s, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Arc SINe Single word

/*
vasin.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0E01, vasin_p, vasin.p, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Arc SINe Pair word

/*
vasin.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0F00, vasin_t, vasin.t, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // Arc SINe Triple word

/*
vasin.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x0F01, vasin_q, vasin.q, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Arc SINe Quad word

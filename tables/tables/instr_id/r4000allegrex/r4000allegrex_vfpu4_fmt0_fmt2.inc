/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x0, vrcp_s, vrcp.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Single word

/*
vrcp.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x1, vrcp_p, vrcp.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Pair word

/*
vrcp.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x2, vrcp_t, vrcp.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Triple word

/*
vrcp.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x00 << 2 | 0x3, vrcp_q, vrcp.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // ReCiProcal Quad word

/*
vrsq.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x0, vrsq_s, vrsq.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Single word

/*
vrsq.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x1, vrsq_p, vrsq.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Pair word

/*
vrsq.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x2, vrsq_t, vrsq.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Triple word

/*
vrsq.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x01 << 2 | 0x3, vrsq_q, vrsq.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Reciprocal Square Root value Quad word


/*
vsin.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/


    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x0, vsin_s, vsin.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // SINe Single word

/*
vsin.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x1, vsin_p, vsin.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // SINe Pair word

/*
vsin.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x2, vsin_t, vsin.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // SINe Triple word

/*
vsin.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x02 << 2 | 0x3, vsin_q, vsin.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SINe Quad word

/*
vcos.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x0, vcos_s, vcos.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // COSine Pair word

/*
vcos.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x1, vcos_p, vcos.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // COSine Pair word

/*
vcos.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x2, vcos_t, vcos.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // COSine Quad word

/*
vcos.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|0 1 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x03 << 2 | 0x3, vcos_q, vcos.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // COSine Quad word

/*
vexp2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x0, vexp2_s, vexp2.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Single word

/*
vexp2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x1, vexp2_p, vexp2.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Pair word

/*
vexp2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x2, vexp2_t, vexp2.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Triple word

/*
vexp2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x04 << 2 | 0x3, vexp2_q, vexp2.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // EXPonential base 2 Quad word

/*
vlog2.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x0, vlog2_s, vlog2.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Single word

/*
vlog2.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x1, vlog2_p, vlog2.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Pair word

/*
vlog2.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x2, vlog2_t, vlog2.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Triple word

/*
vlog2.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 0 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x05 << 2 | 0x3, vlog2_q, vlog2.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // LOGarithm base 2 Quad word

/*
vsqrt.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x0, vsqrt_s, vsqrt.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Single word

/*
vsqrt.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x1, vsqrt_p, vsqrt.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Pair word

/*
vsqrt.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x2, vsqrt_t, vsqrt.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Triple word

/*
vsqrt.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 0|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x06 << 2 | 0x3, vsqrt_q, vsqrt.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // SQuare RooT Quad word

/*
vasin.s
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|0|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x0, vasin_s, vasin.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // Arc SINe Single word

/*
vasin.p
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|0|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x1, vasin_p, vasin.p, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R
    ) // Arc SINe Pair word

/*
vasin.t
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|1|      vs     |0|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x2, vasin_t, vasin.t, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R
    ) // Arc SINe Triple word

/*
vasin.q
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 0 0|1 0|1 1 1|1|      vs     |1|      vd     |
    ------6-------2----3----2----3---1-------7-------1-------7-------
*/

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x07 << 2 | 0x3, vasin_q, vasin.q, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R
    ) // Arc SINe Quad word

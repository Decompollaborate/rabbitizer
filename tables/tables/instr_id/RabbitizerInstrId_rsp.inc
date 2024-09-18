/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // RSP instructions

    RABBITIZER_DEF_INSTR_ID(
        rsp, , INVALID, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    #include "rsp/rsp_cop2.inc"
    #include "rsp/rsp_cop2_vu.inc"

    #include "rsp/rsp_normal_lwc2.inc"
    #include "rsp/rsp_normal_swc2.inc"

    // CPU instructions but with rsp registers

    #include "rsp/rsp_normal.inc"
    #include "rsp/rsp_special.inc"
    #include "rsp/rsp_regimm.inc"
    #include "rsp/rsp_cop0.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_00, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_01, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_02, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_03, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_04, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_05, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_06, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_07, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_08, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_09, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_10, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_11, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_12, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_13, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_14, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_15, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_16, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_17, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_18, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_19, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        rsp, , MAX, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )

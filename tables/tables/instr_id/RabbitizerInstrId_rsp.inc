/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // RSP instructions

    RABBITIZER_DEF_INSTR_ID(
        rsp, , INVALID,
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
        rsp, , USERDEF_00,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_01,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_02,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_03,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_04,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_05,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_06,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_07,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_08,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_09,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_10,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_11,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_12,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_13,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_14,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_15,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_16,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_17,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_18,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, , USERDEF_19,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        rsp, , MAX,
        operands: Operand::arr0()
    )

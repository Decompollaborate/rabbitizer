/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , INVALID,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_immediate},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    )

    #include "r4000allegrex/r4000allegrex_normal.inc"

    #include "r4000allegrex/r4000allegrex_special.inc"
    #include "r4000allegrex/r4000allegrex_special_rs.inc"
    #include "r4000allegrex/r4000allegrex_special_sa.inc"

    #include "r4000allegrex/r4000allegrex_regimm.inc"

    #include "r4000allegrex/r4000allegrex_special2.inc"

    #include "r4000allegrex/r4000allegrex_special3.inc"
    #include "r4000allegrex/r4000allegrex_special3_bshfl.inc"

    #include "r4000allegrex/r4000allegrex_cop0.inc"
    #include "r4000allegrex/r4000allegrex_cop0_bc0.inc"
    #include "r4000allegrex/r4000allegrex_cop0_tlb.inc"

    #include "r4000allegrex/r4000allegrex_cop1.inc"
    #include "r4000allegrex/r4000allegrex_cop1_bc1.inc"
    #include "r4000allegrex/r4000allegrex_cop1_fpu_s.inc"
    #include "r4000allegrex/r4000allegrex_cop1_fpu_w.inc"

    #include "r4000allegrex/r4000allegrex_cop2.inc"
    #include "r4000allegrex/r4000allegrex_cop2_bc2.inc"
    #include "r4000allegrex/r4000allegrex_cop2_mfhc2.inc"
    #include "r4000allegrex/r4000allegrex_cop2_mfhc2_p.inc"
    #include "r4000allegrex/r4000allegrex_cop2_mfhc2_p_s.inc"
    #include "r4000allegrex/r4000allegrex_cop2_mthc2.inc"

    #include "r4000allegrex/r4000allegrex_vfpu0.inc"
    #include "r4000allegrex/r4000allegrex_vfpu1.inc"
    #include "r4000allegrex/r4000allegrex_vfpu3.inc"

    #include "r4000allegrex/r4000allegrex_vfpu4.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt0.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt2.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt3.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_rnd.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_cvtflt.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_cvtint.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt8.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_fmt9.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_control.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_color.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt0_cst.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt2.inc"
    #include "r4000allegrex/r4000allegrex_vfpu4_fmt2_cndmove.inc"

    #include "r4000allegrex/r4000allegrex_vfpu5.inc"

    #include "r4000allegrex/r4000allegrex_vfpu6.inc"
    #include "r4000allegrex/r4000allegrex_vfpu6_fmt7.inc"
    #include "r4000allegrex/r4000allegrex_vfpu6_fmt7_fmt0.inc"

    #include "r4000allegrex/r4000allegrex_vfpu7.inc"

    #include "r4000allegrex/r4000allegrex_quadlr.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_00,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_01,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_02,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_03,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_04,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_05,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_06,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_07,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_08,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_09,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_10,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_11,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_12,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_13,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_14,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_15,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_16,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_17,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_18,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , USERDEF_19,
        .operands={0}
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, , MAX,
        .operands={0}
    )

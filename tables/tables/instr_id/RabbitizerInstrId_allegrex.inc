/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        allegrex, , INVALID,
        .operands={RAB_OPERAND_cpu_rs, RAB_OPERAND_cpu_rt, RAB_OPERAND_cpu_immediate},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN
    )

    #include "allegrex/allegrex_normal.inc"

    #include "allegrex/allegrex_special.inc"
    #include "allegrex/allegrex_special_rs.inc"
    #include "allegrex/allegrex_special_sa.inc"

    #include "allegrex/allegrex_regimm.inc"

    #include "allegrex/allegrex_special2.inc"

    #include "allegrex/allegrex_special3.inc"
    #include "allegrex/allegrex_special3_bshfl.inc"

    #include "allegrex/allegrex_cop0.inc"
    #include "allegrex/allegrex_cop1.inc"
    #include "allegrex/allegrex_cop2.inc"
    #include "allegrex/allegrex_cop3.inc"

    #include "allegrex/allegrex_vfpu0.inc"
    #include "allegrex/allegrex_vfpu1.inc"
    #include "allegrex/allegrex_vfpu3.inc"
    #include "allegrex/allegrex_vfpu4.inc"
    #include "allegrex/allegrex_vfpu5.inc"
    #include "allegrex/allegrex_vfpu6.inc"
    #include "allegrex/allegrex_vfpu7.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_00,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_01,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_02,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_03,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_04,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_05,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_06,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_07,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_08,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_09,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_10,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_11,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_12,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_13,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_14,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_15,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_16,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_17,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_18,
        .operands={0}
    )
    RABBITIZER_DEF_INSTR_ID(
        allegrex, , USERDEF_19,
        .operands={0}
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        allegrex, , MAX,
        .operands={0}
    )

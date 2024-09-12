/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        r5900, , INVALID,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN
    )

    #include "r5900/r5900_normal.inc"
    #include "r5900/r5900_special.inc"
    #include "r5900/r5900_regimm.inc"

    #include "r5900/r5900_mmi.inc"
    #include "r5900/r5900_mmi_0.inc"
    #include "r5900/r5900_mmi_1.inc"
    #include "r5900/r5900_mmi_2.inc"
    #include "r5900/r5900_mmi_3.inc"
    #include "r5900/r5900_mmi_pmfhl.inc"
    #include "r5900/r5900_mmi_pmthl.inc"

    #include "r5900/r5900_cop0_tlb.inc"

    #include "r5900/r5900_cop1_fpu_s.inc"

    #include "r5900/r5900_cop2.inc"
    #include "r5900/r5900_cop2_bc2.inc"
    #include "r5900/r5900_cop2_special1.inc"
    #include "r5900/r5900_cop2_special2.inc"
    #include "r5900/r5900_cop2_viwr.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_00,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_01,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_02,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_03,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_04,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_05,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_06,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_07,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_08,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_09,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_10,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_11,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_12,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_13,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_14,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_15,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_16,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_17,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_18,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_19,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        r5900, , MAX,
        operands: Operand::arr0()
    )

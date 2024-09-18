/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        r5900, , INVALID, IsaVersion::EXTENSION, IsaExtension::R5900,
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
        r5900, , USERDEF_00, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_01, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_02, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_03, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_04, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_05, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_06, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_07, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_08, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_09, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_10, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_11, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_12, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_13, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_14, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_15, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_16, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_17, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_18, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r5900, , USERDEF_19, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        r5900, , MAX, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr0()
    )

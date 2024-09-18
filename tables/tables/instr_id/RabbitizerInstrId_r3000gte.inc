/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , INVALID, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN
    )

    #include "r3000gte/r3000gte_cop2_gte.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_00, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_01, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_02, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_03, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_04, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_05, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_06, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_07, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_08, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_09, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_10, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_11, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_12, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_13, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_14, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_15, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_16, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_17, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_18, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_19, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , MAX, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

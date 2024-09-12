/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , INVALID,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN
    )

    #include "r3000gte/r3000gte_cop2_gte.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_00,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_01,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_02,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_03,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_04,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_05,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_06,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_07,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_08,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_09,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_10,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_11,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_12,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_13,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_14,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_15,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_16,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_17,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_18,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , USERDEF_19,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, , MAX,
        operands: Operand::arr0()
    )

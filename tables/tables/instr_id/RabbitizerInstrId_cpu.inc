/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        cpu, , INVALID, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate)
    )

    #include "cpu/cpu_normal.inc"
    #include "cpu/cpu_special.inc"
    #include "cpu/cpu_regimm.inc"

    #include "cpu/cpu_cop0.inc"
    #include "cpu/cpu_cop0_bc0.inc"
    #include "cpu/cpu_cop0_tlb.inc"

    #include "cpu/cpu_cop1.inc"
    #include "cpu/cpu_cop1_bc1.inc"
    #include "cpu/cpu_cop1_fpu_s.inc"
    #include "cpu/cpu_cop1_fpu_d.inc"
    #include "cpu/cpu_cop1_fpu_w.inc"
    #include "cpu/cpu_cop1_fpu_l.inc"

    #include "cpu/cpu_cop2.inc"

#ifndef INSTRID_AVOID_USERDEF
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_00, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_01, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_02, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_03, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_04, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_05, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_06, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_07, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_08, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_09, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_10, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_11, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_12, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_13, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_14, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_15, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_16, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_17, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_18, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, , USERDEF_19, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )
#endif

    RABBITIZER_DEF_INSTR_ID(
        cpu, , MAX, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )

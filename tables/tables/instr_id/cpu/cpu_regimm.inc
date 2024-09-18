/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26----------20-------16------------------------------0
    | = REGIMM  |          |   rt    |                              |
    ------6---------------------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BLTZ  | BGEZ  | BLTZL | BGEZL | ---   | ---   | ---   | ---   |
 01 | TGEI  | TGEIU | TLTI  | TLTIU | TEQI  | ---   | TNEI  | ---   |
 10 | BLTZAL| BGEZAL|BLTZALL|BGEZALL| ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x00, bltz, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Less Than Zero
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x01, bgez, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x02, bltzl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Less Than Zero Likely
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x03, bgezl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero Likely

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x08, tgei, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x09, tgeiu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0A, tlti, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0B, tltiu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0C, teqi, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x0E, tnei, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x10, bltzal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x11, bgezal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x12, bltzall, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link Likely
    RABBITIZER_DEF_INSTR_ID(
        cpu, 0x13, bgezall, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link Likely


    // Pseudo-Instruction Unique IDs

    RABBITIZER_DEF_INSTR_ID(
        cpu, 0xFFFFFF11, bal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true
    ) // Branch and Link

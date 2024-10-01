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
    RAB_DEF_OPCODE(
        core, 0x00, bltz, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Less Than Zero
    RAB_DEF_OPCODE(
        core, 0x01, bgez, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero
    RAB_DEF_OPCODE(
        core, 0x02, bltzl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Less Than Zero Likely
    RAB_DEF_OPCODE(
        core, 0x03, bgezl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero Likely

    RAB_DEF_OPCODE(
        core, 0x08, tgei, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, 0x09, tgeiu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, 0x0A, tlti, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, 0x0B, tltiu, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, 0x0C, teqi, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, 0x0E, tnei, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )

    RAB_DEF_OPCODE(
        core, 0x10, bltzal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link
    RAB_DEF_OPCODE(
        core, 0x11, bgezal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link
    RAB_DEF_OPCODE(
        core, 0x12, bltzall, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link Likely
    RAB_DEF_OPCODE(
        core, 0x13, bgezall, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link Likely


    // Pseudo-Instruction Unique IDs

    RAB_DEF_OPCODE_PSEUDO(
        core, bal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch and Link

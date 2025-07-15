/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
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
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_REGIMM, 0x00, bltz,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Less Than Zero
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_REGIMM, 0x01, bgez,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x02, bltzl, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Less Than Zero Likely
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x03, bgezl, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Greater than or Equal to Zero Likely

    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x08, tgei, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x09, tgeiu, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x0A, tlti, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x0B, tltiu, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x0C, teqi, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x0E, tnei, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        causes_exception: true,
        causes_conditional_exception: true,
        reads_rs: true,
        not_emitted_by_compilers: true
    )

    RAB_DEF_OPCODE_MIPS1(
        core, CORE_REGIMM, 0x10, bltzal,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_REGIMM, 0x11, bgezal,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x12, bltzall, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        does_link: true
    ) // Branch on Less Than Zero and Link Likely
    RAB_DEF_OPCODE(
        core, CORE_REGIMM, 0x13, bgezall, MIPS_II,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link Likely


    // Pseudo-Instruction Unique IDs

    RAB_DEF_OPCODE_MIPS1_PSEUDO(
        core, CORE_REGIMM, 0x11, bal,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch and Link

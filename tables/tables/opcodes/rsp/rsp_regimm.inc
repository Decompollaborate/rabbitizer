/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rs, IMM
    RAB_DEF_OPCODE(
        rsp, 0x00, bltz, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Less Than Zero
    RAB_DEF_OPCODE(
        rsp, 0x01, bgez, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Greater than or Equal to Zero

    RAB_DEF_OPCODE(
        rsp, 0x10, bltzal, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        does_link: true
    ) // Branch on Less Than Zero and Link
    RAB_DEF_OPCODE(
        rsp, 0x11, bgezal, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_rs, Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link


    // Pseudo-Instruction Unique IDs

    RAB_DEF_OPCODE_PSEUDO(
        rsp, bal, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch and Link

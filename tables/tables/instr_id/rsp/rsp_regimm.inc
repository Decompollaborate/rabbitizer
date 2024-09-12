/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, bltz,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Less Than Zero
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x01, bgez,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Greater than or Equal to Zero

    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x10, bltzal,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        does_link: true
    ) // Branch on Less Than Zero and Link
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x11, bgezal,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true
    ) // Branch on Greater Than or Equal to Zero and Link


    // Pseudo-Instruction Unique IDs

    RABBITIZER_DEF_INSTR_ID(
        rsp, -0x11, bal,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true
    ) // Branch and Link

/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP LABEL
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, j,
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true
    ) // Jump
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x03, jal,
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true
    ) // Jump And Link

    // OP rs, rt, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, beq,
        operands: Operand::arr3(Operand::rsp_rs, Operand::rsp_rt, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true
    ) // Branch on EQual
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x05, bne,
        operands: Operand::arr3(Operand::rsp_rs, Operand::rsp_rt, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true
    ) // Branch on Not Equal

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, blez,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Less than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x07, bgtz,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Greater Than Zero

    // OP rt, rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x08, addi,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true
    ) // Add Immediate
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x09, addiu,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Add Immediate Unsigned Word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0A, slti,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0B, sltiu,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate Unsigned
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0C, andi,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // And Immediate
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0D, ori,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Or Immediate
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0E, xori,
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // eXclusive OR Immediate

    // OP rt, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0F, lui,
        operands: Operand::arr2(Operand::rsp_rt, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true
    ) // Load Upper Immediate

    // OP rt, IMM(base)
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x20, lb,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE
    ) // Load Byte
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x21, lh,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT
    ) // Load Halfword
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x23, lw,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD
    ) // Load Word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x24, lbu,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        does_unsigned_memory_access: true
    ) // Load Byte Insigned
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x25, lhu,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        does_unsigned_memory_access: true
    ) // Load Halfword Unsigned
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x28, sb,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE
    ) // Store Byte
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x29, sh,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT
    ) // Store Halfword
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2B, sw,
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD
    ) // Store Word
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x33, pref,
        operands: Operand::arr2(Operand::rsp_hint, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Prefetch


    // Pseudo-Instruction Unique IDs

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, -0x03, b,
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true
    ) // Branch (unconditional)

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        rsp, -0x04, beqz,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on EQual Zero
    RABBITIZER_DEF_INSTR_ID(
        rsp, -0x05, bnez,
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true
    ) // Branch on Not Equal Zero

/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26---------------------------------------------------0
    |  opcode   |                                                   |
    ------6----------------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | *1    | *2    | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
010 | *3    | *4    |  *5   | ---   | BEQL  | BNEL  | BLEZL | BGTZL |
011 | DADDI | DADDIU| LDL   | LDR   | ---   | ---   | ---   | ---   |
100 | LB    | LH    | LWL   | LW    | LBU   | LHU   | LWR   | LWU   |
101 | SB    | SH    | SWL   | SW    | SDL   | SDR   | SWR   | CACHE |
110 | LL    | LWC1  | LWC2  | PREF  | LLD   | LDC1  | LDC2  | LD    |
111 | SC    | SWC1  | SWC2  | ---   | SCD   | SDC1  | SDC2  | SD    |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL, see SPECIAL list
     *2 = REGIMM, see REGIMM list
     *3 = COP0
     *4 = COP1
     *5 = COP2
*/

    // OP LABEL
    RAB_DEF_OPCODE(
        core, 0x02, j, IsaVersion::MIPS_I, None,
        operands: Operand::arr1(Operand::core_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true
    ) // Jump
    RAB_DEF_OPCODE(
        core, 0x03, jal, IsaVersion::MIPS_I, None,
        operands: Operand::arr1(Operand::core_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true
    ) // Jump And Link

    // OP rs, rt, IMM
    RAB_DEF_OPCODE(
        core, 0x04, beq, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on EQual
    RAB_DEF_OPCODE(
        core, 0x05, bne, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on Not Equal
    RAB_DEF_OPCODE(
        core, 0x14, beql, IsaVersion::MIPS_II, None,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on EQual Likely
    RAB_DEF_OPCODE(
        core, 0x15, bnel, IsaVersion::MIPS_II, None,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on Not Equal Likely

    // OP rs, IMM
    RAB_DEF_OPCODE(
        core, 0x06, blez, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true
    ) // Branch on Less than or Equal to Zero
    RAB_DEF_OPCODE(
        core, 0x16, blezl, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Less than or Equal to Zero Likely
    RAB_DEF_OPCODE(
        core, 0x07, bgtz, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true
    ) // Branch on Greater Than Zero
    RAB_DEF_OPCODE(
        core, 0x17, bgtzl, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Greater Than Zero Likely

    // OP rt, rs, IMM
    RAB_DEF_OPCODE(
        core, 0x08, addi, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true
    ) // Add Immediate
    RAB_DEF_OPCODE(
        core, 0x09, addiu, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Add Immediate Unsigned Word
    RAB_DEF_OPCODE(
        core, 0x0A, slti, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate
    RAB_DEF_OPCODE(
        core, 0x0B, sltiu, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate Unsigned
    RAB_DEF_OPCODE(
        core, 0x0C, andi, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // And Immediate
    RAB_DEF_OPCODE(
        core, 0x0D, ori, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_unsigned_lo: true
    ) // Or Immediate
    RAB_DEF_OPCODE(
        core, 0x0E, xori, IsaVersion::MIPS_I, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // eXclusive OR Immediate
    RAB_DEF_OPCODE(
        core, 0x18, daddi, IsaVersion::MIPS_III, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Doubleword add Immediate
    RAB_DEF_OPCODE(
        core, 0x19, daddiu, IsaVersion::MIPS_III, None,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Doubleword add Immediate Unsigned

    // OP rt, IMM
    RAB_DEF_OPCODE(
        core, 0x0F, lui, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true
    ) // Load Upper Immediate

    // OP rt, IMM(base)
    RAB_DEF_OPCODE(
        core, 0x1A, ldl, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEWORD_LEFT)
    ) // Load Doubleword Left
    RAB_DEF_OPCODE(
        core, 0x1B, ldr, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEWORD_RIGHT)
    ) // Load Doubleword Right
    RAB_DEF_OPCODE(
        core, 0x20, lb, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::BYTE)
    ) // Load Byte
    RAB_DEF_OPCODE(
        core, 0x21, lh, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::SHORT)
    ) // Load Halfword
    RAB_DEF_OPCODE(
        core, 0x22, lwl, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD_LEFT)
    ) // Load Word Left
    RAB_DEF_OPCODE(
        core, 0x23, lw, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Load Word
    RAB_DEF_OPCODE(
        core, 0x24, lbu, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::BYTE),
        does_unsigned_memory_access: true
    ) // Load Byte Insigned
    RAB_DEF_OPCODE(
        core, 0x25, lhu, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::SHORT),
        does_unsigned_memory_access: true
    ) // Load Halfword Unsigned
    RAB_DEF_OPCODE(
        core, 0x26, lwr, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD_RIGHT)
    ) // Load Word Right
    RAB_DEF_OPCODE(
        core, 0x27, lwu, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD),
        does_unsigned_memory_access: true
    ) // Load Word Unsigned
    RAB_DEF_OPCODE(
        core, 0x28, sb, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::BYTE)
    ) // Store Byte
    RAB_DEF_OPCODE(
        core, 0x29, sh, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::SHORT)
    ) // Store Halfword
    RAB_DEF_OPCODE(
        core, 0x2A, swl, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::WORD_LEFT)
    ) // Store Word Left
    RAB_DEF_OPCODE(
        core, 0x2B, sw, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::WORD)
    ) // Store Word
    RAB_DEF_OPCODE(
        core, 0x2C, sdl, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEWORD_LEFT)
    ) // Store Doubleword Left
    RAB_DEF_OPCODE(
        core, 0x2D, sdr, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEWORD_RIGHT)
    ) // Store Doubleword Right
    RAB_DEF_OPCODE(
        core, 0x2E, swr, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::WORD_RIGHT)
    ) // Store Word Right
    RAB_DEF_OPCODE(
        core, 0x30, ll, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Load Linked word
    RAB_DEF_OPCODE(
        core, 0x33, pref, IsaVersion::MIPS_IV, None,
        operands: Operand::arr2(Operand::core_hint, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Prefetch
    RAB_DEF_OPCODE(
        core, 0x34, lld, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Load Linked Doubleword
    RAB_DEF_OPCODE(
        core, 0x37, ld, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Load Doubleword
    RAB_DEF_OPCODE(
        core, 0x38, sc, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::WORD)
    ) // Store Conditional word
    RAB_DEF_OPCODE(
        core, 0x3C, scd, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Store Conditional Doubleword
    RAB_DEF_OPCODE(
        core, 0x3F, sd, IsaVersion::MIPS_III, None,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Store Doubleword

    // OP op, IMM(base)
    RAB_DEF_OPCODE(
        core, 0x2F, cache, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_op, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        not_emitted_by_compilers: true
    ) // Cache

    // OP ft, IMM(base)
    RAB_DEF_OPCODE(
        core, 0x31, lwc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::FLOAT)
    ) // Load Word to Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x35, ldc1, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEFLOAT)
    ) // Load Doubleword to Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x39, swc1, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::FLOAT)
    ) // Store Word from Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x3D, sdc1, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEFLOAT)
    ) // Store Doubleword from Coprocessor z

    // OP cop2t, IMM(base)
    RAB_DEF_OPCODE(
        core, 0x32, lwc2, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::WORD)
    ) // Load Word to Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x36, ldc2, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Load Doubleword to Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x3A, swc2, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::WORD)
    ) // Store Word from Coprocessor z
    RAB_DEF_OPCODE(
        core, 0x3E, sdc2, IsaVersion::MIPS_II, None,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::DOUBLEWORD)
    ) // Store Doubleword from Coprocessor z


    // Pseudo-Instruction Unique IDs

    // OP IMM
    RAB_DEF_OPCODE_PSEUDO(
        core, b, IsaVersion::MIPS_I, None,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true
    ) // Branch (unconditional)

    // OP rs, IMM
    RAB_DEF_OPCODE_PSEUDO(
        core, beqz, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true
    ) // Branch on EQual Zero
    RAB_DEF_OPCODE_PSEUDO(
        core, bnez, IsaVersion::MIPS_I, None,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true
    ) // Branch on Not Equal Zero

    RAB_DEF_OPCODE_REDIRECT(0x00, none, special)
    RAB_DEF_OPCODE_REDIRECT(0x01, none, regimm)
    RAB_DEF_OPCODE_REDIRECT(0x10, none, coprocessor0)
    RAB_DEF_OPCODE_REDIRECT(0x11, none, coprocessor1)
    RAB_DEF_OPCODE_REDIRECT(0x12, none, coprocessor2)

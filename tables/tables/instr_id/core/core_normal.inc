/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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
     *1 = SPECIAL, see SPECIAL list    *2 = REGIMM, see REGIMM list
     *3 = COP0                         *4 = COP1
     *5 = COP2
*/

    // OP LABEL
    RABBITIZER_DEF_INSTR_ID(
        core, 0x02, j, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true
    ) // Jump
    RABBITIZER_DEF_INSTR_ID(
        core, 0x03, jal, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true
    ) // Jump And Link

    // OP rs, rt, IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0x04, beq, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on EQual
    RABBITIZER_DEF_INSTR_ID(
        core, 0x05, bne, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on Not Equal
    RABBITIZER_DEF_INSTR_ID(
        core, 0x14, beql, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on EQual Likely
    RABBITIZER_DEF_INSTR_ID(
        core, 0x15, bnel, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true
    ) // Branch on Not Equal Likely

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0x06, blez, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true
    ) // Branch on Less than or Equal to Zero
    RABBITIZER_DEF_INSTR_ID(
        core, 0x16, blezl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Less than or Equal to Zero Likely
    RABBITIZER_DEF_INSTR_ID(
        core, 0x07, bgtz, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true
    ) // Branch on Greater Than Zero
    RABBITIZER_DEF_INSTR_ID(
        core, 0x17, bgtzl, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true
    ) // Branch on Greater Than Zero Likely

    // OP rt, rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0x08, addi, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true
    ) // Add Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x09, addiu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Add Immediate Unsigned Word
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0A, slti, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0B, sltiu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true
    ) // Set on Less Than Immediate Unsigned
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0C, andi, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // And Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0D, ori, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Or Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0E, xori, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true
    ) // eXclusive OR Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x18, daddi, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Doubleword add Immediate
    RABBITIZER_DEF_INSTR_ID(
        core, 0x19, daddiu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true
    ) // Doubleword add Immediate Unsigned

    // OP rt, IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0x0F, lui, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true
    ) // Load Upper Immediate

    // OP rt, IMM(base)
    RABBITIZER_DEF_INSTR_ID(
        core, 0x1A, ldl, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Doubleword Left
    RABBITIZER_DEF_INSTR_ID(
        core, 0x1B, ldr, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Doubleword Right
    RABBITIZER_DEF_INSTR_ID(
        core, 0x20, lb, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE
    ) // Load Byte
    RABBITIZER_DEF_INSTR_ID(
        core, 0x21, lh, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT
    ) // Load Halfword
    RABBITIZER_DEF_INSTR_ID(
        core, 0x22, lwl, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Word Left
    RABBITIZER_DEF_INSTR_ID(
        core, 0x23, lw, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD
    ) // Load Word
    RABBITIZER_DEF_INSTR_ID(
        core, 0x24, lbu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
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
        core, 0x25, lhu, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
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
        core, 0x26, lwr, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Word Right
    RABBITIZER_DEF_INSTR_ID(
        core, 0x27, lwu, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        does_unsigned_memory_access: true
    ) // Load Word Unsigned
    RABBITIZER_DEF_INSTR_ID(
        core, 0x28, sb, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE
    ) // Store Byte
    RABBITIZER_DEF_INSTR_ID(
        core, 0x29, sh, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT
    ) // Store Halfword
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2A, swl, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Word Left
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2B, sw, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD
    ) // Store Word
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2C, sdl, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Doubleword Left
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2D, sdr, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Doubleword Right
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2E, swr, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Word Right
    RABBITIZER_DEF_INSTR_ID(
        core, 0x30, ll, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Linked word
    RABBITIZER_DEF_INSTR_ID(
        core, 0x33, pref, IsaVersion::MIPS_IV, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_hint, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Prefetch
    RABBITIZER_DEF_INSTR_ID(
        core, 0x34, lld, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Linked Doubleword
    RABBITIZER_DEF_INSTR_ID(
        core, 0x37, ld, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEWORD
    ) // Load Doubleword
    RABBITIZER_DEF_INSTR_ID(
        core, 0x38, sc, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Conditional word
    RABBITIZER_DEF_INSTR_ID(
        core, 0x3C, scd, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Conditional Doubleword
    RABBITIZER_DEF_INSTR_ID(
        core, 0x3F, sd, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEWORD
    ) // Store Doubleword

    // OP op, IMM(base)
    RABBITIZER_DEF_INSTR_ID(
        core, 0x2F, cache, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_op, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        not_emitted_by_compilers: true
    ) // Cache

    // OP ft, IMM(base)
    RABBITIZER_DEF_INSTR_ID(
        core, 0x31, lwc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::FLOAT
    ) // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x35, ldc1, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEFLOAT
    ) // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x39, swc1, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::FLOAT
    ) // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x3D, sdc1, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEFLOAT
    ) // Store Doubleword from Coprocessor z

    // OP cop2t, IMM(base)
    RABBITIZER_DEF_INSTR_ID(
        core, 0x32, lwc2, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Word to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x36, ldc2, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Doubleword to Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x3A, swc2, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Word from Coprocessor z
    RABBITIZER_DEF_INSTR_ID(
        core, 0x3E, sdc2, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Doubleword from Coprocessor z


    // Pseudo-Instruction Unique IDs

    // OP IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0xFFFFFF03, b, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_pseudo: true
    ) // Branch (unconditional)

    // OP rs, IMM
    RABBITIZER_DEF_INSTR_ID(
        core, 0xFFFFFF04, beqz, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true
    ) // Branch on EQual Zero
    RABBITIZER_DEF_INSTR_ID(
        core, 0xFFFFFF05, bnez, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true
    ) // Branch on Not Equal Zero

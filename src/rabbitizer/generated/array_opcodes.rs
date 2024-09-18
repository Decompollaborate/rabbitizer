/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{
    opcode, AccessType, InstrSuffix, InstrType, IsaExtension, IsaVersion, Opcode, OpcodeDescriptor,
    Operand,
};
pub static OPCODES: [OpcodeDescriptor; opcode::OPCODE_COUNT] = {
    let mut table = [OpcodeDescriptor::default(); opcode::OPCODE_COUNT];
    table[Opcode::cpu_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        ..OpcodeDescriptor::new("INVALID", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_j as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        ..OpcodeDescriptor::new("j", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_jal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true,
        ..OpcodeDescriptor::new("jal", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_beq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("beq", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bne", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_beql as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("beql", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bnel as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bnel", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_blez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("blez", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_blezl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("blezl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgtz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgtz", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgtzl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgtzl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_addi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addi", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_addiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addiu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_slti as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("slti", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("sltiu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_andi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("andi", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("ori", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_xori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("xori", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_daddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("daddi", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_daddiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("daddiu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lui as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true,
        ..OpcodeDescriptor::new("lui", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ldl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldl", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ldr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldr", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("lb", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("lh", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lwl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwl", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("lw", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lbu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lbu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lhu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lhu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lwr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lwu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lwu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("sb", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("sh", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_swl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swl", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("sw", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sdl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdl", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sdr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdr", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_swr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ll as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ll", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_pref as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_hint, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("pref", IsaVersion::MIPS_IV, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lld as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lld", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ld as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEWORD,
        ..OpcodeDescriptor::new("ld", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sc", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_scd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("scd", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEWORD,
        ..OpcodeDescriptor::new("sd", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cache as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_op, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("cache", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lwc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::FLOAT,
        ..OpcodeDescriptor::new("lwc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ldc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEFLOAT,
        ..OpcodeDescriptor::new("ldc1", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_swc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::FLOAT,
        ..OpcodeDescriptor::new("swc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sdc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEFLOAT,
        ..OpcodeDescriptor::new("sdc1", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_lwc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ldc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldc2", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_swc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sdc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdc2", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_b as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("b", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_beqz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("beqz", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bnez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bnez", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sll", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sra", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsll", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrl", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsra", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsll32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsll32", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrl32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrl32", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsra32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsra32", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsllv", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrlv", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrav", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sllv", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_srav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srav", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mthi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_hi: true,
        ..OpcodeDescriptor::new("mthi", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mtlo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("mtlo", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_jr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true,
        ..OpcodeDescriptor::new("jr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_jalr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("jalr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mfhi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_hi: true,
        ..OpcodeDescriptor::new("mfhi", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mflo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_lo: true,
        ..OpcodeDescriptor::new("mflo", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_movz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movz", IsaVersion::MIPS_IV, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_movn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movn", IsaVersion::MIPS_IV, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_div as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("div", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_divu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("divu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sn64_div as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("sn64_div", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sn64_divu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("sn64_divu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ddiv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("ddiv", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ddivu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("ddivu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_add as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("add", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_addu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("addu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sub", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_subu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("subu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_and as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("and", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_or as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        maybe_is_move: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("or", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_xor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("xor", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_nor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("nor", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_slt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("slt", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sltu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dadd", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_daddu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("daddu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsub", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dsubu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsubu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_syscall as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code_lower),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("syscall", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_break as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("break", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sync as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sync", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mult as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("mult", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_multu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("multu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmult as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("dmult", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmultu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("dmultu", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tge as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tge", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tgeu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgeu", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlt", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tltu", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_teq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("teq", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tne", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_nop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true,
        ..OpcodeDescriptor::new("nop", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_move as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("move", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_not as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("not", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_neg as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("neg", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_negu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("negu", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bltz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bltz", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgez", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bltzl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgezl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tgei as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgei", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tgeiu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgeiu", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlti as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlti", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tltiu", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_teqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("teqi", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tnei as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tnei", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzal", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezal", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzall as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzall", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezall as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezall", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bal", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mfc0", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("dmfc0", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("cfc0", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mtc0", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("dmtc0", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("ctc0", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc0f", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc0t", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc0fl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc0tl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbr as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbwi as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbwi", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbwr as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("tlbwr", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbp as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbp", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_rfe as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("rfe", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_eret as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("eret", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mfc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("dmfc1", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true,
        ..OpcodeDescriptor::new("mtc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_dmtc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true,
        ..OpcodeDescriptor::new("dmtc1", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc1", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc1f", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc1t", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc1fl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc1tl", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_add_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("add_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("sub_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mul_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mul_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_div_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("div_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("sqrt_s", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_abs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("abs_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mov_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mov_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_neg_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("neg_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_round_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_l_s", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_l_s", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_l_s", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_l_s", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_round_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_w_s", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_w_s", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_w_s", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_w_s", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_w_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_l_s", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_f_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_un_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_un_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_eq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_eq_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ueq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ueq_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_olt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_olt_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ult_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ult_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ole_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ole_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ule_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ule_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_sf_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_sf_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngle_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngle_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_seq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_seq_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngl_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngl_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_lt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_nge_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_nge_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_le_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngt_s", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_add_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("add_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sub_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("sub_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mul_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mul_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_div_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("div_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_sqrt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("sqrt_d", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_abs_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("abs_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mov_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mov_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_neg_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("neg_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_round_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_l_d", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_l_d", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_l_d", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_l_d", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_round_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_w_d", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_w_d", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_w_d", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_w_d", IsaVersion::MIPS_II, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_w_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_l_d", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_f_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_f_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_un_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_un_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_eq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_eq_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ueq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ueq_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_olt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_olt_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ult_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ult_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ole_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ole_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ule_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ule_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_df_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_df_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngle_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngle_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_seq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_seq_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngl_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngl_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_lt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_nge_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_nge_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_le_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngt_d", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_w", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_w", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_l as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_l", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_l as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_l", IsaVersion::MIPS_III, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("mtc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::cpu_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX", IsaVersion::MIPS_I, IsaExtension::NONE)
    }
    .check_panic_chain();
    table[Opcode::rsp_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("INVALID", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_mfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index),
        ..OpcodeDescriptor::new("mfc2", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_mtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index),
        ..OpcodeDescriptor::new("mtc2", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::rsp_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::rsp_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulf as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulf", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrndp as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vrndp", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulq", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudl", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudm as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudm", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudn", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudh", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacf as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacf", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrndn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vrndn", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacq", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadl", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadm as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadm", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadn", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadh", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vadd", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsub", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vabs as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vabs", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vaddc as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vaddc", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vsubc as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsubc", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vsar as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsar", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vand", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vnand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnand", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vnor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vxor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vnxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnxor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vlt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vlt", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_veq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("veq", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vne", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vge as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vge", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vcl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vcl", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vch as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vch", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vcr as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vcr", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmrg as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmrg", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcp as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcp", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcpl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcpl", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcph as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcph", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vmov as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vmov", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsq", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsql as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsql", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsqh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsqh", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vnop", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lbv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lbv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lsv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lsv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_llv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("llv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_ldv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ldv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lqv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lqv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lrv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lrv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lpv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lpv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_luv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("luv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lhv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lhv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lfv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_ltv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ltv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sbv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sbv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_ssv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ssv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_slv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("slv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sdv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sdv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sqv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sqv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_srv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("srv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_spv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("spv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_suv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("suv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_shv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("shv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sfv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_stv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("stv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_swv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("swv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_j as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        ..OpcodeDescriptor::new("j", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_jal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true,
        ..OpcodeDescriptor::new("jal", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_beq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_rs,
            Operand::rsp_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true,
        ..OpcodeDescriptor::new("beq", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_rs,
            Operand::rsp_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bne", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_blez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("blez", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bgtz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bgtz", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_addi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addi", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_addiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addiu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_slti as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("slti", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("sltiu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_andi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("andi", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_ori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("ori", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_xori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("xori", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lui as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true,
        ..OpcodeDescriptor::new("lui", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("lb", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("lh", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("lw", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lbu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lbu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_lhu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lhu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("sb", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("sh", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("sw", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_pref as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_hint, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("pref", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_b as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        ..OpcodeDescriptor::new("b", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_beqz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("beqz", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bnez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bnez", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sll", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sra", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sllv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_srav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srav", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_jr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::rsp_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true,
        ..OpcodeDescriptor::new("jr", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_jalr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::rsp_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("jalr", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_movz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movz", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_movn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movn", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_add as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("add", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_addu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("addu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sub", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_subu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("subu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_and as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("and", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_or as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("or", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_xor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("xor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_nor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("nor", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_slt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("slt", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_sltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sltu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_break as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("break", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_nop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true,
        ..OpcodeDescriptor::new("nop", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_move as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("move", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_not as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("not", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_neg as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("neg", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_negu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("negu", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bltz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bltz", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bgez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bgez", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bltzal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzal", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bgezal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezal", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_bal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bal", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_mfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mfc0", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_mtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mtc0", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::rsp_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX", IsaVersion::EXTENSION, IsaExtension::RSP)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("INVALID", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_RTPS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("RTPS", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_RTPT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("RTPT", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCL as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCL", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCS", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCT", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_INTPL as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("INTPL", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCS", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCT", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCDS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCDS", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCDT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCDT", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCCS", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCCT", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_CDP as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("CDP", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_CC as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("CC", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCLIP as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCLIP", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_AVSZ3 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("AVSZ3", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_AVSZ4 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("AVSZ4", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_MVMVA as usize] = OpcodeDescriptor {
        operands: Operand::arr5(
            Operand::r3000gte_sf,
            Operand::r3000gte_mx,
            Operand::r3000gte_v,
            Operand::r3000gte_cv,
            Operand::r3000gte_lm,
        ),
        ..OpcodeDescriptor::new("MVMVA", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_SQR as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("SQR", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_OP as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("OP", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_GPF as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("GPF", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_GPL as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("GPL", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r3000gte_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX", IsaVersion::EXTENSION, IsaExtension::R3000GTE)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new(
            "INVALID",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_lv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("lv_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        modifies_rs: true,
        ..OpcodeDescriptor::new("sv_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_lv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("lv_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base_maybe_wb,
        ),
        instr_type: InstrType::I,
        modifies_rs: true,
        ..OpcodeDescriptor::new("sv_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_clz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("clz", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_clo as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("clo", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_madd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("madd", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_maddu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("maddu", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_msub as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("msub", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_msubu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("msubu", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_max as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("max", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_min as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("min", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_rotr as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("rotr", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_rotrv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("rotrv", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sleep as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sleep", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfie as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfie", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtie as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtie", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_ext as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::cpu_rt,
            Operand::cpu_rs,
            Operand::r4000allegrex_pos,
            Operand::r4000allegrex_size,
        ),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("ext", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_ins as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::cpu_rt,
            Operand::cpu_rs,
            Operand::r4000allegrex_pos,
            Operand::r4000allegrex_size_plus_pos,
        ),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("ins", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_wsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("wsbh", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_wsbw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("wsbw", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_seb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("seb", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_seh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("seh", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bitrev as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bitrev", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvf as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bvf", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvt as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bvt", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvfl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bvfl", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvtl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bvtl", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfv", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_cop2cd),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfvc", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsync2 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsync2", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtv", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_cop2cd),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtvc", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsbn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsbn_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcrs_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcrs_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdet_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdet_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vscmp_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vscmp_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vscmp_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vscmp_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vwbn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_bn,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vwbn_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vidt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vidt_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vidt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vidt_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat0_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat0_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat0_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat0_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat1_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat1_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat1_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsat1_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vzero_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vzero_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vzero_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vzero_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vexp2_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vexp2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vexp2_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vexp2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vlog2_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vlog2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vlog2_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vlog2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsqrt_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsqrt_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsqrt_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsqrt_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vasin_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vasin_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vasin_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vasin_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnrcp_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnrcp_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnrcp_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnrcp_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnsin_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnsin_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnsin_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vnsin_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrexp2_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrexp2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrexp2_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrexp2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrnds_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrnds_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndi_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndi_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndi_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndi_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf1_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf1_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf1_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf1_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf2_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf2_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vrndf2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2h_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2h_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2h_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2h_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vh2f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vh2f_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vh2f_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vh2f_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsbz_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsbz_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlgb_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlgb_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vuc2ifs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vuc2ifs_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vc2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vc2i_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vus2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vus2i_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vus2i_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vus2i_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vs2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vs2i_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vs2i_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vs2i_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2uc_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vi2uc_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2c_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2c_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2us_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vi2us_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2us_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vi2us_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2s_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2s_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2s_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2s_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsrt1_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsrt2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vbfy1_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vbfy1_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vbfy2_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsocp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsocp_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsocp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsocp_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt3_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsrt3_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vsrt4_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmfvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_cop2cs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmfvc", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmtvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_cop2cd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmtvc", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt4444_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vt4444_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt5551_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vt5551_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt5650_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vt5650_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2in_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2in_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2in_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2in_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iz_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iz_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iz_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iz_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iu_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iu_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iu_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2iu_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2id_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2id_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2id_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vf2id_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovt_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovt_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovt_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovt_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovf_s",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovf_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovf_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcmovf_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxs as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_rpx,
            Operand::r4000allegrex_rpy,
            Operand::r4000allegrex_rpz,
            Operand::r4000allegrex_rpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxs", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxt as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_rpx,
            Operand::r4000allegrex_rpy,
            Operand::r4000allegrex_rpz,
            Operand::r4000allegrex_rpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxt", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxd as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_wpx,
            Operand::r4000allegrex_wpy,
            Operand::r4000allegrex_wpz,
            Operand::r4000allegrex_wpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxd", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_viim_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_int16),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("viim_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfim_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_float16),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfim_s", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mp_vd,
            Operand::r4000allegrex_mp_vs_transpose,
            Operand::r4000allegrex_mp_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmul_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mt_vd,
            Operand::r4000allegrex_mt_vs_transpose,
            Operand::r4000allegrex_mt_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmul_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mq_vd,
            Operand::r4000allegrex_mq_vs_transpose,
            Operand::r4000allegrex_mq_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmul_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vhtfm2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vtfm2_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm3_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vhtfm3_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm3_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vtfm3_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vhtfm4_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vtfm4_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mp_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmscl_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mt_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmscl_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mq_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmscl_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcrsp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vcrsp_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vqmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vqmul_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_p_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_p", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_t_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_t", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_q_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmov_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmov_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmmov_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmidt_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmidt_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmidt_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmzero_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmzero_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmzero_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmone_p",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmone_t",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new(
            "vmone_q",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vnop", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsync as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vsync", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vflush as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vflush", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_svl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("svl_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_svr_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("svr_q", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_00",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_01",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_02",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_03",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_04",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_05",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_06",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_07",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_08",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_09",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_10",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_11",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_12",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_13",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_14",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_15",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_16",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_17",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_18",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new(
            "USERDEF_19",
            IsaVersion::EXTENSION,
            IsaExtension::R4000ALLEGREX,
        )
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX", IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX)
    }
    .check_panic_chain();
    table[Opcode::r5900_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("INVALID", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_lq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_sq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_lqc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vft, Operand::cpu_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lqc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_sqc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vft, Operand::cpu_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sqc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_sync_p as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sync_p", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mult as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("mult", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mfsa as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("mfsa", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsa as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsa", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsab as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsab", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsah as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsah", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_madd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("madd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_maddu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("maddu", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_plzcw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("plzcw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mfhi1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("mfhi1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mthi1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mthi1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mflo1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        reads_rd: true,
        ..OpcodeDescriptor::new("mflo1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mtlo1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mtlo1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mult1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("mult1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_multu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("multu1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_div1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("div1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_divu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("divu1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_madd1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("madd1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_maddu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("maddu1", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psllh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psrah as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrah", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psllw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psraw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psraw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgtw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgtw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaxw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaxw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgth", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaxh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaxh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgtb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgtb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ppacw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppacw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ppach as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppach", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ppacb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppacb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pext5 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pext5", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ppac5 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppac5", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pabsw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pabsw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pminw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pminw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_padsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padsbh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pabsh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pabsh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pminh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pminh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqb", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_padduw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padduw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubuw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextuw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_padduh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padduh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubuh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubuh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextuh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextuh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_paddub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("paddub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psubub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pextub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_qfsrv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("qfsrv", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaddw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psllvw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllvw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlvw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlvw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmsubw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmflo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmflo", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pinth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pinth", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmultw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmultw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyld as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyld", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaddh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaddh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_phmadh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("phmadh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pand", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pxor", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmsubh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmsubh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_phmsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("phmsbh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pexeh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexeh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_prevh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("prevh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmulth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmulth", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivbw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivbw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pexew as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexew", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_prot3w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("prot3w", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmadduw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmadduw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_psravw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psravw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmthi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmthi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmtlo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmtlo", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pinteh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pinteh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmultuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmultuw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivuw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivuw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyud as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyud", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_por as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("por", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pnor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pnor", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pexch as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexch", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pexcw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexcw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_lw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_uw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_uw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_slw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_slw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_lh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_sh", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_pmthl_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmthl_lw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ei as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("ei", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_di as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("di", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_c1__sqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_copraw),
        ..OpcodeDescriptor::new("c1__sqrt_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_rsqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("rsqrt_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_adda_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("adda_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_suba_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("suba_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_mula_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mula_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_madd_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("madd_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_msub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("msub_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_madda_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("madda_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_msuba_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("msuba_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_max_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("max_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_min_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("min_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_c_lt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_c_le_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_s", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_qmfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vfs),
        modifies_rt: true,
        ..OpcodeDescriptor::new("qmfc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vis),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_qmtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vfs),
        reads_rt: true,
        ..OpcodeDescriptor::new("qmtc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vis),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        ..OpcodeDescriptor::new("bc2f", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        ..OpcodeDescriptor::new("bc2t", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc2fl", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc2tl", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddy", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsuby as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsuby", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddy", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsuby as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsuby", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxy", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vminix as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminix", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiy", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulx", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmuly as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmuly", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmuli as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmuli", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vminii as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminii", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmadd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmul as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmul", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vopmsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vopmsub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmini as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmini", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("viadd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_visub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("visub", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vit,
            Operand::r5900_vis,
            Operand::r5900_immediate5,
        ),
        ..OpcodeDescriptor::new("viaddi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("viand", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vior as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("vior", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vcallms as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r5900_immediate15),
        ..OpcodeDescriptor::new("vcallms", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vcallmsr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r5900_vis),
        ..OpcodeDescriptor::new("vcallmsr", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vadday as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadday", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubay", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadday as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmadday", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddaz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddaw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubay", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof0", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof4 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof4", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof12 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof12", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof15 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof15", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi0", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi4 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi4", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi12 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi12", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi15 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi15", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulax", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulay", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaz", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vabs as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vabs", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulai", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vclipw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vclipw", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddaq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddai", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddai", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubai", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubai", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vadda as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vftxyzw,
            Operand::r5900_vfsxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadda", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadda as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmadda", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmula as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmula", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsuba as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsuba", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsuba as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vftxyzw,
            Operand::r5900_vfsxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsuba", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vopmula as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vopmula", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vnop", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmove as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmove", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmr32 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmr32", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vlqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vlqi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsqi", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vlqd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vlqd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsqd", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vdiv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vdiv", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqrt as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_Q, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vsqrt", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vrsqrt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vrsqrt", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vwaitq as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vwaitq", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmtir as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vmtir", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vmfir as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmfir", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vrnext as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vrnext", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vrget as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vrget", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vrinit as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vrinit", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vrxor as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vrxor", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_w", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_z as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_z", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_y as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_y", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_x as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_x", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_w", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_z as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_z", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_y as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_y", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_x as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_x", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    table[Opcode::r5900_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX", IsaVersion::EXTENSION, IsaExtension::R5900)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < opcode::OPCODE_COUNT {
        table[i].check_panic();
        i += 1;
    }
    table
};
